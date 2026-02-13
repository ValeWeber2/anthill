#![allow(dead_code)]

use std::collections::HashMap;

use rand::RngCore;

use crate::core::entity_logic::{Entity, Npc};
use crate::core::game_items::GameItemSprite;
use crate::data::levels::level_paths;
use crate::proc_gen::proc_gen_level::ProcGenLevel;
use crate::util::errors_results::{DataError, EngineError};
use crate::world::coordinate_system::Point;
use crate::world::tiles::Collision;
use crate::world::world_data::{LevelData, SpawnKind};
use crate::world::world_loader::load_world_from_ron;
use crate::{
    core::{entity_logic::EntityId, game::GameState},
    util::errors_results::GameError,
    world::worldspace::World,
};

// /// The game has procedurally generated levels, but also at some fixed points, there are handmade pre-defined levels.
// /// This constant defines at what interval these levels are supposed to appear.
// ///
// /// Example with default interval of `4`: Static level appears at levels 2, 6, 10, 14, 18, ...
// const STATIC_LEVEL_INTERVAL: usize = 4;

pub struct Level {
    pub world: World,

    pub entry: Point,
    pub exit: Point,

    pub npcs: Vec<Npc>,
    pub npc_index: HashMap<EntityId, usize>,

    pub item_sprites: Vec<GameItemSprite>,
    pub item_sprites_index: HashMap<EntityId, usize>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            world: World::new(),

            entry: Point::default(),
            exit: Point::default(),

            npcs: Vec::new(),
            npc_index: HashMap::new(),

            item_sprites: Vec::new(),
            item_sprites_index: HashMap::new(),
        }
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

    pub fn is_available(&self, pos: Point) -> bool {
        self.world.is_in_bounds(pos.x as isize, pos.y as isize)
            && self.npcs.iter().all(|npc| npc.base.pos != pos)
            && self.item_sprites.iter().all(|item| item.base.pos != pos)
            && self.world.get_tile(pos).tile_type.is_walkable()
    }

    /// Spawns an NPC on the map.
    ///
    /// The function checks whether the target position is free.  
    /// If the position is unavailable, a `GameError::SpawningError` is returned.
    /// On success, the NPC is added to the internal list and its ID is indexed.
    pub fn spawn_npc(&mut self, npc: Npc) -> Result<(), GameError> {
        if !self.is_available(npc.pos()) {
            let err = GameError::from(EngineError::SpawningError(npc.pos()));
            return Err(err);
        }

        let npc_id = npc.id();

        self.npcs.push(npc);
        let index = self.npcs.len() - 1;
        self.npc_index.insert(npc_id, index);

        Ok(())
    }

    /// Spawns an item sprite on the map.
    pub fn spawn_item_sprite(&mut self, item_sprite: GameItemSprite) -> Result<(), GameError> {
        if !self.is_available(item_sprite.pos()) {
            let err = GameError::from(EngineError::SpawningError(item_sprite.pos()));
            return Err(err);
        }

        let item_sprite_id = item_sprite.id();

        self.item_sprites.push(item_sprite);
        let index = self.item_sprites.len() - 1;
        self.item_sprites_index.insert(item_sprite_id, index);

        Ok(())
    }

    /// Removes an entity from the level if it exists.
    ///
    /// Looks up the ID in NPCs and item sprites. Uses `swap_remove`
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
}

/// All possibilities where a level can be entered. Used in [GameState::goto_level].
/// Can be extended in the future with `Custom(Point)` or `Random` in cases like traps, where you fall through the floor.
pub enum LevelEntrance {
    Entry,
    Exit,
}

impl GameState {
    /// Getter for the level that is currently active in the game.
    pub fn current_level(&self) -> &Level {
        &self.levels[self.level_nr]
    }

    /// Mutable getter for the level that is currently active in the game.
    pub fn current_level_mut(&mut self) -> &mut Level {
        &mut self.levels[self.level_nr]
    }

    /// Getter for the world of the level that is currently active in the game.
    pub fn current_world(&self) -> &World {
        &self.current_level().world
    }

    /// Mutable getter for the world of the level that is currently active in the game.
    pub fn current_world_mut(&mut self) -> &mut World {
        &mut self.current_level_mut().world
    }

    pub fn goto_level(
        &mut self,
        index: usize,
        entrance_point: LevelEntrance,
    ) -> Result<(), GameError> {
        match self.levels.get(index) {
            Some(_) => self.level_nr = index,
            None => {
                self.initialize_level(index)?;
                self.level_nr = index;
            }
        }

        self.player.character.base.pos = match entrance_point {
            LevelEntrance::Entry => self.current_level().entry,
            LevelEntrance::Exit => self.current_level().exit,
        };

        self.compute_fov();

        Ok(())
    }

    pub fn goto_level_next(&mut self) -> Result<(), GameError> {
        self.goto_level(self.level_nr + 1, LevelEntrance::Entry)
    }

    pub fn goto_level_previous(&mut self) -> Result<(), GameError> {
        self.goto_level(self.level_nr - 1, LevelEntrance::Exit)
    }

    pub fn initialize_level(&mut self, index: usize) -> Result<(), GameError> {
        // If current level is a static level
        // if index % STATIC_LEVEL_INTERVAL == 2 {
        let new_level: Level = if index == 0 || index == 1 {
            self.load_static_level(index).map_err(|error| {
                self.log.debug_warn(format!("Couldn't load level {}", error));
                error
            })?
        } else {
            self.load_generated_level(index).unwrap()
        };

        self.levels.insert(index, new_level);

        Ok(())
    }

    pub fn load_static_level(&mut self, level_nr: usize) -> Result<Level, GameError> {
        if level_nr > level_paths().len() {
            return Err(GameError::from(DataError::StaticWorldNotFound(level_nr)));
        }

        let data = load_world_from_ron(level_paths()[level_nr])?;

        let mut level = Level::new();

        level.world.apply_world_data(&data, level_nr)?;
        level.entry = data.entry;
        level.exit = data.exit;

        for spawn in &data.spawns {
            let pos = Point::new(spawn.x, spawn.y);

            if !level.is_available(pos) {
                self.log.debug_warn(format!("Spawn blocked at ({}, {})", spawn.x, spawn.y)); // debugging purposes only
                continue;
            }

            match &spawn.kind {
                SpawnKind::Npc { def_id } => {
                    let npc = self.create_npc(def_id.clone(), pos)?;
                    level.spawn_npc(npc)?;
                }
                SpawnKind::Item { def_id } => {
                    let item_id = self.register_item(def_id);
                    let item_sprite = self.create_item_sprite(item_id, pos)?;
                    level.spawn_item_sprite(item_sprite)?;
                }
            }
        }

        Ok(level)
    }

    pub fn load_generated_level(&mut self, level_nr: usize) -> Result<Level, GameError> {
        let proc_gen = ProcGenLevel::generate(self.proc_gen.next_u64());
        let data = LevelData::from(proc_gen);

        let mut level = Level::new();

        level.world.apply_world_data(&data, level_nr)?;
        level.entry = data.entry;
        level.exit = data.exit;

        for spawn in &data.spawns {
            let pos = Point::new(spawn.x, spawn.y);

            if !level.is_available(pos) {
                self.log.debug_warn(format!("Spawn blocked at ({}, {})", spawn.x, spawn.y)); // debugging purposes only
                continue;
            }

            match &spawn.kind {
                SpawnKind::Npc { def_id } => {
                    let npc = self.create_npc(def_id.clone(), pos)?;
                    level.spawn_npc(npc)?;
                }
                SpawnKind::Item { def_id } => {
                    let item_id = self.register_item(def_id);
                    let item_sprite = self.create_item_sprite(item_id, pos)?;
                    level.spawn_item_sprite(item_sprite)?;
                }
            }
        }

        Ok(level)
    }
}
