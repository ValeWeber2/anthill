#![allow(dead_code)]

use std::fmt::{self, Display, Formatter};

use crate::{
    core::{
        entity_logic::{Entity, EntityId, EntityRef, SpawningError},
        game::GameState,
        game_items::GameItemId,
        inventory::InventoryError,
    },
    world::worldspace::{Direction, MovementError, Point},
};

pub enum PlayerInput {
    Wait,
    Direction(Direction),
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

        let action_result: Result<(), GameActionError> = match intended_action {
            ActionKind::Wait => Ok(()),
            ActionKind::Move(Direction::Up) => self
                .world
                .move_entity(&mut self.player.character, 0, -1)
                .map_err(GameActionError::MovementError),
            ActionKind::Move(Direction::Right) => self
                .world
                .move_entity(&mut self.player.character, 1, 0)
                .map_err(GameActionError::MovementError),
            ActionKind::Move(Direction::Down) => self
                .world
                .move_entity(&mut self.player.character, 0, 1)
                .map_err(GameActionError::MovementError),
            ActionKind::Move(Direction::Left) => self
                .world
                .move_entity(&mut self.player.character, -1, 0)
                .map_err(GameActionError::MovementError),
            ActionKind::Attack(_) => todo!(),
            ActionKind::PickUpItem(entity_id) => self.pick_up_item(entity_id),
            ActionKind::DropItem(item_id) => self.drop_item(item_id),
            ActionKind::UseItem(item_id) => {
                self.use_item(item_id).map_err(GameActionError::InventoryError)
            }
            ActionKind::UnequipWeapon => {
                self.unequip_weapon().map_err(GameActionError::InventoryError)
            }
            ActionKind::UnequipArmor => {
                self.unequip_armor().map_err(GameActionError::InventoryError)
            }
        };

        match action_result {
            Ok(()) => self.next_round(),
            Err(error) => {
                // Log for Debugging
                self.log.debug_print(error.to_string());

                // Log for user only if defined for user.
                if let Some(message) = error.notify_user() {
                    self.log.print(message.to_string());
                }
            }
        }
    }

    pub fn interpret_player_input(&mut self, input: PlayerInput) -> ActionKind {
        match input {
            PlayerInput::Direction(direction) => {
                let target_point: Point = self.player.character.pos().get_neighbour(direction);
                // let target_tile = self.world.get_tile(target_point.x, target_point.y);

                if let Some(entity_id) = self.get_entity_at(target_point) {
                    match self.get_entity_by_id(entity_id) {
                        Some(EntityRef::Npc(_)) => {
                            return ActionKind::Attack(entity_id);
                        }
                        Some(EntityRef::ItemSprite(_)) => {
                            return ActionKind::PickUpItem(entity_id);
                        }
                        _ => {}
                    }
                }

                ActionKind::Move(direction)
            }
            PlayerInput::DropItem(item_id) => ActionKind::DropItem(item_id),
            PlayerInput::Wait => ActionKind::Wait,
            PlayerInput::UseItem(item_id) => ActionKind::UseItem(item_id),
            PlayerInput::UnequipWeapon => ActionKind::UnequipWeapon,
            PlayerInput::UnequipArmor => ActionKind::UnequipArmor,
        }
    }

    fn pick_up_item(&mut self, entity_id: EntityId) -> Result<(), GameActionError> {
        let entity_ref =
            self.get_entity_by_id(entity_id).ok_or(GameActionError::EntityNotFound(entity_id))?;

        let item_sprite = match entity_ref {
            EntityRef::ItemSprite(item_sprite) => item_sprite,
            _ => return Err(GameActionError::NotAnItem(entity_id)),
        };

        self.add_item_to_inv(item_sprite.item_id).map_err(GameActionError::InventoryError)?;
        self.despawn(entity_id);

        Ok(())
    }

    fn drop_item(&mut self, item_id: GameItemId) -> Result<(), GameActionError> {
        self.remove_item_from_inv(item_id).map_err(GameActionError::InventoryError)?;

        let player_pos = self.player.character.pos();
        self.spawn_item(item_id, *player_pos).map_err(GameActionError::SpawningError)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum GameActionError {
    MovementError(MovementError),
    EntityNotFound(EntityId),
    NotAnItem(EntityId),
    InventoryError(InventoryError),
    SpawningError(SpawningError),
}

impl GameActionError {
    fn notify_user(&self) -> Option<&'static str> {
        match self {
            GameActionError::InventoryError(InventoryError::NoArmorEquipped) => {
                Some("You do not have a weapon equipped.")
            }
            GameActionError::InventoryError(InventoryError::NoWeaponEquipped) => {
                Some("You are not wearing any armor.")
            }
            _ => None,
        }
    }
}

impl Display for GameActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameActionError::MovementError(movement_error) => {
                write!(f, "Movement Error: {}", movement_error)
            }
            GameActionError::EntityNotFound(entity_id) => {
                write!(f, "Entity {} not found", entity_id)
            }
            GameActionError::NotAnItem(entity_id) => {
                write!(f, "Entity {} is not an item", entity_id)
            }
            GameActionError::InventoryError(inventory_error) => {
                write!(f, "Inventory Error: {}", inventory_error)
            }
            GameActionError::SpawningError(spawning_error) => {
                write!(f, "Spawning Error: {}", spawning_error)
            }
        }
    }
}
