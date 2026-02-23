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

/// Abstraction layer over player input. These represent the player's input separated from the concrete keybindings.
pub enum PlayerInput {
    /// Wait in place for 1 round.
    Wait,

    /// Directional input. Can be used for attacking, interacting, or moving.
    Direction(Direction),

    /// Use an item from the inventory.
    UseItem(GameItemId),

    /// Drop an item from the inventory.
    DropItem(GameItemId),

    /// Unequip the weapon currently in the weapon slot.
    UnequipWeapon,

    /// Unequip the weapon currently in the weapon slot.
    UnequipArmor,

    /// Make a ranged attack.
    RangedAttack(EntityId),
}

/// Actions/Intentions of the player. Are translated from [PlayerInput] in the context of the game state.
pub enum ActionKind {
    /// Wait in place for 1 round.
    Wait,

    /// Move in the given direction.
    Move(Direction),

    /// Attack the given Entity (likely Npc)
    Attack(EntityId),

    /// Pick up the item contained in the given Entity (likely GameItemSprite)
    PickUpItem(EntityId),

    /// Use an item from the inventory.
    UseItem(GameItemId),

    /// Drop an item from the inventory onto the ground as an ItemSprite.
    DropItem(GameItemId),

    /// Unequip the weapon in the current weapon slot.
    UnequipWeapon,

    /// Unequip the weapon in the current weapon slot.
    UnequipArmor,

    /// Perform an interaction with the tile at the given point.
    TileInteraction(Point),

    /// Make a ranged attack against the given Entity.
    RangedAttack(EntityId),
}

impl GameState {
    /// Interprets the player input and executes the intended action.
    ///
    /// Main engine that moves the game forward. These "Actions" all move the game forward one round, which updates all the moving parts of the game.
    ///
    /// # Panics
    /// * If a static level file could not be loaded.
    /// * If the level data from a loaded file or procedurally generated is corrupt and cannot be applied to a level.
    ///
    /// These break the game's state, meaning that the game cannot be continued.
    pub fn resolve_player_action(&mut self, input: PlayerInput) {
        if let Some(intended_action) = self.interpret_player_input(input) {
            let action_result: GameResult = match intended_action {
                ActionKind::Wait => Ok(GameOutcome::Success),
                ActionKind::Move(direction) => {
                    self.move_player_character(PointVector::from(direction))
                }
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

    /// Accepts [PlayerInput] and interprets the action that would result of that input.
    ///
    /// This is done because directional inputs can mean many things (pick up item, attack npc, move in direction, interact with world object).
    ///
    /// # Returns
    /// * Some([ActionKind]) if the input could be interpreted.
    /// * None if the target point of directional movement would be out of bounds.
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

    /// Used to pick up items off the ground. Moves the item from a [GameItemSprite] to the player's inventory.
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

    /// Used to drop items from the inventory onto the ground. Spawns a new [GameItemSprite] in the world.
    fn drop_item(&mut self, item_id: GameItemId) -> GameResult {
        let player_pos = self.player.character.pos();

        if self.current_level().is_occupied(player_pos) {
            return Ok(GameOutcome::Fail(FailReason::TileOccupied(player_pos)));
        }

        self.remove_item_from_inv(item_id)?;

        let item_sprite = self.create_item_sprite(item_id, player_pos)?;
        self.current_level_mut().spawn_item_sprite(item_sprite)?;

        Ok(GameOutcome::Success)
    }

    /// Moves the player character to a new relative position described by the `point_vector` argument.
    ///
    /// Performs out of bounds and tile accessibility checks.
    fn move_player_character(&mut self, point_vector: PointVector) -> GameResult {
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

    /// The player performs an interaction with a tile at the given point.
    ///
    /// Does nothing if the target tile has no defined interactions.
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
