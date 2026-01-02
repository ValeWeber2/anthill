#![allow(dead_code)]

use crate::core::entity_logic::{BaseStats, Entity, EntityBase, EntityId, Movable};
use crate::core::game_items::{ArmorItem, GameItemId, WeaponItem};
use crate::world::worldspace::Point;
use ratatui::style::Color;

pub struct Player {
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
    pub armor: Option<ArmorItem>,
    pub weapon: Option<WeaponItem>,
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
            armor: None,
            weapon: None,
        }
    }
    pub fn attack_damage(&self) -> u32 {
        self.stats.strength as u32 * 2
    }

    pub fn dodge_chance(&self) -> u8 {
        (self.stats.dexterity / 2).min(50)
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.stats.base.take_damage(amount);
    }

    pub fn heal(&mut self, amount: u32) {
        self.stats.base.heal(amount);
    }

    pub fn is_alive(&self) -> bool {
        self.stats.base.is_alive()
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.stats.experience += amount;

        let required_xp = self.stats.level as u32 * 100;
        if self.stats.experience >= required_xp {
            self.stats.experience -= required_xp;
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.stats.level += 1;
        self.stats.strength += 1;
        self.stats.dexterity += 1;
        self.stats.vitality += 1;

        self.stats.base.hp_max += 10;
        self.stats.base.hp_current = self.stats.base.hp_max;
    }
}

impl Default for PlayerCharacter {
    fn default() -> Self {
        Self::new(999999) // placeholder, never inserted into world
    }
}

pub struct PcStats {
    pub base: BaseStats,
    pub strength: u8,
    pub dexterity: u8,
    pub vitality: u8,
    pub perception: u8,
    pub level: u8,
    pub experience: u32,
}

impl PcStats {
    pub fn new() -> Self {
        let vitality = 10;
        let hp_max = vitality as u32 * 10;

        Self {
            base: BaseStats { hp_max, hp_current: hp_max },
            strength: 10,
            dexterity: 10,
            vitality,
            perception: 10,
            level: 1,
            experience: 0,
        }
    }
}

impl BaseStats {
    pub fn take_damage(&mut self, amount: u32) {
        if amount >= self.hp_current {
            self.hp_current = 0;
        } else {
            self.hp_current -= amount;
        }
    }

    pub fn heal(&mut self, amount: u32) {
        self.hp_current = (self.hp_current + amount).min(self.hp_max);
    }

    pub fn is_alive(&self) -> bool {
        self.hp_current > 0
    }
}

impl Entity for PlayerCharacter {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn id(&self) -> EntityId {
        self.base.id
    }
    fn pos(&self) -> &Point {
        &self.base.pos
    }
}

impl Movable for PlayerCharacter {
    fn move_to(&mut self, point: Point) {
        self.base.pos.x = point.x;
        self.base.pos.y = point.y;
    }
}
