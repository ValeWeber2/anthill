#![allow(dead_code)]

use crate::core::game::{BaseStats, Entity, EntityBase, EntityId, GameItem};
use crate::world::worldspace::Point;

pub struct Player {
    pub name: String,
    pub character: PlayerCharacter,
}

pub struct PlayerCharacter {
    pub base: EntityBase,
    pub stats: PcStats,
    pub inventory: Vec<GameItem>,
}

pub struct PcStats {
    base: BaseStats,
    strength: u8,
    dexterity: u8,
}

impl Entity for PlayerCharacter {
    fn id(&self) -> EntityId {
        self.base.id
    }
    fn pos(&self) -> &Point {
        &self.base.pos
    }
}
