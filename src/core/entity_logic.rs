#![allow(dead_code)]

use ratatui::style::Style;

use crate::ai::npc_ai::NpcAiState;
use crate::core::game::GameState;
use crate::data::npc_defs::{NpcDef, NpcDefId, npc_defs};
use crate::util::errors_results::{
    DataError, EngineError, FailReason, GameError, GameOutcome, GameResult,
};
use crate::world::coordinate_system::Point;
use crate::world::tiles::{Collision, Drawable};

impl GameState {
    /// Creates a new entity of type `Npc`.
    ///
    /// # Returns
    /// The Npcs [EntityId], which can then be used to get access to the newly spawned Npc.
    ///
    /// # Errors
    /// - [DataError::MissingNpcDefinition()] if npc is not defined in game data.
    /// - [EngineError::SpawningError()] if the position is not available.
    pub fn create_npc(&mut self, npc_def_id: NpcDefId, pos: Point) -> Result<Npc, GameError> {
        // Looking if the npc_def exists.
        let npc_def = get_npc_def_by_id(npc_def_id.clone())
            .ok_or(DataError::MissingNpcDefinition(npc_def_id))?;

        // Creating npc and assigning id.
        let entity_id = self.id_system.next_entity_id();
        let npc = Npc::new(
            entity_id,
            npc_def.name.to_string(),
            pos,
            npc_def.glyph,
            npc_def.style,
            npc_def.stats,
        );

        Ok(npc)
    }

    pub fn move_npc(&mut self, npc_id: EntityId, dx: isize, dy: isize) -> GameResult {
        let (new_x, new_y) = {
            let npc =
                self.current_level().get_npc(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;

            let new_x = npc.pos().x as isize + dx;
            let new_y = npc.pos().y as isize + dy;
            let new_point = Point::new(new_x as usize, new_y as usize);

            if !self.current_world().is_in_bounds(new_x, new_y) {
                return Ok(GameOutcome::Fail(FailReason::PointOutOfBounds(new_point)));
            }

            if !self.current_world().get_tile(new_point).tile_type.is_walkable() {
                return Ok(GameOutcome::Fail(FailReason::TileNotWalkable(new_point)));
            }

            (new_x, new_y)
        };

        let npc =
            self.current_level_mut().get_npc_mut(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;
        npc.move_to(Point::new(new_x as usize, new_y as usize));

        Ok(GameOutcome::Success)
    }
}

pub fn get_npc_def_by_id(npc_def_id: NpcDefId) -> Option<NpcDef> {
    npc_defs().get(&npc_def_id).cloned()
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
#[derive(Clone)]
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
    use crate::world::level::Level;
    use crate::world::worldspace::Room;

    use super::*;

    #[test]
    fn test_spawn_npc() {
        let mut game = GameState::default();
        let mut level: Level = Level::new();
        level.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let npc = game.create_npc("goblin".into(), Point::new(50, 7)).unwrap();
        let npc_id = npc.id();

        let _ = level.spawn_npc(npc);

        game.levels.insert(0, level);

        // Vec contains NPC
        assert_eq!(game.current_level().npcs.len(), 1);
        assert_eq!(game.current_level().npcs[0].id(), npc_id);

        // HashMap contains correct index
        assert_eq!(game.current_level().npc_index.get(&npc_id), Some(&0));
    }

    #[test]
    fn test_get_entity_by_id() {
        let mut game = GameState::default();
        let mut level: Level = Level::new();
        level.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let npc = game.create_npc("orc".into(), Point { x: 50, y: 7 }).unwrap();
        let npc_id = npc.id();

        let _ = level.spawn_npc(npc);

        game.levels.insert(0, level);

        match game.current_level().get_npc(npc_id) {
            Some(npc) => assert_eq!(npc.name(), "Orc"),
            _ => panic!("Expected NPC"),
        }
    }

    #[test]
    fn test_get_entity_by_id_item() {
        let mut game = GameState::default();
        let mut level: Level = Level::new();
        level.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let item_def_id: String = "armor_leather".to_string();
        let item_id = game.register_item(item_def_id);
        let item_sprite = game.create_item_sprite(item_id, Point::new(50, 7)).unwrap();
        let item_sprite_id = item_sprite.id();

        let _ = level.spawn_item_sprite(item_sprite);

        game.levels.insert(0, level);

        match game.current_level().get_item_sprite(item_sprite_id) {
            Some(item) => assert_eq!(item.name(), "Leather Armor"),
            _ => panic!("Expected Item"),
        }
    }

    #[test]
    fn test_get_entity_at() {
        let mut game = GameState::default();
        let mut level: Level = Level::new();
        level.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let pos = Point { x: 50, y: 7 };

        let npc = game.create_npc("orc".into(), Point { x: 50, y: 7 }).unwrap();
        let npc_id = npc.id();

        game.levels.insert(0, level);

        assert_eq!(game.current_level().get_entity_at(pos), Some(npc_id));
    }

    #[test]
    fn test_despawn_npc_updates_indices() {
        let mut game = GameState::default();
        let mut level: Level = Level::new();
        level.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let npc1 = game.create_npc("goblin".into(), Point::new(50, 7)).unwrap();
        let npc1_id = npc1.id();
        let _ = level.spawn_npc(npc1);

        let npc2 = game.create_npc("orc".into(), Point::new(51, 7)).unwrap();
        let npc2_id = npc2.id();
        let _ = level.spawn_npc(npc2);

        game.levels.insert(0, level);

        // Remove the first NPC
        game.current_level_mut().despawn(npc1_id);

        // Only one NPC should remain
        assert_eq!(game.current_level().npcs.len(), 1);

        // The remaining NPC must now be at index 0
        assert_eq!(game.current_level().npc_index.get(&npc2_id), Some(&0));
    }

    #[test]
    fn test_despawn_removes_from_position() {
        let mut game = GameState::default();
        let mut level: Level = Level::new();
        level.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let pos = Point::new(50, 7);

        let npc = game.create_npc("goblin".into(), pos).unwrap();
        let npc_id = npc.id();

        assert_eq!(game.current_level().get_entity_at(pos), Some(npc_id));

        game.current_level_mut().despawn(npc_id);

        assert_eq!(game.current_level().get_entity_at(pos), None);
    }

    #[test]
    fn test_get_entity_by_id_missing() {
        let mut game = GameState::default();
        let level: Level = Level::new();

        game.levels.insert(0, level);

        let missing = 9999;

        assert!(game.current_level().get_npc(missing).is_none());
        assert!(game.current_level().get_item_sprite(missing).is_none());
    }

    #[test]
    fn test_multiple_spawns_indices() {
        let mut game = GameState::default();
        let mut level: Level = Level::new();
        level.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        let npc1 = game.create_npc("goblin".into(), Point::new(50, 7)).unwrap();
        let npc1_id = npc1.id();
        let _ = level.spawn_npc(npc1);

        let npc2 = game.create_npc("orc".into(), Point::new(51, 7)).unwrap();
        let npc2_id = npc2.id();
        let _ = level.spawn_npc(npc2);

        game.levels.insert(0, level);

        assert_eq!(game.current_level().npc_index.get(&npc1_id), Some(&0));
        assert_eq!(game.current_level().npc_index.get(&npc2_id), Some(&1));
    }
}
