#![allow(dead_code)]

use std::collections::HashMap;

use ratatui::style::Style;

use crate::ai::npc_ai::NpcAiState;
use crate::core::game::GameState;
use crate::core::game_items::GameItemSprite;
use crate::core::text_log::LogData;
use crate::data::npc_defs::{NpcDef, NpcDefId, npc_defs};
use crate::util::errors_results::{
    DataError, EngineError, FailReason, GameError, GameOutcome, GameResult,
};
use crate::world::coordinate_system::Point;
use crate::world::tiles::{Collision, Drawable};

impl GameState {
    /// Creates and registers a new entity of type `T`.
    ///
    /// Fails if the target position is unavailable. On success, constructs the
    /// entity, stores it, updates its index, and returns its [`EntityId`].
    pub fn spawn<T: Spawnable + Entity>(
        &mut self,
        name: String,
        pos: Point,
        glyph: char,
        style: Style,
        extra: T::Extra,
    ) -> Result<EntityId, GameError> {
        if !self.is_available(pos) {
            let err = GameError::from(EngineError::SpawningError(pos));
            self.log.info(LogData::Debug(format!("Not able to spawn {}: {}", name, err)));
            return Err(err);
        }

        let id = self.next_entity_id();
        let entity = T::new(id, name, pos, glyph, style, extra);

        self.log.info(LogData::Debug(format!(
            "Spawned {} (ID: {}) at position ({}, {})",
            entity.name(),
            entity.id(),
            entity.pos().x,
            entity.pos().y,
        )));

        T::storage_mut(self).push(entity);
        let index = T::storage_mut(self).len() - 1;
        T::index_mut(self).insert(id, index);

        Ok(id)
    }

    pub fn spawn_npc(&mut self, npc_def_id: NpcDefId, pos: Point) -> Result<EntityId, GameError> {
        self.log.info(LogData::Debug(format!("Trying to spawn {}", npc_def_id)));
        let npc_def = self
            .get_npc_def_by_id(npc_def_id.clone())
            .ok_or(DataError::MissingNpcDefinition(npc_def_id))?;
        self.spawn::<Npc>(npc_def.name.to_owned(), pos, npc_def.glyph, npc_def.style, npc_def.stats)
    }

    pub fn next_entity_id(&mut self) -> EntityId {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;

        id
    }

    /// Removes an entity from the world if it exists.
    ///
    /// Looks up the ID in NPCs first, then item sprites. Uses `swap_remove`
    /// and fixes the moved entity’s index if needed.
    pub fn despawn(&mut self, id: EntityId) {
        if let Some(&index) = self.npc_index.get(&id) {
            self.npcs.swap_remove(index);

            if let Some(moved) = self.npcs.get(index) {
                self.npc_index.insert(moved.id(), index);
            }

            self.npc_index.remove(&id);
            return;
        }

        if let Some(&index) = self.item_sprites_index.get(&id) {
            self.item_sprites.swap_remove(index);

            if let Some(moved) = self.item_sprites.get(index) {
                self.item_sprites_index.insert(moved.id(), index);
            }

            self.item_sprites_index.remove(&id);
        }
    }

    pub fn get_entity_at(&self, pos: Point) -> Option<EntityId> {
        for npc in &self.npcs {
            if npc.pos() == pos {
                return Some(npc.id());
            }
        }

        for item in &self.item_sprites {
            if item.pos() == pos {
                return Some(item.id());
            }
        }

        None
    }

    pub fn get_npc_def_by_id(&self, npc_def_id: NpcDefId) -> Option<NpcDef> {
        npc_defs().get(&npc_def_id).cloned()
    }

    pub fn get_npc(&self, id: EntityId) -> Option<&Npc> {
        self.npc_index.get(&id).map(|&index| &self.npcs[index])
    }

    pub fn get_npc_mut(&mut self, id: EntityId) -> Option<&mut Npc> {
        self.npc_index.get(&id).map(|&index| &mut self.npcs[index])
    }

    pub fn get_item_sprite(&self, id: EntityId) -> Option<&GameItemSprite> {
        self.item_sprites_index.get(&id).map(|&index| &self.item_sprites[index])
    }

    pub fn get_item_sprite_mut(&mut self, id: EntityId) -> Option<&mut GameItemSprite> {
        self.item_sprites_index.get(&id).map(|&index| &mut self.item_sprites[index])
    }

    pub fn move_npc(&mut self, npc_id: EntityId, dx: isize, dy: isize) -> GameResult {
        let (new_x, new_y) = {
            let npc = self.get_npc(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;

            let new_x = npc.pos().x as isize + dx;
            let new_y = npc.pos().y as isize + dy;
            let new_point = Point::new(new_x as usize, new_y as usize);

            if !self.world.is_in_bounds(new_x, new_y) {
                return Ok(GameOutcome::Fail(FailReason::PointOutOfBounds(new_point)));
            }

            if !self.world.get_tile(new_point).tile_type.is_walkable() {
                return Ok(GameOutcome::Fail(FailReason::TileNotWalkable(new_point)));
            }

            (new_x, new_y)
        };

        let npc = self.get_npc_mut(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;
        npc.move_to(Point::new(new_x as usize, new_y as usize));

        Ok(GameOutcome::Success)
    }
}

pub trait Spawnable {
    type Extra;

    fn new(
        id: EntityId,
        name: String,
        pos: Point,
        glyph: char,
        style: Style,
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
    fn pos(&self) -> Point;
}

pub trait Movable {
    fn move_to(&mut self, point: Point);
}

pub type EntityId = u32;

#[derive(Clone)]
pub struct EntityBase {
    pub id: EntityId,
    pub name: String,
    pub pos: Point,
    pub glyph: char,
    pub style: Style, // from ratatui
}

impl Drawable for EntityBase {
    fn glyph(&self) -> char {
        self.glyph
    }
    fn style(&self) -> Style {
        self.style
    }
}

#[derive(Clone)]
pub struct BaseStats {
    pub hp_max: u16,
    pub hp_current: u16,
}

// NPC
pub struct Npc {
    pub base: EntityBase,
    pub stats: NpcStats,
    pub ai_state: NpcAiState,
}

impl Entity for Npc {
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

impl Spawnable for Npc {
    type Extra = NpcStats;

    fn new(
        id: EntityId,
        name: String,
        pos: Point,
        glyph: char,
        style: Style,
        stats: NpcStats,
    ) -> Self {
        Npc::new(id, name, pos, glyph, style, stats)
    }

    fn storage_mut(state: &mut GameState) -> &mut Vec<Self> {
        &mut state.npcs
    }

    fn index_mut(state: &mut GameState) -> &mut HashMap<EntityId, usize> {
        &mut state.npc_index
    }
}

impl Movable for Npc {
    fn move_to(&mut self, point: Point) {
        self.base.pos.x = point.x;
        self.base.pos.y = point.y;
    }
}

impl Npc {
    pub fn new(
        id: EntityId,
        name: String,
        pos: Point,
        glyph: char,
        style: Style,
        stats: NpcStats,
    ) -> Self {
        Self {
            base: EntityBase { id, name, pos, glyph, style },
            stats,
            ai_state: NpcAiState::Wandering,
        }
    }
}

#[derive(Clone)]
pub struct NpcStats {
    pub base: BaseStats,
    pub damage: u16,
    pub dodge: u8,
    pub mitigation: u16,
}

impl NpcStats {
    pub fn dodge_chance(&self) -> u8 {
        self.dodge.min(50)
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use crate::{core::entity_logic::NpcStats, world::worldspace::Room};

    use super::*;

    #[test]
    fn test_spawn_npc() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let id = game.spawn_npc("goblin".into(), Point::new(50, 7)).unwrap();

        // Vec contains NPC
        assert_eq!(game.npcs.len(), 1);
        assert_eq!(game.npcs[0].id(), id);

        // HashMap contains correct index
        assert_eq!(game.npc_index.get(&id), Some(&0));
    }

    #[test]
    fn test_get_entity_by_id() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let npc_id = game.spawn_npc("orc".into(), Point { x: 50, y: 7 }).unwrap();

        match game.get_npc(npc_id) {
            Some(npc) => assert_eq!(npc.name(), "Orc"),
            _ => panic!("Expected NPC"),
        }
    }

    #[test]
    fn test_get_entity_by_id_item() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let item_def_id: String = "armor_leather".to_string();
        let item_id = game.register_item(item_def_id);
        let item_sprite_id = game.spawn_item(item_id, Point::new(50, 7)).unwrap();

        match game.get_item_sprite(item_sprite_id) {
            Some(item) => assert_eq!(item.name(), "Leather Armor"),
            _ => panic!("Expected Item"),
        }
    }

    #[test]
    fn test_get_entity_at() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));
        let pos = Point { x: 50, y: 7 };

        let id = game
            .spawn::<Npc>(
                "Skeleton".into(),
                pos,
                's',
                Color::White.into(),
                NpcStats {
                    base: BaseStats { hp_max: 10, hp_current: 10 },
                    damage: 2,
                    dodge: 50,
                    mitigation: 0,
                },
            )
            .unwrap();

        assert_eq!(game.get_entity_at(pos), Some(id));
    }

    #[test]
    fn test_despawn_npc_updates_indices() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let id1 = game.spawn_npc("goblin".into(), Point::new(50, 7)).unwrap();

        let id2 = game.spawn_npc("orc".into(), Point::new(51, 7)).unwrap();

        // Remove the first NPC
        game.despawn(id1);

        // Only one NPC should remain
        assert_eq!(game.npcs.len(), 1);

        // The remaining NPC must now be at index 0
        assert_eq!(game.npc_index.get(&id2), Some(&0));
    }

    #[test]
    fn test_despawn_removes_from_position() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let pos = Point::new(50, 7);

        let id = game.spawn_npc("goblin".into(), pos).unwrap();

        assert_eq!(game.get_entity_at(pos), Some(id));

        game.despawn(id);

        assert_eq!(game.get_entity_at(pos), None);
    }

    #[test]
    fn test_get_entity_by_id_missing() {
        let game = GameState::default();

        let missing = 9999;

        assert!(game.get_npc(missing).is_none());
        assert!(game.get_item_sprite(missing).is_none());
    }

    #[test]
    fn test_multiple_spawns_indices() {
        let mut game = GameState::default();
        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let id1 = game.spawn_npc("goblin".into(), Point::new(50, 7)).unwrap();

        let id2 = game.spawn_npc("orc".into(), Point::new(51, 7)).unwrap();

        assert_eq!(game.npc_index.get(&id1), Some(&0));
        assert_eq!(game.npc_index.get(&id2), Some(&1));
    }
}
