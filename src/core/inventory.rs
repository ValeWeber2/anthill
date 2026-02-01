#![allow(dead_code)]

use crate::{
    core::{
        game::GameState,
        game_items::{ArmorItem, GameItemId, GameItemKindDef, WeaponItem},
        text_log::LogData,
    },
    util::errors_results::{
        DataError, EngineError, FailReason, GameError, GameOutcome, GameResult,
    },
};

pub const INVENTORY_LIMIT: usize = 26;

impl GameState {
    pub fn add_item_to_inv(&mut self, item_id: u32) -> GameResult {
        if self.player.character.inventory.len() >= INVENTORY_LIMIT {
            self.log.debug_print(format!("Inventory full. Couldn't add item {}", item_id));
            return Ok(GameOutcome::Fail(FailReason::InventoryFull));
        }

        self.player.character.inventory.push(item_id);
        Ok(GameOutcome::Success)
    }

    pub fn remove_item_from_inv(&mut self, item_id: u32) -> GameResult {
        let search_item = self.player.character.inventory.iter().position(|item| *item == item_id);

        if let Some(found_item) = search_item {
            self.player.character.inventory.swap_remove(found_item);
        } else {
            let error = GameError::from(EngineError::ItemNotInInventory(item_id));
            self.log.debug_print(format!("Couldn't remove item {}: {}", item_id, error));
            return Err(error);
        }

        Ok(GameOutcome::Success)
    }

    /// Uses an item from the player's inventory.
    ///
    /// Checks whether the item is present, resolves its definition, and
    /// dispatches to the appropriate handler (armor, weapon, or food).  
    /// Returns an error if the item is missing or unregistered.
    pub fn use_item(&mut self, item_id: u32) -> GameResult {
        let search_item = self.player.character.inventory.iter().position(|item| *item == item_id);

        if search_item.is_some() {
            let item =
                self.get_item_by_id(item_id).ok_or(EngineError::UnregisteredItem(item_id))?;

            let item_def = self
                .get_item_def_by_id(item.def_id.clone())
                .ok_or(DataError::MissingItemDefinition(item.def_id))?;

            match item_def.kind {
                GameItemKindDef::Armor { .. } => self.use_armor(item_id),
                GameItemKindDef::Weapon { .. } => self.use_weapon(item_id),
                GameItemKindDef::Food { nutrition } => self.use_food(item_id, nutrition),
            }
        } else {
            let error = GameError::from(EngineError::ItemNotInInventory(item_id));
            self.log.debug_print(format!("Couldn't use item {}: {}", item_id, error));
            Err(error)
        }
    }

    pub fn use_armor(&mut self, item_id: GameItemId) -> GameResult {
        self.remove_item_from_inv(item_id)?;

        // if old armor exists, return it to inventory
        if let Some(old_armor) = self.player.character.armor.take() {
            self.add_item_to_inv(old_armor.0)?;
        }

        // equip the new armor
        self.player.character.armor = Some(ArmorItem(item_id));

        Ok(GameOutcome::Success)
    }

    pub fn use_weapon(&mut self, item_id: GameItemId) -> GameResult {
        self.remove_item_from_inv(item_id)?;

        // if old weapon exists, return it to inventory
        if let Some(old_weapon) = self.player.character.weapon.take() {
            self.add_item_to_inv(old_weapon.0)?;
        }

        self.player.character.weapon = Some(WeaponItem(item_id));

        Ok(GameOutcome::Success)
    }

    pub fn use_food(&mut self, item_id: GameItemId, nutrition: u16) -> GameResult {
        self.player.character.stats.base.hp_current = (self.player.character.stats.base.hp_current
            + nutrition)
            .min(self.player.character.stats.base.hp_max); // multiply by some factor?
        self.remove_item_from_inv(item_id)?;
        let item_name = {
            let item =
                self.get_item_by_id(item_id).ok_or(EngineError::UnregisteredItem(item_id))?;
            let def = self
                .get_item_def_by_id(item.def_id.clone())
                .ok_or(DataError::MissingItemDefinition(item.def_id))?;
            def.name.to_string()
        };

        self.log.info(LogData::PlayerEats { item_name });
        self.deregister_item(item_id)?;

        Ok(GameOutcome::Success)
    }

    pub fn unequip_armor(&mut self) -> GameResult {
        if let Some(armor_item) = self.player.character.armor.take() {
            self.add_item_to_inv(armor_item.0)?;

            Ok(GameOutcome::Success)
        } else {
            Ok(GameOutcome::Fail(FailReason::CannotUnequipEmptySlot))
        }
    }

    pub fn unequip_weapon(&mut self) -> GameResult {
        if let Some(weapon_item) = self.player.character.weapon.take() {
            self.add_item_to_inv(weapon_item.0)?;

            Ok(GameOutcome::Success)
        } else {
            Ok(GameOutcome::Fail(FailReason::CannotUnequipEmptySlot))
        }
    }
}
