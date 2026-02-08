#![allow(dead_code)]

use rand::Rng;
use rand::{SeedableRng, rngs::StdRng};
use std::collections::HashMap;

use bitflags::bitflags;

use crate::core::entity_logic::EntityId;
use crate::core::game_items::{GameItem, GameItemId};
use crate::core::player::Player;
use crate::util::errors_results::{FailReason, GameOutcome};
use crate::util::text_log::Log;
use crate::world::coordinate_system::{Direction, Point};
use crate::world::level::Level;

// ----------------------------------------------
//                Game State Struct
// ----------------------------------------------
pub struct GameState {
    /// Contains the data for every level in the game.
    pub levels: Vec<Level>,

    /// Points to the [Level] the player is on.
    pub level_nr: usize,

    pub player: Player,

    /// Represents the character on screen that is being controlled. (Usually on the Player character, but can be detached.)
    pub cursor: Option<CursorState>,

    pub log: Log,
    pub round_nr: u64,

    pub id_system: IdSystem,
    pub items: HashMap<GameItemId, GameItem>, // stores all items that are currently in the game

    pub rng: StdRng,
    pub proc_gen: StdRng,

    pub game_rules: GameRules,
}

impl GameState {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(73);
        let proc_gen = StdRng::seed_from_u64(rng.random());

        let mut state = Self {
            levels: Vec::new(),
            player: Player::new(0),
            cursor: None,
            log: Log::new(true),
            round_nr: 0,
            level_nr: 0,
            id_system: IdSystem::default(),
            items: HashMap::new(),
            rng: StdRng::seed_from_u64(73),
            proc_gen,
            game_rules: GameRules::empty(),
        };

        let player_id = state.id_system.next_entity_id();
        state.player = Player::new(player_id);

        let _ = state.goto_level(state.level_nr);
        state
    }

    // This is the routine of operations that need to be called every round.
    pub fn next_round(&mut self) {
        let npc_ids: Vec<EntityId> = self.current_level().npc_index.keys().copied().collect();

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
            levels: Vec::new(),
            level_nr: 0,
            player: Player::default(),
            cursor: None,
            log: Log::new(true),
            round_nr: 0,
            id_system: IdSystem::default(),
            items: HashMap::new(),
            rng: StdRng::seed_from_u64(73),
            proc_gen: StdRng::seed_from_u64(42),
            game_rules: GameRules::empty(),
        }
    }
}

// ----------------------------------------------
//                  ID System
// ----------------------------------------------

#[derive(Default)]
pub struct IdSystem {
    entity_id_counter: EntityId,
    item_id_counter: GameItemId,
}

impl IdSystem {
    pub fn next_entity_id(&mut self) -> EntityId {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;

        id
    }

    pub fn next_item_id(&mut self) -> GameItemId {
        let id = self.item_id_counter;
        self.item_id_counter += 1;

        id
    }
}

// ----------------------------------------------
//                Gamerule System
// ----------------------------------------------

bitflags! {
    pub struct GameRules: u8 {
        // This disables collision detection for the player, allowing them to walk through walls.
        const NO_CLIP = 0b00000001;
    }
}

// ----------------------------------------------
//                Cursor System
// ----------------------------------------------

pub struct CursorState {
    pub kind: CursorKind,
    pub point: Point,
}

pub enum CursorKind {
    Look,
    RangedAttack,
}

impl GameState {
    pub fn move_cursor(&mut self, direction: Direction) -> GameOutcome {
        let Some(point) = self.cursor.as_ref().map(|cursor_state| cursor_state.point) else {
            return GameOutcome::Success;
        };

        let new_pos = point + direction;

        if !self.current_world().is_in_bounds(new_pos.x as isize, new_pos.y as isize) {
            return GameOutcome::Fail(FailReason::PointOutOfBounds(new_pos));
        }

        if !self.current_world().get_tile(new_pos).visible {
            return GameOutcome::Fail(FailReason::TileNotVisible(new_pos));
        }

        if let Some(cursor) = self.cursor.as_mut() {
            cursor.point = new_pos;
        }

        GameOutcome::Success
    }
}
