#![allow(dead_code)]

use std::collections::HashMap;

use crate::core::entity_logic::Npc;
use crate::core::game_items::GameItemSprite;
use crate::data::levels::level_paths;
use crate::util::errors_results::DataError;
use crate::world::coordinate_system::Point;
use crate::world::world_data::WorldData;
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

    pub npcs: Vec<Npc>,
    pub npc_index: HashMap<EntityId, usize>,

    pub item_sprites: Vec<GameItemSprite>,
    pub item_sprites_index: HashMap<EntityId, usize>,
}

impl Level {
    pub fn builder(level_nr: usize) -> LevelBuilder {
        LevelBuilder::new(level_nr)
    }

    pub fn load_static_level(index: usize) -> Result<Self, GameError> {
        if index > level_paths().len() {
            return Err(GameError::from(DataError::StaticWorldNotFound(index)));
        }

        let data = load_world_from_ron(level_paths()[index])?;

        let level = Level::builder(index)
            .world(&data)
            .set_entry_point(data.entry)
            // .set_spawns(&data)
            .build();

        Ok(level)
    }
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

    pub fn goto_level(&mut self, index: usize) -> Result<(), GameError> {
        match self.levels.get(index) {
            Some(_) => self.level_nr = index,
            None => {
                self.initialize_level(index)?;
                self.level_nr = index;
            }
        }

        let new_level = self.current_level();
        self.player.character.base.pos = new_level.entry;
        self.compute_fov();

        Ok(())
    }

    pub fn initialize_level(&mut self, index: usize) -> Result<(), GameError> {
        // If current level is a static level
        // if index % STATIC_LEVEL_INTERVAL == 2 {
        let new_level: Level = if index == 0 || index == 1 {
            Level::load_static_level(index)?
        } else {
            todo!("Generate Level using Procedural Generation");
        };

        self.levels.insert(index, new_level);

        Ok(())
    }
}

#[derive(Default)]
pub struct LevelBuilder {
    pub level_nr: usize,

    pub world: World,

    pub entry: Point,

    pub npcs: Vec<Npc>,
    pub npc_index: HashMap<EntityId, usize>,

    pub item_sprites: Vec<GameItemSprite>,
    pub item_sprites_index: HashMap<EntityId, usize>,
}

impl LevelBuilder {
    pub fn new(level_nr: usize) -> Self {
        Self {
            level_nr,
            world: World::new(),
            entry: Point::default(),
            npcs: Vec::new(),
            npc_index: HashMap::new(),
            item_sprites: Vec::new(),
            item_sprites_index: HashMap::new(),
        }
    }

    /// Builder method that applies [WorldData] to self, which creates the worldspace.
    pub fn world(mut self, world_data: &WorldData) -> Self {
        let mut world = World::new();
        let _ = world.apply_world_data(world_data, self.level_nr);
        // TODO: Handle Errors

        self.world = world;
        self
    }

    pub fn set_entry_point(mut self, point: Point) -> Self {
        self.entry = point;
        self
    }

    // Builder method that applies [SpawnData] to self, which spawns all npcs and item_sprites.
    // pub fn set_spawns(mut self, world_data: &WorldData) -> Self {
    //     for spawn in &world_data.spawns {
    //         let pos = Point::new(spawn.x, spawn.y);
    //
    //         if !self.world.is_in_bounds(pos.x as isize, pos.y as isize) || !self.world.get_tile(pos).tile_type.is_walkable() {
    //             // self.log.debug_print(format!("Spawn blocked at ({}, {})", spawn.x, spawn.y)); // debugging purposes only
    //             continue;
    //         }
    //
    //         match &spawn.kind {
    //             SpawnKind::Npc { def_id } => {
    //                 let _ = self.spawn_npc(def_id.clone(), pos);
    //             }
    //             SpawnKind::Item { def_id } => {
    //                 let item_id = self.register_item(def_id.clone());
    //                 let _ = self.spawn_item(item_id, pos);
    //             }
    //         }
    //     }
    //
    //     self
    // }

    pub fn build(self) -> Level {
        Level {
            world: self.world,

            entry: self.entry,

            npcs: self.npcs,
            npc_index: self.npc_index,

            item_sprites: self.item_sprites,
            item_sprites_index: self.item_sprites_index,
        }
    }
}
