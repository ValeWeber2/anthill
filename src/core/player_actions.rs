use crate::{
    core::{
        entity_logic::{Entity, EntityId},
        game::GameState,
        game_items::GameItemId,
    },
    util::errors_results::{EngineError, GameOutcome, GameResult},
    world::{
        coordinate_system::{Direction, Point},
        tiles::TileType,
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
}

impl GameState {
    pub fn resolve_player_action(&mut self, input: PlayerInput) {
        let intended_action = self.interpret_player_input(input);

        let action_result: GameResult = match intended_action {
            ActionKind::Wait => Ok(GameOutcome::Success),
            ActionKind::Move(Direction::Up) => {
                self.world.move_player_character(&mut self.player.character, 0, -1)
            }
            ActionKind::Move(Direction::Right) => {
                self.world.move_player_character(&mut self.player.character, 1, 0)
            }
            ActionKind::Move(Direction::Down) => {
                self.world.move_player_character(&mut self.player.character, 0, 1)
            }
            ActionKind::Move(Direction::Left) => {
                self.world.move_player_character(&mut self.player.character, -1, 0)
            }
            ActionKind::Attack(npc_id) => self.player_attack_npc(npc_id),
            ActionKind::PickUpItem(entity_id) => self.pick_up_item(entity_id),
            ActionKind::DropItem(item_id) => self.drop_item(item_id),
            ActionKind::UseItem(item_id) => self.use_item(item_id),
            ActionKind::UnequipWeapon => self.unequip_weapon(),
            ActionKind::UnequipArmor => self.unequip_armor(),
        };

        match action_result {
            Ok(GameOutcome::Success) => {
                let pos = self.player.character.pos();
                let tile = self.world.get_tile(pos).tile_type;

                if let TileType::Stair = tile {
                    let next = self.current_level + 1;

                    if next < self.level_paths.len() {
                        self.log.print("You go down the stairs...".to_string());
                        let _ = self.load_level(next);
                    } else {
                        self.log.print("This stair leads nowhere...".to_string()); //test later
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
                // let target_tile = self.world.get_tile(target_point.x, target_point.y);

                if let Some(entity_id) = self.get_entity_at(target_point) {
                    if self.get_npc(entity_id).is_some() {
                        return ActionKind::Attack(entity_id);
                    }
                    if self.get_item_sprite(entity_id).is_some() {
                        return ActionKind::PickUpItem(entity_id);
                    }
                }

                ActionKind::Move(direction)
            }
            PlayerInput::UseItem(item_id) => ActionKind::UseItem(item_id),
            PlayerInput::DropItem(item_id) => ActionKind::DropItem(item_id),
            PlayerInput::UnequipWeapon => ActionKind::UnequipWeapon,
            PlayerInput::UnequipArmor => ActionKind::UnequipArmor,
        }
    }

    fn pick_up_item(&mut self, entity_id: EntityId) -> GameResult {
        let item_sprite =
            self.get_item_sprite(entity_id).ok_or(EngineError::ItemSpriteNotFound(entity_id))?;

        let result = self.add_item_to_inv(item_sprite.item_id);

        if let Ok(GameOutcome::Success) = result {
            self.despawn(entity_id);
        }

        result
    }

    fn drop_item(&mut self, item_id: GameItemId) -> GameResult {
        self.remove_item_from_inv(item_id)?;

        let player_pos = self.player.character.pos();
        self.spawn_item(item_id, player_pos)?;

        Ok(GameOutcome::Success)
    }
}
