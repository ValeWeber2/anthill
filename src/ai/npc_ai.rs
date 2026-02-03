#![allow(dead_code)]

use crate::{
    core::{
        entity_logic::{Entity, EntityId, Npc},
        game::GameState,
    },
    util::errors_results::{EngineError, GameError, GameOutcome, GameResult},
    world::{
        coordinate_system::{Direction, Point, PointVector},
        worldspace::MovementError,
    },
};

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
                self.log.print("The NPC attacks".to_string());
                let _ = self.npc_attack_player(npc_id);
            }
        }

        Ok(GameOutcome::Success)
    }

    fn npc_choose_action(&mut self, npc_id: EntityId) -> Result<NpcActionKind, GameError> {
        let npc = self.current_level().get_npc(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;
        let melee_area = self.current_world().get_points_in_radius(npc.pos(), 1);

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
                    self.current_world().next_step_toward(npc.pos(), self.player.character.pos())
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

        let player_pos: Point = { self.player.character.pos() };

        let detectable_area: Vec<Point> = self.current_world().get_points_in_radius(npc_pos, 10);

        let npc: &mut Npc =
            self.current_level_mut().get_npc_mut(npc_id).ok_or(EngineError::NpcNotFound(npc_id))?;

        if detectable_area.contains(&player_pos) {
            npc.ai_state = NpcAiState::Aggressive;
        } else {
            npc.ai_state = NpcAiState::Wandering;
        }

        Ok(())
    }
}

pub enum NpcAiError {
    MovementError(MovementError),
    NpcNotFound,
}
