use crate::{
    ai::npc_ai::NpcAiError,
    core::{entity_logic::EntityId, game::GameState, game_items::GameItemKindDef},
    util::{
        errors_results::{DataError, EngineError, FailReason, GameError, GameOutcome, GameResult},
        rng::{DieSize, Roll},
    },
};

impl GameState {
    pub fn player_attack_npc(&mut self, npc_id: EntityId) -> GameResult {
        // Fetching values
        let npc = self.current_level().get_npc(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;
        let npc_name = npc.base.name.clone();
        let npc_mitigation = npc.stats.mitigation;
        let npc_dodge_chance = npc.stats.dodge_chance();

        // Damage
        let base_damage = self.player.character.attack_damage_bonus();
        let (weapon_damage, crit_chance): (u16, u8) = self.get_player_weapon_damage()?;

        // Calculate resulting damage (if any)
        let attack_result = self.resolve_attack(
            base_damage + weapon_damage,
            crit_chance,
            npc_dodge_chance,
            npc_mitigation,
        );

        let mut attack_message: String;

        match attack_result {
            None => {
                attack_message = format!("{} dodged the attack!", npc_name);
            }
            Some(damage) => {
                // Apply Damage
                let npc = self
                    .current_level_mut()
                    .get_npc_mut(npc_id)
                    .ok_or(EngineError::NpcNotFound(npc_id))?;
                npc.stats.base.take_damage(damage);

                attack_message = format!("Player hits {} for {} damage!", npc.base.name, damage);

                if !npc.stats.base.is_alive() {
                    attack_message = format!("{}\n{} died!", attack_message, npc.base.name);
                    self.current_level_mut().despawn(npc_id);
                    self.player.character.gain_experience(25);
                }
            }
        }

        self.log.print(attack_message);

        Ok(GameOutcome::Success)
    }

    pub fn player_ranged_attack_npc(&mut self, npc_id: EntityId) -> GameResult {
        if self.current_level().get_npc(npc_id).is_none() {
            return Ok(GameOutcome::Fail(FailReason::InvalidTarget(npc_id))); // Target entity is not an npc
        }

        let Some(weapon_id) = self.player.character.weapon else {
            return Ok(GameOutcome::Fail(FailReason::EquipmentSlotEmpty)); // No weapon equipped
        };

        let weapon_item =
            self.get_item_by_id(weapon_id.0).ok_or(EngineError::UnregisteredItem(weapon_id.0))?; // Weapon not a registered item

        let weapon_def = self
            .get_item_def_by_id(weapon_item.def_id.clone())
            .ok_or(DataError::MissingItemDefinition(weapon_item.def_id))?; // Weapon is not defined

        let GameItemKindDef::Weapon { ranged: true, .. } = weapon_def.kind else {
            return Err(GameError::from(EngineError::InvalidItem(weapon_def.kind))); // Weapon is not ranged
        };

        self.player_attack_npc(npc_id)
    }

    pub fn npc_attack_player(&mut self, npc_id: EntityId) -> Result<(), NpcAiError> {
        let (npc_name, npc_damage) = {
            let npc = self.current_level().get_npc(npc_id).ok_or(NpcAiError::NpcNotFound)?;
            (npc.base.name.to_string(), npc.stats.damage)
        };

        let attack_result = self.resolve_attack(
            npc_damage,
            5,
            self.player.character.dodge_chance(),
            self.get_player_armor_mitigation().unwrap_or(0),
        );

        match attack_result {
            None => {
                self.log.print("Player dodged the attack!".to_string());
            }
            Some(damage) => {
                self.player.character.take_damage(damage);
                self.log.print(format!("{} hits Player for {} damage!", npc_name, damage,));
            }
        }

        Ok(())
    }

    /// Rolls to see if a dodg occurs.
    fn dodge_roll(&mut self, dodge_chance: u8) -> bool {
        self.roll(&Roll::new(1, DieSize::D100)) as u8 <= dodge_chance
    }

    /// Rolls to see if a critical strike occurs.
    fn is_critical_strike(&mut self, crit_chance: u8) -> bool {
        self.roll(&Roll::new(1, DieSize::D100)) as u8 <= crit_chance
    }

    /// Resolves all computation steps as part of attack. Returns the damage dealt (if any).
    fn resolve_attack(
        &mut self,
        attacker_damage: u16,
        attacker_crit_chance: u8,
        defender_dodge_chance: u8,
        defender_mitigation: u16,
    ) -> Option<u16> {
        if self.dodge_roll(defender_dodge_chance) {
            return None;
        }

        let damage_unmitigated = if self.is_critical_strike(attacker_crit_chance) {
            2 * attacker_damage
        } else {
            attacker_damage
        };

        let damage_mitigated = damage_unmitigated.saturating_sub(defender_mitigation);

        Some(damage_mitigated)
    }

    fn get_player_weapon_damage(&self) -> Result<(u16, u8), GameError> {
        if let Some(weapon) = &self.player.character.weapon {
            let item_id =
                self.get_item_by_id(weapon.0).ok_or(EngineError::UnregisteredItem(weapon.0))?;
            let item_def = self
                .get_item_def_by_id(item_id.def_id.clone())
                .ok_or(DataError::MissingItemDefinition(item_id.def_id))?;

            match item_def.kind {
                GameItemKindDef::Weapon { damage, crit_chance, ranged: _ranged } => {
                    Ok((damage, crit_chance))
                }
                _ => Err(GameError::from(EngineError::InvalidItem(item_def.kind))),
            }
        } else {
            Ok((1, 5)) // If no weapon is equipped, fist damage is just 1.
        }
    }

    fn get_player_armor_mitigation(&self) -> Result<u16, &'static str> {
        if let Some(armor) = &self.player.character.armor {
            let item_id = self.get_item_by_id(armor.0).ok_or("The item is not registered")?;
            let item_def =
                self.get_item_def_by_id(item_id.def_id).ok_or("The item is not defined")?;

            match item_def.kind {
                GameItemKindDef::Armor { mitigation } => Ok(mitigation),
                _ => Err("The given item is not a weapon"),
            }
        } else {
            Ok(0)
        }
    }
}
