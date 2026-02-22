use crate::{
    core::{
        entity_logic::{Entity, EntityId, Movable},
        game::{GameRules, GameState},
        game_items::GameItemId,
    },
    util::{
        errors_results::{DataError, EngineError, FailReason, GameError, GameOutcome, GameResult},
        text_log::LogData,
    },
    world::{
        coordinate_system::{Direction, Point, PointVector},
        tiles::{Collision, DoorType, Interactable, TileType},
    },
};

pub enum PlayerInput {
    Wait,
    Direction(Direction),
    #[allow(dead_code)]
    UseItem(u32),
    DropItem(GameItemId),
    UnequipWeapon,
    UnequipArmor,
    RangedAttack(EntityId),
}

pub enum ActionKind {
    Wait,
    Move(Direction),
    Attack(EntityId),
    PickUpItem(EntityId),
    UseItem(u32),
    DropItem(GameItemId),
    UnequipWeapon,
    UnequipArmor,
    TileInteraction(Point),
    RangedAttack(EntityId),
}

impl GameState {
    pub fn resolve_player_action(&mut self, input: PlayerInput) {
        if let Some(intended_action) = self.interpret_player_input(input) {
            let action_result: GameResult = match intended_action {
                ActionKind::Wait => Ok(GameOutcome::Success),
                ActionKind::Move(Direction::Up) => self.move_player_character(0, -1),
                ActionKind::Move(Direction::Right) => self.move_player_character(1, 0),
                ActionKind::Move(Direction::Down) => self.move_player_character(0, 1),
                ActionKind::Move(Direction::Left) => self.move_player_character(-1, 0),
                ActionKind::Attack(npc_id) => self.player_attack_npc(npc_id),
                ActionKind::PickUpItem(entity_id) => self.pick_up_item(entity_id),
                ActionKind::DropItem(item_id) => self.drop_item(item_id),
                ActionKind::UseItem(item_id) => self.use_item(item_id),
                ActionKind::UnequipWeapon => self.unequip_weapon(),
                ActionKind::UnequipArmor => self.unequip_armor(),
                ActionKind::TileInteraction(point) => self.tile_interaction(point),
                ActionKind::RangedAttack(npc_id) => self.player_ranged_attack_npc(npc_id),
            };

            match action_result {
                Ok(GameOutcome::Success) => self.next_round(),
                Ok(GameOutcome::Fail(reason)) => {
                    // Log for user only if message is defined for user
                    if let Some(log_data) = reason.notify_user() {
                        self.log.info(log_data);
                    }
                }
                Err(error) => {
                    match error {
                        // Game cannot be played without world files. Should crash.
                        GameError::Data(DataError::StaticWorldNotFound(index)) => {
                            panic!(
                                "Fatal game error: Static level {} could not be loaded. {}",
                                index, error
                            );
                        }
                        // Game cannot be played with corrupt world files or corrupt procedural world. Should crash.
                        GameError::Data(DataError::InvalidWorldFormat(index)) => {
                            panic!(
                                "Fatal game error: Data of level {} could not be applied. Data corrupt. {}",
                                index, error
                            );
                        }
                        error => {
                            // Log for Debugging
                            self.log.debug_warn(error.to_string());
                        }
                    }
                }
            }
        }
    }

    pub fn interpret_player_input(&mut self, input: PlayerInput) -> Option<ActionKind> {
        match input {
            PlayerInput::Wait => Some(ActionKind::Wait),
            PlayerInput::Direction(direction) => {
                let target_point: Point = self.player.character.pos().get_adjacent(direction);

                // Fallback if point is out of bounds
                if !self
                    .current_world()
                    .is_in_bounds(target_point.x as isize, target_point.y as isize)
                {
                    return None;
                }

                if let Some(entity_id) = self.current_level().get_npc_at(target_point) {
                    return Some(ActionKind::Attack(entity_id));
                }

                if let Some(entity_id) = self.current_level().get_item_sprite_at(target_point) {
                    return Some(ActionKind::PickUpItem(entity_id));
                }

                let target_tile = self.current_world().get_tile(target_point);
                if target_tile.tile_type.is_interactable() {
                    return Some(ActionKind::TileInteraction(target_point));
                }

                Some(ActionKind::Move(direction))
            }
            PlayerInput::UseItem(item_id) => Some(ActionKind::UseItem(item_id)),
            PlayerInput::DropItem(item_id) => Some(ActionKind::DropItem(item_id)),
            PlayerInput::UnequipWeapon => Some(ActionKind::UnequipWeapon),
            PlayerInput::UnequipArmor => Some(ActionKind::UnequipArmor),
            PlayerInput::RangedAttack(entity_id) => Some(ActionKind::RangedAttack(entity_id)),
        }
    }

    fn pick_up_item(&mut self, entity_id: EntityId) -> GameResult {
        let item_sprite = self
            .current_level()
            .get_item_sprite(entity_id)
            .ok_or(EngineError::ItemSpriteNotFound(entity_id))?;

        let item = self
            .get_item_by_id(item_sprite.item_id)
            .ok_or(EngineError::UnregisteredItem(item_sprite.item_id))?;
        let item_def = self
            .get_item_def_by_id(&item.def_id)
            .ok_or(DataError::MissingItemDefinition(item.def_id))?;

        let result = self.add_item_to_inv(item_sprite.item_id);

        if let Ok(GameOutcome::Success) = result {
            self.current_level_mut().despawn(entity_id);
            self.log.info(LogData::ItemPickUp { item_name: item_def.name.to_string() })
        }

        result
    }

    fn drop_item(&mut self, item_id: GameItemId) -> GameResult {
        self.remove_item_from_inv(item_id)?;

        let player_pos = self.player.character.pos();
        let item_sprite = self.create_item_sprite(item_id, player_pos)?;
        self.current_level_mut().spawn_item_sprite(item_sprite)?;

        Ok(GameOutcome::Success)
    }

    fn move_player_character(&mut self, dx: isize, dy: isize) -> GameResult {
        let point_vector = PointVector::new(dx, dy);
        let new_pos = self.player.character.pos() + point_vector;

        if !self.current_world().is_in_bounds(new_pos.x as isize, new_pos.y as isize) {
            return Ok(GameOutcome::Fail(FailReason::PointOutOfBounds(new_pos)));
        }

        if !self.current_world().get_tile(new_pos).tile_type.is_walkable()
            && !self.game_rules.contains(GameRules::NO_CLIP)
        {
            return Ok(GameOutcome::Fail(FailReason::TileNotWalkable(new_pos)));
        }

        self.player.character.move_to(new_pos);

        Ok(GameOutcome::Success)
    }

    fn tile_interaction(&mut self, point: Point) -> GameResult {
        let tile = self.current_world_mut().get_tile_mut(point);

        match tile.tile_type {
            TileType::Door(DoorType::Closed) => {
                tile.tile_type = TileType::Door(DoorType::Open);
                self.log.print("You open the door".to_string());
                Ok(GameOutcome::Success)
            }

            TileType::StairsDown => {
                self.log.info(LogData::UseStairsDown);
                self.goto_level_next()?;
                Ok(GameOutcome::Success)
            }

            TileType::StairsUp => {
                self.log.info(LogData::UseStairsUp);
                self.goto_level_previous()?;
                Ok(GameOutcome::Success)
            }

            _ => Ok(GameOutcome::Fail(FailReason::NoInteraction)),
        }
    }
}
