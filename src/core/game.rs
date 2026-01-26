#![allow(dead_code)]

use rand::{SeedableRng, rngs::StdRng};
use std::collections::HashMap;

use crate::core::entity_logic::{EntityId, Npc};
use crate::core::game_items::{GameItem, GameItemId, GameItemSprite};
use crate::core::player::Player;
use crate::world::{
    coordinate_system::Point, world_data::SpawnKind, world_loader::load_world_from_ron,
    worldspace::World,
};

// ----------------------------------------------
//                Game State Struct
// ----------------------------------------------
pub struct GameState {
    pub world: World,
    pub player: Player,
    pub log: Log,
    pub round_nr: u64,
    pub level_nr: u8,
    pub entity_id_counter: u32,
    pub npcs: Vec<Npc>,
    pub npc_index: HashMap<EntityId, usize>,
    pub item_sprites: Vec<GameItemSprite>,
    pub item_sprites_index: HashMap<EntityId, usize>,
    pub items: HashMap<GameItemId, GameItem>, // stores all items that are currently in the game
    pub item_id_counter: GameItemId,
    pub rng: StdRng,
    pub current_level: usize,
    pub level_paths: Vec<&'static str>,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            world: World::default(),
            player: Player::new(0),
            log: Log::new(true),
            round_nr: 0,
            level_nr: 1,
            entity_id_counter: 0,
            npcs: Vec::new(),
            npc_index: HashMap::new(),
            item_sprites: Vec::new(),
            item_sprites_index: HashMap::new(),
            items: HashMap::new(),
            item_id_counter: 0,
            rng: StdRng::seed_from_u64(73),
            current_level: 0,
            level_paths: vec![
                "assets/worlds/level_01.ron",
                "assets/worlds/level_02.ron",
                "assets/worlds/level_03.ron",
            ],
        };

        let player_id = state.next_entity_id();
        state.player = Player::new(player_id);

        state.load_level(0).expect("Failed to load the first level");
        state
    }

    pub fn load_level(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.level_paths.len() {
            return Err("The level index is out of bounds");
        }

        self.world = World::new(self);
        self.npcs.clear();
        self.npc_index.clear();
        self.item_sprites.clear();
        self.items.clear();
        self.item_sprites_index.clear();
        self.item_id_counter = 0;

        let data = load_world_from_ron(self.level_paths[index])
            .map_err(|_| "Failed to apply world data")?;
        self.world.apply_world_data(&data)?;

        if let Some(room) = data.rooms.first() {
            self.player.character.base.pos = Point::new(room.x + 1, room.y + 1);
        }

        for s in &data.spawns {
            let pos = Point::new(s.x, s.y);

            if !self.is_available(pos) {
                self.log.debug_print(format!("Spawn blocked at ({}, {})", s.x, s.y)); // debugging purposes only
                continue;
            }

            match &s.kind {
                SpawnKind::Npc { def_id } => {
                    let _ = self.spawn_npc(def_id.clone(), pos);
                }
                SpawnKind::Item { def_id } => {
                    let item_id = self.register_item(def_id.clone());
                    let _ = self.spawn_item(item_id, pos);
                }
            }
        }

        self.compute_fov();
        self.current_level = index;
        self.level_nr = (index + 1) as u8;

        Ok(())
    }

    // This is the routine of operations that need to be called every round.
    pub fn next_round(&mut self) {
        let npc_ids: Vec<EntityId> = self.npc_index.keys().copied().collect();

        for npc_id in npc_ids {
            let _ = self.npc_take_turn(npc_id);
        }

        self.compute_fov();

        self.round_nr += 1;
    }
}

impl Default for GameState {
    // placeholder, only for tests
    fn default() -> Self {
        Self {
            world: World::default(),
            player: Player::default(),
            log: Log::new(true),
            round_nr: 0,
            level_nr: 1,
            entity_id_counter: 0,
            npcs: Vec::new(),
            npc_index: HashMap::new(),
            item_sprites: Vec::new(),
            item_sprites_index: HashMap::new(),
            items: HashMap::new(),
            item_id_counter: 0,
            rng: StdRng::seed_from_u64(73),
            current_level: 0,
            level_paths: Vec::new(),
        }
    }
}

// ----------------------------------------------
//                  Game Text Log
// ----------------------------------------------
pub struct Log {
    pub print_debug_info: bool,
    pub messages: Vec<String>,
}

impl Log {
    pub fn new(print_debug_info: bool) -> Self {
        Self { print_debug_info, messages: Vec::new() }
    }

    pub fn print(&mut self, message: String) {
        let lines: Vec<&str> = message.split("\n").collect();
        for line in lines {
            self.messages.push(line.to_string());
        }
    }

    pub fn debug_print(&mut self, message: String) {
        if !self.print_debug_info {
            return;
        }

        self.print(message);
    }
}
