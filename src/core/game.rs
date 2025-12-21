#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::u32;

use crate::core::player::Player;
use crate::world::worldspace::{Drawable, Point, World};
use ratatui::style::Color;

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

    // placeholder, only for tests
    pub fn default() -> Self {
        Self {
            world: World::default(),
            player: Player::default(),
            log: Log::new(),
            round_nr: 0,
            id_counter: 0,
        }
    }

    pub fn spawn<T: Spawnable + Entity>(
        &mut self,
        name: String,
        pos: Point,
        glyph: char,
        color: Color,
        extra: T::Extra,
    ) -> Result<EntityId, SpawningError> {
        if !self.world.is_available(pos) {
            let err = SpawningError::PositionUnavailable { x: pos.x, y: pos.y };
            self.log.messages.push(format!("Not able to spawn {}: {}", name, err));
            return Err(err);
        }

        let id = self.next_id();
        let entity = T::new(id, name, pos, glyph, color, extra);

        self.log.messages.push(format!(
            "Spawned {} (ID: {}) at position ({}, {})",
            entity.name(),
            entity.id(),
            entity.pos().x,
            entity.pos().y,
        ));

        T::storage_mut(self).push(entity);
        let index = T::storage_mut(self).len() - 1;
        T::index_mut(self).insert(id, index);

        Ok(id)
    }

    pub fn spawn_npc(
        &mut self,
        name: String,
        pos: Point,
        glyph: char,
        color: Color,
        stats: NpcStats,
    ) -> Result<EntityId, SpawningError> {
        self.spawn::<Npc>(name, pos, glyph, color, stats)
    }

    pub fn spawn_item(
        &mut self,
        name: String,
        pos: Point,
        glyph: char,
        color: Color,
        item_type: GameItem,
    ) -> Result<EntityId, SpawningError> {
        self.spawn::<ItemSprite>(name, pos, glyph, color, item_type)
    }

    pub fn next_id(&mut self) -> EntityId {
        let id = self.id_counter;
        self.id_counter += 1;

        id
    }

    pub fn despawn(&mut self, id: EntityId) {
        if let Some(&index) = self.world.npc_index.get(&id) {
            self.world.npcs.swap_remove(index);

            if let Some(moved) = self.world.npcs.get(index) {
                self.world.npc_index.insert(moved.id(), index);
            }

            self.world.npc_index.remove(&id);
            return;
        }

        if let Some(&index) = self.world.item_index.get(&id) {
            self.world.items.swap_remove(index);

            if let Some(moved) = self.world.items.get(index) {
                self.world.item_index.insert(moved.id(), index);
            }

            self.world.item_index.remove(&id);
            return;
        }
    }

    pub fn get_entity_by_id(&self, id: EntityId) -> Option<EntityRef> {
        if let Some(&i) = self.world.npc_index.get(&id) {
            return Some(EntityRef::Npc(&self.world.npcs[i]));
        }

        if let Some(&i) = self.world.item_index.get(&id) {
            return Some(EntityRef::Item(&self.world.items[i]));
        }

        None
    }

    pub fn get_entity_at(&self, pos: Point) -> Option<EntityId> {
        for npc in &self.world.npcs {
            if *npc.pos() == pos {
                return Some(npc.id());
            }
        }

        for item in &self.world.items {
            if *item.pos() == pos {
                return Some(item.id());
            }
        }

        None
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

pub enum EntityRef<'a> {
    Npc(&'a Npc),
    Item(&'a ItemSprite),
}

pub trait Spawnable {
    type Extra;

    fn new(
        id: EntityId,
        name: String,
        pos: Point,
        glyph: char,
        color: Color,
        extra: Self::Extra,
    ) -> Self;

    fn storage_mut(state: &mut GameState) -> &mut Vec<Self>
    where
        Self: Sized;
    fn index_mut(state: &mut GameState) -> &mut HashMap<EntityId, usize>;
}
pub trait Entity {
    fn name(&self) -> &str;
    fn id(&self) -> EntityId;
    fn pos(&self) -> &Point;
}

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

impl Spawnable for Npc {
    type Extra = NpcStats;

    fn new(
        id: EntityId,
        name: String,
        pos: Point,
        glyph: char,
        color: Color,
        stats: NpcStats,
    ) -> Self {
        Npc::new(id, name, pos, glyph, color, stats)
    }

    fn storage_mut(state: &mut GameState) -> &mut Vec<Self> {
        &mut state.world.npcs
    }

    fn index_mut(state: &mut GameState) -> &mut HashMap<EntityId, usize> {
        &mut state.world.npc_index
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

impl Spawnable for ItemSprite {
    type Extra = GameItem;

    fn new(
        id: EntityId,
        name: String,
        pos: Point,
        glyph: char,
        color: Color,
        item: GameItem,
    ) -> Self {
        ItemSprite::new(id, name, pos, glyph, color, item)
    }

    fn storage_mut(state: &mut GameState) -> &mut Vec<Self> {
        &mut state.world.items
    }

    fn index_mut(state: &mut GameState) -> &mut HashMap<EntityId, usize> {
        &mut state.world.item_index
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

#[derive(Debug, Clone)]
pub enum SpawningError {
    PositionUnavailable { x: usize, y: usize },
}

impl Display for SpawningError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SpawningError::PositionUnavailable { x, y } => {
                write!(f, "Position ({}, {}) is not available", x, y)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::world::worldspace::Room;

    use super::*;

    #[test]
    fn test_spawn_npc() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let id = game.spawn_npc(
                    format!("Goblin"),
                    Point::new(50, 7),
                    'g',
                    Color::Green,
                    NpcStats { base: BaseStats { hp_max: 10, hp_current: 10 }, damage: 2 }
        ).unwrap();

        // Vec contains NPC
        assert_eq!(game.world.npcs.len(), 1);
        assert_eq!(game.world.npcs[0].id(), id);

        // HashMap contains correct index
        assert_eq!(game.world.npc_index.get(&id), Some(&0));
    }

    #[test]
    fn test_get_entity_by_id() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let npc_id = game.spawn_npc(
            "Orc".into(),
            Point { x: 50, y: 7 },
            'o',
            Color::LightGreen,
            NpcStats { base: BaseStats { hp_max: 10, hp_current: 10 }, damage: 2 },
        ).unwrap();

        match game.get_entity_by_id(npc_id) {
            Some(EntityRef::Npc(npc)) => assert_eq!(npc.name(), "Orc"),
            _ => panic!("Expected NPC"),
        }
    }

    #[test]
    fn test_get_entity_by_id_item() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let item_id = game.spawn_item(
            "Key".into(),
            Point::new(50, 7),
            '?',
            Color::White,
            GameItem::Key { name: "Key".into() }
        ).unwrap();

        match game.get_entity_by_id(item_id) {
            Some(EntityRef::Item(item)) => assert_eq!(item.name(), "Key"),
            _ => panic!("Expected Item"),
        }
    }

    #[test]
    fn test_get_entity_at() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));
        let pos = Point { x: 50, y: 7 };

        let id = game.spawn::<Npc>(
            "Skeleton".into(),
            pos,
            's',
            Color::White,
            NpcStats { base: BaseStats { hp_max: 10, hp_current: 10 }, damage: 2 },
        ).unwrap();

        assert_eq!(game.get_entity_at(pos), Some(id));
    }

    #[test]
    fn test_despawn_npc_updates_indices() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let id1 = game.spawn_npc(
            "A".into(),
            Point::new(50, 7),
            'a',
            Color::White,
            NpcStats { base: BaseStats { hp_max: 10, hp_current: 10 }, damage: 1 },
        ).unwrap();

        let id2 = game.spawn_npc(
            "B".into(),
            Point::new(51, 7),
            'b',
            Color::White,
            NpcStats { base: BaseStats { hp_max: 10, hp_current: 10 }, damage: 1 },
        ).unwrap();

        // Remove the first NPC
        game.despawn(id1);

        // Only one NPC should remain
        assert_eq!(game.world.npcs.len(), 1);

        // The remaining NPC must now be at index 0
        assert_eq!(game.world.npc_index.get(&id2), Some(&0));
    }

    #[test]
    fn test_despawn_removes_from_position() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let pos = Point::new(50, 7);

        let id = game.spawn_npc(
            "Ghost".into(),
            pos,
            'G',
            Color::Cyan,
            NpcStats { base: BaseStats { hp_max: 10, hp_current: 10 }, damage: 1 },
        ).unwrap();

        assert_eq!(game.get_entity_at(pos), Some(id));

        game.despawn(id);

        assert_eq!(game.get_entity_at(pos), None);
    }

    #[test]
    fn test_get_entity_by_id_missing() {
        let game = GameState::default();

        let missing = 9999;

        assert!(game.get_entity_by_id(missing).is_none());
    }

    #[test]
    fn test_multiple_spawns_indices() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let id1 = game.spawn_npc("A".into(), Point::new(50, 7), 'a', Color::White,
            NpcStats { base: BaseStats { hp_max: 10, hp_current: 10 }, damage: 1 }).unwrap();

        let id2 = game.spawn_npc("B".into(), Point::new(51, 7), 'b', Color::White,
            NpcStats { base: BaseStats { hp_max: 10, hp_current: 10 }, damage: 1 }).unwrap();

        assert_eq!(game.world.npc_index.get(&id1), Some(&0));
        assert_eq!(game.world.npc_index.get(&id2), Some(&1));
    }
}
