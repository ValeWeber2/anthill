use crate::{
    core::{
        entity_logic::{Entity, EntityId},
        game::GameState,
        game_items::{AttackRange, GameItemKindDef},
    },
    util::{
        errors_results::{DataError, EngineError, FailReason, GameError, GameOutcome, GameResult},
        rng::{DieSize, Roll},
        text_log::LogData,
    },
};

/// Defines the degrees of success an attack can have.
enum AttackDegree {
    /// The attack missed and nothing happens.
    Miss,

    /// The attack hits and deals the listed damage.
    Hit(u16),

    /// The attack hits critically and deals the listed damage, which is even more than on a hit.
    CriticalHit(u16),
}

impl GameState {
    /// Handles a player attacking an npc.
    ///
    /// # Side Effects
    /// * `GameState::rng`` is used.
    /// * Calls `Npc::stats.base.take_damage()`
    /// * Calls `GameState::player_add_experience()`
    /// * Calls `Level::despawn()`
    ///
    /// # Errors
    /// * [EngineError::NpcNotFound] if the NPC with the given id could not be found in the current Level.
    /// * [DataError::MissingItemDefinition] if the player's weapon has no definition.
    /// * [EngineError::UnregisteredItem] if the player's weapon is not registered.
    /// * [EngineError::InvalidItem] if the player's item equipped in the weapon slot is not a valid weapon.
    ///
    /// # Returns
    /// * [GameOutcome::Success] if the attack resolution was successful.
    pub fn player_attack_npc(&mut self, npc_id: EntityId) -> GameResult {
        // Fetching values
        let npc = self.current_level().get_npc(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;
        let npc_name = npc.name().to_string();
        let npc_mitigation = npc.stats.mitigation;
        let npc_dodge_chance = npc.stats.dodge_chance();

        // Damage
        let (weapon_damage, crit_chance, range): (Roll, u8, AttackRange) =
            self.get_player_weapon_stats()?;
        let base_damage = if range.is_some() {
            self.player.character.attack_damage_bonus_ranged()
        } else {
            self.player.character.attack_damage_bonus_melee()
        };
        let rolled_damage = self.roll(&weapon_damage) as u16;

        // Calculate resulting damage (if any)
        let attack_result = self.resolve_attack(
            rolled_damage.saturating_add_signed(base_damage),
            crit_chance,
            npc_dodge_chance,
            npc_mitigation,
        );

        let attack_message: LogData = match attack_result {
            AttackDegree::Miss => LogData::PlayerAttackMiss { npc_name },
            AttackDegree::Hit(damage) => {
                let npc = self
                    .current_level_mut()
                    .get_npc_mut(npc_id)
                    .ok_or(EngineError::NpcNotFound(npc_id))?;
                npc.stats.base.take_damage(damage);
                LogData::PlayerAttackHit { npc_name, damage }
            }
            AttackDegree::CriticalHit(damage) => {
                let npc = self
                    .current_level_mut()
                    .get_npc_mut(npc_id)
                    .ok_or(EngineError::NpcNotFound(npc_id))?;
                npc.stats.base.take_damage(damage);
                LogData::PlayerAttackHitCritical { npc_name, damage }
            }
        };

        self.log.info(attack_message);

        // Checks if the npc is dead. Later this will be moved into some central event handler.
        let npc = self.current_level().get_npc(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;
        let npc_name = npc.name().to_string();
        if !npc.stats.base.is_alive() {
            self.log.info(LogData::NpcDied { npc_name });
            self.current_level_mut().despawn(npc_id);
            self.player_add_experience(25);
        }

        Ok(GameOutcome::Success)
    }

    /// Handles a player attacking an npc with a ranged weapon. Conducts all checks required to validate the ranged attack and then calls [GameState::player_attack_npc]
    /// Find side effects and returns [GameState::player_attack_npc].
    ///
    /// # Side Effects
    /// Calls [GameState::player_attack_npc] (with all its side effects)
    ///
    /// # Errors
    /// * [EngineError::NpcNotFound] if the NPC with the given id could not be found in the current Level.
    /// * [DataError::MissingItemDefinition] if the player's weapon has no definition.
    /// * [EngineError::UnregisteredItem] if the player's weapon is not registered.
    /// * [EngineError::InvalidItem] if the player's item equipped in the weapon slot is not a valid weapon and not ranged.
    ///
    /// # Returns
    /// * [GameOutcome::Fail] with [FailReason::InvalidTarget] if the target is invalid for a ranged attack.
    /// * [GameOutcome::Fail] with [FailReason::EquipmentSlotEmpty] if the player has no weapon equipped.
    /// * [GameOutcome::Fail] with [FailReason::EquipmentSlotEmpty] if the player has no weapon equipped.
    /// * [GameOutcome::Fail] with [FailReason::OutOfRange] if the ranged weapon's range is not sufficient for the attack.
    pub fn player_ranged_attack_npc(&mut self, npc_id: EntityId) -> GameResult {
        let Some(npc) = self.current_level().get_npc(npc_id) else {
            return Ok(GameOutcome::Fail(FailReason::InvalidTarget(npc_id))); // Target entity is not an npc
        };

        let Some(weapon_id) = self.player.character.weapon else {
            return Ok(GameOutcome::Fail(FailReason::EquipmentSlotEmpty)); // No weapon equipped
        };

        let weapon_item =
            self.get_item_by_id(weapon_id.0).ok_or(EngineError::UnregisteredItem(weapon_id.0))?; // Weapon not a registered item

        let weapon_def = self
            .get_item_def_by_id(&weapon_item.def_id)
            .ok_or(DataError::MissingItemDefinition(weapon_item.def_id))?; // Weapon is not defined

        let GameItemKindDef::Weapon { range: Some(range), .. } = weapon_def.kind else {
            return Err(GameError::from(EngineError::InvalidItem(weapon_def.kind))); // Weapon is not ranged
        };

        if self.player.character.pos().distance_squared_from(npc.pos()) > range.pow(2) {
            return Ok(GameOutcome::Fail(FailReason::OutOfRange)); // Bow attack out of range
        }

        self.player_attack_npc(npc_id)
    }

    /// Handles an NPC attacking a player.
    ///
    /// # Errors
    /// * [EngineError::NpcNotFound] if the NPC with the given id could not be found in the current Level.
    ///
    /// # Returns
    /// * [Ok] if the procedure was successful.
    pub fn npc_attack_player(&mut self, npc_id: EntityId) -> Result<(), GameError> {
        let (npc_name, npc_damage) = {
            let npc =
                self.current_level().get_npc(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;
            (npc.base.name.to_string(), npc.stats.damage)
        };

        // Roll the damage and add the current level. This increases monster damage the deeper you go, increasing difficulty.
        let rolled_damage = self.roll(&npc_damage.add_modifier(self.level_nr as i16)) as u16;

        let attack_result = self.resolve_attack(
            rolled_damage,
            5,
            self.player.character.dodge_chance(),
            self.get_player_armor_mitigation().unwrap_or(0),
        );

        match attack_result {
            AttackDegree::Miss => {
                self.log.info(LogData::NpcAttackMiss { npc_name });
            }
            AttackDegree::Hit(damage) => {
                self.player.character.take_damage(damage);
                self.log.info(LogData::NpcAttackHit { npc_name, damage });
            }
            AttackDegree::CriticalHit(damage) => {
                self.player.character.take_damage(damage);
                self.log.info(LogData::NpcAttackHitCritical { npc_name, damage });
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
    ) -> AttackDegree {
        if self.dodge_roll(defender_dodge_chance) {
            return AttackDegree::Miss;
        }

        let is_critical_strike = self.is_critical_strike(attacker_crit_chance);

        if is_critical_strike {
            let damage_unmitigated = 2 * attacker_damage;
            let damage_mitigated = damage_unmitigated.saturating_sub(defender_mitigation);

            AttackDegree::CriticalHit(damage_mitigated)
        } else {
            let damage_mitigated = attacker_damage.saturating_sub(defender_mitigation);

            AttackDegree::Hit(damage_mitigated)
        }
    }

    /// Retrieves the player's weapon stats in a tuple.
    ///
    /// # Errors
    /// * [EngineError::UnregisteredItem] if the Player's weapon is not registered.
    /// * [DataError::MissingItemDefinition] if the Player's weapon has no definition.
    /// * [EngineError::InvalidItem] if the Player's item equipped as weapon is not a weapon.
    ///
    /// # Returns
    /// A tuple containing the statistics of the weapon
    /// * 0 - Damage (as [Roll])
    /// * 1 - Crit Chance (as [u8])
    /// * 2 - Range of the attack (as [AttackRange])
    fn get_player_weapon_stats(&self) -> Result<(Roll, u8, AttackRange), GameError> {
        if let Some(weapon) = &self.player.character.weapon {
            let item =
                self.get_item_by_id(weapon.0).ok_or(EngineError::UnregisteredItem(weapon.0))?;
            let item_def = self
                .get_item_def_by_id(&item.def_id)
                .ok_or(DataError::MissingItemDefinition(item.def_id))?;

            match item_def.kind {
                GameItemKindDef::Weapon { damage, crit_chance, range } => {
                    Ok((damage, crit_chance, range))
                }
                _ => Err(GameError::from(EngineError::InvalidItem(item_def.kind))),
            }
        } else {
            Ok((Roll::new(1, DieSize::D4), 5, None)) // If no weapon is equipped, fist damage is just 1d4.
        }
    }

    /// Retrieves the player's armor's mitigation statistic.
    ///
    /// # Errors
    /// * [EngineError::UnregisteredItem] if the Player's weapon is not registered.
    /// * [DataError::MissingItemDefinition] if the Player's weapon has no definition.
    /// * [EngineError::InvalidItem] if the Player's item equipped as weapon is not a weapon.
    ///
    /// # Returns
    /// A tuple containing the statistics of the weapon
    /// * 0 - Damage (as [Roll])
    /// * 1 - Crit Chance (as [u8])
    /// * 2 - Range of the attack (as [AttackRange])
    fn get_player_armor_mitigation(&self) -> Result<u16, GameError> {
        if let Some(armor) = &self.player.character.armor {
            let item =
                self.get_item_by_id(armor.0).ok_or(EngineError::UnregisteredItem(armor.0))?;
            let item_def = self
                .get_item_def_by_id(&item.def_id)
                .ok_or(DataError::MissingItemDefinition(item.def_id))?;

            match item_def.kind {
                GameItemKindDef::Armor { mitigation } => Ok(mitigation),
                _ => Err(GameError::from(EngineError::InvalidItem(item_def.kind))),
            }
        } else {
            Ok(0)
        }
    }
}
