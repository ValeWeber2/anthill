#![allow(dead_code)]

use crate::core::game::{BaseStats, Entity, EntityBase, EntityId, GameItem};
use crate::world::worldspace::Point;
use ratatui::style::Color;

pub struct Player {
    pub name: String,
    pub character: PlayerCharacter,
}

impl Player {
    pub fn new() -> Self {
        Self { name: "Hero".to_string(), character: PlayerCharacter::new() }
    }
}

pub struct PlayerCharacter {
    pub base: EntityBase,
    pub stats: PcStats,
    pub inventory: Vec<GameItem>,
}

impl PlayerCharacter {
    pub fn new() -> Self {
        Self {
            base: EntityBase {
                id: 0,
                name: "Hero".to_string(),
                pos: Point::new(0, 0),
                glyph: '@',
                color: Color::Yellow,
            },
            stats: PcStats {
                base: BaseStats { hp_max: 100, hp_current: 100 },
                strength: 10,
                dexterity: 10,
            },
            inventory: Vec::new(),
        }
    }
}

pub struct PcStats {
    pub base: BaseStats,
    pub strength: u8,
    pub dexterity: u8,
}

impl Entity for PlayerCharacter {
    fn id(&self) -> EntityId {
        self.base.id
    }
    fn pos(&self) -> &Point {
        &self.base.pos
    }
}
