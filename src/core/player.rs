#![allow(dead_code)]

use crate::core::game::{BaseStats, Entity, EntityBase, EntityId, GameItem};
use crate::world::worldspace::Point;
use crate::world::worldspace::World;
use ratatui::style::Color;

pub struct Player {
    pub name: String,
    pub character: PlayerCharacter,
}

impl Player {
    pub fn new(id: EntityId) -> Self {
        Self { name: "Hero".to_string(), character: PlayerCharacter::new(id) }
    }

    // for testing, don't insert default player into the world!
    pub fn default() -> Self {
        Self { name: "Hero".to_string(), character: PlayerCharacter::default() }
    }
}

pub struct PlayerCharacter {
    pub base: EntityBase,
    pub stats: PcStats,
    pub inventory: Vec<GameItem>,
}

impl PlayerCharacter {
    pub fn new(id: EntityId) -> Self {
        Self {
            base: EntityBase {
                id,
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

    pub fn move_by(&mut self, dx: i32, dy: i32, world: &World) {
        let new_x = self.base.pos.x as isize + dx as isize;
        let new_y = self.base.pos.y as isize + dy as isize;

        if world.is_in_bounds(new_x, new_y) {
            self.base.pos.x = new_x as usize;
            self.base.pos.y = new_y as usize;
        }
    }
}

impl Default for PlayerCharacter {
    fn default() -> Self {
        Self::new(999999) // placeholder, never inserted inro world
    }
}

pub struct PcStats {
    pub base: BaseStats,
    pub strength: u8,
    pub dexterity: u8,
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
