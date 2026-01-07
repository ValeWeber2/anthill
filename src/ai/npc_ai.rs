#![allow(dead_code)]

use crate::{
    core::{
        entity_logic::{Entity, EntityId, Npc},
        game::GameState,
    },
    world::worldspace::{Direction, MovementError, Point, PointDelta},
};

#[derive(Default)]
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
    pub fn npc_take_turn(&mut self, npc_id: EntityId) -> Result<(), &'static str> {
        // Update NpcAiState
        self.npc_refresh_ai_state(npc_id)?;

        // Decide Action
        let npc_action = self.npc_choose_action(npc_id)?;

        // Resolve Action
        match npc_action {
            NpcActionKind::Wait => {}
            NpcActionKind::Move(direction) => {
                let delta = PointDelta::from(direction);
                let _ = self.world.move_npc(npc_id, delta.x, delta.y);
            }
            NpcActionKind::Attack => todo!(),
        }

        Ok(())
    }

    fn npc_choose_action(&mut self, npc_id: EntityId) -> Result<NpcActionKind, &'static str> {
        let npc = self.world.get_npc(npc_id).ok_or("npc not found")?;
        let melee_area = self.world.get_points_in_radius(npc.pos(), 1);

        let npc_action = match npc.ai_state {
            NpcAiState::Inactive => NpcActionKind::Wait,
            NpcAiState::Wandering => {
                let random_direction = Direction::random(&mut self.rng);
                NpcActionKind::Move(random_direction)
            }
            NpcAiState::Aggressive => {
                if melee_area.contains(self.player.character.pos()) {
                    NpcActionKind::Attack
                } else {
                    todo!("Pathfinding algorithm");
                }
            }
        };
        Ok(npc_action)
    }

    fn npc_refresh_ai_state(&mut self, npc_id: EntityId) -> Result<(), &'static str> {
        let npc_pos: Point = {
            let npc: &Npc = self.world.get_npc(npc_id).ok_or("npc not found")?;
            *npc.pos()
        };

        let detectable_area: Vec<Point> = self.world.get_points_in_radius(&npc_pos, 10);

        let npc: &mut Npc = self.world.get_npc_mut(npc_id).ok_or("npc not found")?;

        if detectable_area.contains(self.player.character.pos()) {
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
