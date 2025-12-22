#![allow(dead_code)]

use std::fmt::{self, Display, Formatter};

use crate::core::game::GameState;

impl GameState {
    pub fn add_item_to_inv(&mut self, item_id: u32) -> Result<(), InventoryError> {
        if self.player.character.inventory.len() >= 24 {
            let error = InventoryError::InventoryFull;
            self.log.messages.push(format!("Couldn't add item {}: {}", item_id, error));
            return Err(error);
        }

        self.player.character.inventory.push(item_id);
        Ok(())
    }

    pub fn remove_item_from_inv(&mut self, item_id: u32) -> Result<(), InventoryError> {
        let search_item = self.player.character.inventory.iter().position(|item| *item == item_id);

        if let Some(found_item) = search_item {
            self.player.character.inventory.swap_remove(found_item);
        } else {
            let error = InventoryError::ItemNotInInventory;
            self.log.messages.push(format!("Couldn't remove item {}: {}", item_id, error));
            return Err(error);
        }

        Ok(())
    }
}

pub enum InventoryError {
    InventoryFull,
    ItemNotInInventory,
}

impl Display for InventoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::InventoryFull => {
                write!(f, "Inventory is full.")
            }
            InventoryError::ItemNotInInventory => {
                write!(f, "Item not found in inventory.")
            }
        }
    }
}
