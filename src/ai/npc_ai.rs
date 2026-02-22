#![allow(dead_code)]

use crate::{
    core::{
        entity_logic::{Entity, EntityId, Npc},
        game::GameState,
    },
    util::errors_results::{EngineError, GameError, GameOutcome, GameResult},
    world::{
        coordinate_system::{Direction, Point, PointVector},
        tiles::Collision,
        worldspace::MovementError,
    },
};

pub const AGGRO_RADIUS: usize = 6;

#[derive(Default, Clone)]
pub enum NpcAiState {
    #[default]
    Inactive,
    Wandering,
    Aggressive,
}

pub enum NpcActionKind {
    Wait,
    Move(Direction),
    Attack,
}

impl GameState {
    pub fn npc_take_turn(&mut self, npc_id: EntityId) -> GameResult {
        // Update NpcAiState
        self.npc_refresh_ai_state(npc_id)?;

        // Decide Action
        let npc_action = self.npc_choose_action(npc_id)?;

        // Resolve Action
        match npc_action {
            NpcActionKind::Wait => {}
            NpcActionKind::Move(direction) => {
                let delta = PointVector::from(direction);
                let _ = self.move_npc(npc_id, delta.x, delta.y);
            }
            NpcActionKind::Attack => {
                let _ = self.npc_attack_player(npc_id);
            }
        }

        Ok(GameOutcome::Success)
    }

    fn npc_choose_action(&mut self, npc_id: EntityId) -> Result<NpcActionKind, GameError> {
        let npc = self.current_level().get_npc(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;
        // let melee_area = self.current_world().get_points_in_radius(npc.pos(), 1);
        let npc_pos = npc.pos();
        let melee_area: Vec<Point> = vec![
            npc_pos + Direction::Up,
            npc_pos + Direction::Right,
            npc_pos + Direction::Down,
            npc_pos + Direction::Left,
        ];

        let action = match npc.ai_state {
            NpcAiState::Inactive => NpcActionKind::Wait,

            NpcAiState::Wandering => {
                let random_direction = Direction::random(&mut self.rng);
                NpcActionKind::Move(random_direction)
            }

            NpcAiState::Aggressive => {
                if melee_area.contains(&self.player.character.pos()) {
                    NpcActionKind::Attack
                } else if let Some(next_step) =
                    self.next_step_toward(npc.pos(), self.player.character.pos())
                {
                    NpcActionKind::Move(next_step)
                } else {
                    let random_direction = Direction::random(&mut self.rng);
                    NpcActionKind::Move(random_direction)
                }
            }
        };
        Ok(action)
    }

    fn npc_refresh_ai_state(&mut self, npc_id: EntityId) -> Result<(), GameError> {
        let npc_pos: Point = {
            let npc: &Npc =
                self.current_level().get_npc(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;
            npc.pos()
        };

        let player_pos: Point = self.player.character.pos();
        let detectable_area: Vec<Point> = self.current_world().get_points_in_radius(npc_pos, 6);

        let player_reachable = self.current_world().get_tile(player_pos).tile_type.is_walkable();
        // Only aggressive if player in detection radius and player is on a reachable tile (e.g. not inside walls)
        let should_be_agressive = detectable_area.contains(&player_pos) && player_reachable;

        let npc: &mut Npc =
            self.current_level_mut().get_npc_mut(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;

        // If the detection radius contains the player AND the player position is reachable.
        npc.ai_state =
            if should_be_agressive { NpcAiState::Aggressive } else { NpcAiState::Wandering };

        Ok(())
    }
}

pub enum NpcAiError {
    MovementError(MovementError),
    NpcNotFound,
}
