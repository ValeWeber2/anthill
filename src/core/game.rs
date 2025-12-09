#![allow(dead_code)]

use crate::core::player::Player;
use crate::world::worldspace::{Drawable, Point, World};
use ratatui::style::Color;

pub trait Entity {
    fn id(&self) -> EntityId;
    fn pos(&self) -> &Point;
}

// ----------------------------------------------
//                Game State Struct
// ----------------------------------------------
pub struct GameState {
    pub world: World,
    pub player: Player,
    pub log: Log,
    pub round_nr: u64,
}

impl GameState {
    pub fn new() -> Self {
        Self { world: World::new(), player: Player::new(), log: Log::new(), round_nr: 0 }
    }
}


// ----------------------------------------------
//                  Game Text Log
// ----------------------------------------------
pub struct Log {
    pub messages: Vec<String>,
    pub scroll: u16,
}

impl Log {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            scroll: 0,
        }
    }
}

// ----------------------------------------------
//                 Game Entities
//   Things that are sprites on the world map.
// ----------------------------------------------
pub type EntityId = u32;

pub struct EntityBase {
    pub id: EntityId,
    pub name: String,
    pub pos: Point,
    pub glyph: char,
    pub color: Color, // from ratatui
}

impl Drawable for EntityBase {
    fn glyph(&self) -> char {
        self.glyph
    }
    fn color(&self) -> Color {
        self.color
    }
}

pub struct BaseStats {
    pub hp_max: u32,
    pub hp_current: u32,
}

// NPC
pub struct Npc {
    pub base: EntityBase,
    pub stats: NpcStats,
}

pub struct NpcStats {
    base: BaseStats,
    damage: u8,
}

impl Entity for Npc {
    fn id(&self) -> EntityId {
        self.base.id
    }
    fn pos(&self) -> &Point {
        &self.base.pos
    }
}

// Item Sprite
pub struct ItemSprite {
    pub base: EntityBase,
    pub item_type: GameItem,
}

pub enum GameItem {
    Weapon { name: String, damage: u32 },
    Key { name: String },
}

impl Entity for ItemSprite {
    fn id(&self) -> EntityId {
        self.base.id
    }
    fn pos(&self) -> &Point {
        &self.base.pos
    }
}
