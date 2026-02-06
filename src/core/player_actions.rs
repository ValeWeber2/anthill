use crate::{
    core::{
        entity_logic::{Entity, EntityId, Movable},
        game::{GameRules, GameState},
        game_items::GameItemId,
    },
    data::levels::level_paths,
    util::errors_results::{EngineError, FailReason, GameOutcome, GameResult},
    world::{
        coordinate_system::{Direction, Point, PointVector},
        tiles::{Collision, TileType},
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
    RangedAttack(EntityId),
}

impl GameState {
    pub fn resolve_player_action(&mut self, input: PlayerInput) {
        let intended_action = self.interpret_player_input(input);

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
            ActionKind::RangedAttack(npc_id) => self.player_ranged_attack_npc(npc_id),
        };

        match action_result {
            Ok(GameOutcome::Success) => {
                let pos = self.player.character.pos();
                let tile = self.current_world().get_tile(pos).tile_type;

                if let TileType::Stair = tile {
                    let next = self.level_nr + 1;

                    if next <= level_paths().len() {
                        self.log.print("You go down the stairs...".to_string());
                        // let _ = self.load_static_level(next);
                        let _ = self.goto_level(self.level_nr + 1);
                    } else {
                        self.log.print("This stair leads nowhere...".to_string()); //test later
                        self.log.print(format!("{} {}", next, level_paths().len()))
                    }
                }

                self.next_round();
            }

            Ok(GameOutcome::Fail(reason)) => {
                // Log for user only if message is defined for user
                if let Some(message) = reason.notify_user() {
                    self.log.print(message.to_string());
                }
            }
            Err(error) => {
                // Log for Debugging
                self.log.debug_print(error.to_string());
            }
        }
    }

    pub fn interpret_player_input(&mut self, input: PlayerInput) -> ActionKind {
        match input {
            PlayerInput::Wait => ActionKind::Wait,
            PlayerInput::Direction(direction) => {
                let target_point: Point = self.player.character.pos().get_adjacent(direction);
                // let target_tile = self.get_current_world().get_tile(target_point.x, target_point.y);

                if let Some(entity_id) = self.current_level().get_entity_at(target_point) {
                    if self.current_level().get_npc(entity_id).is_some() {
                        return ActionKind::Attack(entity_id);
                    }
                    if self.current_level().get_item_sprite(entity_id).is_some() {
                        return ActionKind::PickUpItem(entity_id);
                    }
                }

                ActionKind::Move(direction)
            }
            PlayerInput::UseItem(item_id) => ActionKind::UseItem(item_id),
            PlayerInput::DropItem(item_id) => ActionKind::DropItem(item_id),
            PlayerInput::UnequipWeapon => ActionKind::UnequipWeapon,
            PlayerInput::UnequipArmor => ActionKind::UnequipArmor,
            PlayerInput::RangedAttack(entity_id) => ActionKind::RangedAttack(entity_id),
        }
    }

    fn pick_up_item(&mut self, entity_id: EntityId) -> GameResult {
        let item_sprite = self
            .current_level()
            .get_item_sprite(entity_id)
            .ok_or(EngineError::ItemSpriteNotFound(entity_id))?;

        let result = self.add_item_to_inv(item_sprite.item_id);

        if let Ok(GameOutcome::Success) = result {
            self.current_level_mut().despawn(entity_id);
        }

        result
    }

    fn drop_item(&mut self, item_id: GameItemId) -> GameResult {
        self.remove_item_from_inv(item_id)?;

        let player_pos = self.player.character.pos();
        self.create_item_sprite(item_id, player_pos)?;

        Ok(GameOutcome::Success)
    }

    pub fn move_player_character(&mut self, dx: isize, dy: isize) -> GameResult {
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
}
