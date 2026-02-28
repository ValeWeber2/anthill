use std::collections::HashMap;
use std::fmt;

use crate::core::buff_effects::{ActiveBuff, PotionEffectDef, PotionType, PotionUsage};
use crate::core::entity_logic::{BaseStats, Entity, EntityBase, EntityId, Movable};
use crate::core::game::{GameRules, GameState};
use crate::core::game_items::{ArmorItem, GameItemId, WeaponItem};
use crate::util::text_log::LogData;
use crate::world::coordinate_system::Point;
use ratatui::style::Color;

pub struct Player {
    #[allow(dead_code)]
    pub name: String,
    pub character: PlayerCharacter,
}

impl Player {
    pub fn new(id: EntityId) -> Self {
        Self { name: "Hero".to_string(), character: PlayerCharacter::new(id) }
    }
}

impl Default for Player {
    // for testing, don't insert default player into the world!
    fn default() -> Self {
        Self { name: "Hero".to_string(), character: PlayerCharacter::default() }
    }
}

pub struct PlayerCharacter {
    pub base: EntityBase,
    pub stats: PcStats,
    pub inventory: Vec<GameItemId>,
    pub class: CharacterClass,
    pub armor: Option<ArmorItem>,
    pub weapon: Option<WeaponItem>,
    pub active_buffs: Vec<ActiveBuff>,
    pub potion_usage: HashMap<PotionType, PotionUsage>,
}

impl PlayerCharacter {
    pub fn new(id: EntityId) -> Self {
        Self {
            base: EntityBase {
                id,
                name: "Hero".to_string(),
                pos: Point::new(0, 0),
                glyph: '@',
                style: Color::Yellow.into(),
            },
            stats: PcStats::new(),
            inventory: Vec::new(),
            class: CharacterClass::default(),
            armor: None,
            weapon: None,
            active_buffs: Vec::new(),
            potion_usage: HashMap::new(),
        }
    }
    pub fn attack_damage_bonus_melee(&self) -> i16 {
        let mut bonus: i16 = self.stats.abilities.strength as i16;

        for buff in &self.active_buffs {
            match buff.effect {
                PotionEffectDef::Strength { amount, .. } => {
                    bonus += amount as i16;
                }
                PotionEffectDef::Fatigue { strength_penalty, .. } => {
                    bonus = bonus.saturating_sub(strength_penalty as i16);
                }
                _ => {}
            }
        }
        bonus
    }

    pub fn attack_damage_bonus_ranged(&self) -> i16 {
        let mut bonus: i16 = self.stats.abilities.perception as i16;

        for buff in &self.active_buffs {
            if let PotionEffectDef::Fatigue { strength_penalty, .. } = buff.effect {
                bonus = bonus.saturating_sub(strength_penalty as i16);
            }
        }
        bonus
    }

    pub fn dodge_chance(&self) -> u8 {
        let mut dodge = (5 + self.stats.abilities.dexterity / 2).min(50);

        for buff in &self.active_buffs {
            match buff.effect {
                PotionEffectDef::Dexterity { amount, .. } => {
                    dodge = (dodge + amount).min(100);
                }
                PotionEffectDef::Cramp { dexterity_penalty, .. } => {
                    dodge = dodge.saturating_sub(dexterity_penalty);
                }
                _ => {}
            }
        }
        dodge
    }

    pub fn take_damage(&mut self, amount: u16) {
        self.stats.base.take_damage(amount);
    }

    pub fn heal(&mut self, amount: u16) {
        self.stats.base.heal(amount);
    }

    pub fn is_alive(&self) -> bool {
        self.stats.base.is_alive()
    }

    /// Add experience points to the player's experience counter. If the experience points are
    /// enough to level up, level up the character.
    ///
    /// # Returns
    /// `true` if the experience resulted in a level-up
    /// `false` if the experience didn't result in a level-up
    pub fn gain_experience(&mut self, amount: u32) -> bool {
        self.stats.experience += amount;

        let required_xp = self.stats.level as u32 * 100;
        if self.stats.experience >= required_xp {
            self.stats.experience -= required_xp;
            self.level_up();
            return true;
        }

        false
    }

    fn level_up(&mut self) {
        self.stats.level += 1;
        self.stats.abilities.strength += 1;
        self.stats.abilities.dexterity += 1;
        self.stats.abilities.vitality += 1;
        self.stats.abilities.perception += 1;

        self.stats.base.hp_max += 10;
        self.stats.base.hp_current = self.stats.base.hp_max;
    }

    pub fn tick_buffs(&mut self) {
        let mut damage_accrued: u16 = 0;
        for buff in &mut self.active_buffs {
            if let PotionEffectDef::Poison { damage_per_tick, duration: _ } = &buff.effect {
                damage_accrued += *damage_per_tick;
            }
            if buff.remaining_turns > 0 {
                buff.remaining_turns -= 1;
            }
        }
        self.take_damage(damage_accrued);
        self.active_buffs.retain(|buff| buff.remaining_turns > 0);
    }
}

impl Default for PlayerCharacter {
    fn default() -> Self {
        Self::new(999999) // placeholder, never inserted inro world
    }
}

impl GameState {
    pub fn player_add_experience(&mut self, amount: u32) {
        let did_level_up = self.player.character.gain_experience(amount);
        if did_level_up {
            self.log.info(LogData::LevelUp { new_level: self.player.character.stats.level });
        }
    }

    pub fn player_is_alive(&self) -> bool {
        if self.game_rules.contains(GameRules::GOD_MODE) {
            return true;
        }
        self.player.character.is_alive()
    }
}

pub struct PcStats {
    pub base: BaseStats,
    pub abilities: AbilityScores,
    pub level: u8,
    pub experience: u32,
}

impl PcStats {
    pub fn new() -> Self {
        let vitality = 1;
        let hp_max = 10 + vitality as u16 * 10;

        Self {
            base: BaseStats { hp_max, hp_current: hp_max },
            abilities: AbilityScores::default(),
            level: 1,
            experience: 0,
        }
    }
}

impl From<AbilityScores> for PcStats {
    fn from(value: AbilityScores) -> Self {
        let abilities = value;

        let hp_max = 10 + abilities.vitality as u16 * 10;

        Self { base: BaseStats { hp_max, hp_current: hp_max }, abilities, level: 1, experience: 0 }
    }
}

pub struct AbilityScores {
    pub strength: u8,
    pub dexterity: u8,
    pub vitality: u8,
    pub perception: u8,
}

impl AbilityScores {
    pub fn new(strength: u8, dexterity: u8, vitality: u8, perception: u8) -> Self {
        Self { strength, dexterity, vitality, perception }
    }
}

impl Default for AbilityScores {
    fn default() -> Self {
        Self { strength: 1, dexterity: 1, vitality: 1, perception: 1 }
    }
}

impl Entity for PlayerCharacter {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn id(&self) -> EntityId {
        self.base.id
    }
    fn pos(&self) -> Point {
        self.base.pos
    }
}

impl Movable for PlayerCharacter {
    fn move_to(&mut self, point: Point) {
        self.base.pos.x = point.x;
        self.base.pos.y = point.y;
    }
}

/// Possible playable classes in the game. Determine the stats the player character starts out with.
#[derive(Default, Clone, Copy)]
pub enum CharacterClass {
    #[default]
    Wretch,
    Barbarian,
    Knight,
    Ranger,
}

impl fmt::Display for CharacterClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CharacterClass::Wretch => write!(f, "Wretch"),
            CharacterClass::Barbarian => write!(f, "Barbarian"),
            CharacterClass::Knight => write!(f, "Knight"),
            CharacterClass::Ranger => write!(f, "Ranger"),
        }
    }
}

impl From<&str> for CharacterClass {
    fn from(value: &str) -> Self {
        match value {
            "Barbarian" => CharacterClass::Barbarian,
            "Knight" => CharacterClass::Knight,
            "Ranger" => CharacterClass::Ranger,
            _ => CharacterClass::Wretch,
        }
    }
}

impl From<CharacterClass> for AbilityScores {
    fn from(value: CharacterClass) -> Self {
        match value {
            CharacterClass::Wretch => AbilityScores::new(1, 1, 1, 1),
            CharacterClass::Barbarian => AbilityScores::new(3, 2, 1, 1),
            CharacterClass::Knight => AbilityScores::new(2, 1, 3, 1),
            CharacterClass::Ranger => AbilityScores::new(1, 1, 2, 3),
        }
    }
}

impl PlayerCharacter {
    /// Applies the chosen class's ability scores to the player character, calculating derived stats (like hit points).
    pub fn apply_class(&mut self, class: CharacterClass) {
        let abilities = AbilityScores::from(class);
        self.stats = PcStats::from(abilities);
        self.class = class;
    }
}
