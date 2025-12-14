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
    pub id_counter: u32,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            world: World::default(),
            player: Player::new(0),
            log: Log::new(),
            round_nr: 0,
            id_counter: 0,
        };

        let player_id = state.next_id();
        state.player = Player::new(player_id);
        state.world = World::new(&mut state);

        state
    }

    pub fn spawn_npc(
        &mut self,
        name: String,
        pos: Point,
        glyph: char,
        color: Color,
        stats: NpcStats,
    ) -> Result<(), ()> {
        if self.world.is_taken(pos) {
            self.log.messages.push(("This position is already taken").into());
            return Err(());
        }

        let id = self.next_id();
        let npc = Npc::new(id, name, pos, glyph, color, stats);

        self.log.messages.push(format!(
            "Spawned NPC at position x: {}, y: {} with ID: {}",
            npc.base.pos.x,
            npc.base.pos.y,
            npc.id()
        ));

        self.world.npcs.push(npc);

        Ok(())
    }

    pub fn spawn_item(
        &mut self,
        name: String,
        pos: Point,
        glyph: char,
        color: Color,
        item_type: GameItem,
    ) -> Result<(), ()> {
        if self.world.is_taken(pos) {
            self.log.messages.push(format!(
                "Not able to spawn {}: Position x: {}, y: {} is already taken",
                name, pos.x, pos.y
            ));
            return Err(());
        }

        let id = self.next_id();
        let item = ItemSprite::new(id, name, pos, glyph, color, item_type);

        self.log.messages.push(format!(
            "Spawned item at position x: {}, y: {} with ID: {}",
            item.base.pos.x,
            item.base.pos.y,
            item.id()
        ));

        self.world.items.push(item);

        Ok(())
    }

    pub fn next_id(&mut self) -> EntityId {
        let id = self.id_counter;
        self.id_counter += 1;

        id
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
        Self { messages: Vec::new(), scroll: 0 }
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
    pub base: BaseStats,
    pub damage: u8,
}

impl Entity for Npc {
    fn id(&self) -> EntityId {
        self.base.id
    }
    fn pos(&self) -> &Point {
        &self.base.pos
    }
}

impl Npc {
    pub fn new(
        id: EntityId,
        name: String,
        pos: Point,
        glyph: char,
        color: Color,
        stats: NpcStats,
    ) -> Self {
        Self { base: EntityBase { id, name, pos, glyph, color }, stats }
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

impl ItemSprite {
    pub fn new(
        id: EntityId,
        name: String,
        pos: Point,
        glyph: char,
        color: Color,
        item_type: GameItem,
    ) -> Self {
        Self { base: EntityBase { id, name, pos, glyph, color }, item_type }
    }
}
