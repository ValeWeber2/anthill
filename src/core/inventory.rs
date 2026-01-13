#![allow(dead_code)]

use std::fmt::{self, Display, Formatter};

use crate::core::{
    game::GameState,
    game_items::{ArmorItem, GameItemId, GameItemKindDef, WeaponItem},
};

impl GameState {
    pub fn add_item_to_inv(&mut self, item_id: u32) -> Result<(), InventoryError> {
        if self.player.character.inventory.len() >= 24 {
            let error = InventoryError::InventoryFull;
            self.log.debug_print(format!("Couldn't add item {}: {}", item_id, error));
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
            self.log.debug_print(format!("Couldn't remove item {}: {}", item_id, error));
            return Err(error);
        }

        Ok(())
    }

    pub fn use_item(&mut self, item_id: u32) -> Result<(), InventoryError> {
        let search_item = self.player.character.inventory.iter().position(|item| *item == item_id);

        if search_item.is_some() {
            let item = self.get_item_by_id(item_id).ok_or(InventoryError::ItemNotFound)?;

            let item_def =
                { self.get_item_def_by_id(item.def_id).ok_or(InventoryError::ItemNotFound)? };

            match item_def.kind {
                GameItemKindDef::Armor { .. } => self.use_armor(&item_id),
                GameItemKindDef::Weapon { .. } => self.use_weapon(&item_id),
                GameItemKindDef::Food { nutrition } => self.use_food(&item_id, nutrition),
            }
        } else {
            let error = InventoryError::ItemNotInInventory;
            self.log.print(format!("Couldn't use item {}: {}", item_id, error));
            Err(error)
        }
    }

    pub fn use_armor(&mut self, item_id: &GameItemId) -> Result<(), InventoryError> {
        self.remove_item_from_inv(*item_id)?;

        // if old armor exists, return it to inventory
        if let Some(old_armor) = self.player.character.armor.take() {
            self.add_item_to_inv(old_armor.0)?;
        }

        // equip the new armor
        self.player.character.armor = Some(ArmorItem(*item_id));

        Ok(())
    }

    pub fn use_weapon(&mut self, item_id: &GameItemId) -> Result<(), InventoryError> {
        self.remove_item_from_inv(*item_id)?;

        // if old weapon exists, return it to inventory
        if let Some(old_weapon) = self.player.character.weapon.take() {
            self.add_item_to_inv(old_weapon.0)?;
        }

        self.player.character.weapon = Some(WeaponItem(*item_id));

        Ok(())
    }

    pub fn use_food(&mut self, item_id: &GameItemId, nutrition: u16) -> Result<(), InventoryError> {
        self.player.character.stats.base.hp_current = (self.player.character.stats.base.hp_current
            + nutrition)
            .min(self.player.character.stats.base.hp_max); // multiply by some factor?
        self.remove_item_from_inv(*item_id)?;
        let item_name = {
            let (_, item) =
                self.items.get_key_value(item_id).ok_or(InventoryError::ItemNotFound)?;
            let def =
                self.get_item_def_by_id(item.def_id.clone()).ok_or(InventoryError::ItemNotFound)?;
            def.name
        };

        self.log.print(format!("You have eaten {}.", item_name));
        Ok(())
    }

    pub fn unequip_armor(&mut self) -> Result<(), InventoryError> {
        if let Some(armor_item) = self.player.character.armor.take() {
            self.add_item_to_inv(armor_item.0)?;

            Ok(())
        } else {
            Err(InventoryError::NoArmorEquipped)
        }
    }

    pub fn unequip_weapon(&mut self) -> Result<(), InventoryError> {
        if let Some(weapon_item) = self.player.character.weapon.take() {
            self.add_item_to_inv(weapon_item.0)?;

            Ok(())
        } else {
            Err(InventoryError::NoWeaponEquipped)
        }
    }
}

#[derive(Debug)]
pub enum InventoryError {
    InventoryFull,
    ItemNotInInventory,
    NoArmorEquipped,
    NoWeaponEquipped,
    ItemNotFound,
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
            InventoryError::NoArmorEquipped => {
                write!(f, "No armor is equipped.")
            }
            InventoryError::NoWeaponEquipped => {
                write!(f, "No weapon is equipped.")
            }
            InventoryError::ItemNotFound => {
                write!(f, "Item not found.")
            }
        }
    }
}
