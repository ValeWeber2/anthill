#![allow(dead_code)]

use rand::{SeedableRng, rngs::StdRng};
use std::collections::HashMap;

use crate::core::entity_logic::{EntityId, Npc};
use crate::core::game_items::{GameItem, GameItemId, GameItemSprite};
use crate::core::player::Player;
use crate::world::world_loader::save_world_to_ron;
use crate::world::worldspace::World;

use crate::world::coordinate_system::Point;
use crate::world::world_data::SpawnKind;

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
        };

        let player_id = state.next_entity_id();
        state.player = Player::new(player_id);

        // Generating rng world
        let new_world = crate::proc_gen::bsp::generate_map();
        let save_result = save_world_to_ron(&new_world.1, "assets/worlds/rng_world.ron");
        if let Err(error) = save_result {
            state.log.debug_print(format!("{}", error));
        }

        // Loading World from File
        // let data = load_world_from_ron("assets/worlds/rng_world.ron")
        //     .expect("Failed to load or parse .ron file");
        let data = new_world.1;

        state.world = World::new();
        state.world.apply_world_data(&data).expect("Failed to apply world data");

        if let Some(r) = data.rooms.first() {
            state.player.character.base.pos = Point::new(r.x + 1, r.y + 1);
        }

        for s in &data.spawns {
            let pos = Point::new(s.x, s.y);

            if !state.is_available(pos) {
                state.log.debug_print(format!("Spawn blocked at ({}, {})", s.x, s.y));
                continue;
            }

            match &s.kind {
                SpawnKind::Npc { def_id } => {
                    let _ = state.spawn_npc(def_id.clone(), pos);
                }
                SpawnKind::Item { def_id } => {
                    let item_id = state.register_item(def_id.clone());
                    let _ = state.spawn_item(item_id, pos);
                }
            }
        }

        state.compute_fov();

        state
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
