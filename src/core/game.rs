#![allow(dead_code)]

use std::collections::HashMap;

use crate::core::game_items::{GameItem, GameItemId};
use crate::core::player::Player;
use crate::world::worldspace::World;

// ----------------------------------------------
//                Game State Struct
// ----------------------------------------------
pub struct GameState {
    pub world: World,
    pub player: Player,
    pub log: Log,
    pub round_nr: u64,
    pub entity_id_counter: u32,
    pub items: HashMap<GameItemId, GameItem>, // stores all items that are currently in the game
    pub item_id_counter: GameItemId,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            world: World::default(),
            player: Player::new(0),
            log: Log::new(),
            round_nr: 0,
            entity_id_counter: 0,
            items: HashMap::new(),
            item_id_counter: 0,
        };

        let player_id = state.next_entity_id();
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
            entity_id_counter: 0,
            items: HashMap::new(),
            item_id_counter: 0,
        }
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
