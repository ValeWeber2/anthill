#![allow(dead_code)]

use crate::{
    core::{
        entity_logic::{Entity, EntityId, EntityRef},
        game::GameState,
    },
    world::worldspace::{Direction, Point},
};

pub enum PlayerInput {
    Wait,
    Direction(Direction), // UseItem
}

pub enum ActionKind {
    Wait,
    Move(Direction),
    Attack(EntityId),
    PickUpItem(EntityId),
}

impl GameState {
    pub fn resolve_player_action(&mut self, input: PlayerInput) {
        let intended_action = self.interpret_player_input(input);

        let action_result = match intended_action {
            ActionKind::Wait => Ok(()),
            ActionKind::Move(Direction::Up) => {
                self.world.move_entity(&mut self.player.character, 0, -1)
            }
            ActionKind::Move(Direction::Right) => {
                self.world.move_entity(&mut self.player.character, 1, 0)
            }
            ActionKind::Move(Direction::Down) => {
                self.world.move_entity(&mut self.player.character, 0, 1)
            }
            ActionKind::Move(Direction::Left) => {
                self.world.move_entity(&mut self.player.character, -1, 0)
            }
            ActionKind::Attack(_) => todo!(),
            ActionKind::PickUpItem(_) => todo!(),
        };

        if action_result.is_ok() {
            self.next_round();
        }
    }

    pub fn interpret_player_input(&mut self, input: PlayerInput) -> ActionKind {
        match input {
            PlayerInput::Direction(direction) => {
                let target_point: Point = self.player.character.pos().get_neighbour(direction);
                // let target_tile = self.world.get_tile(target_point.x, target_point.y);

                if let Some(entity_id) = self.get_entity_at(target_point) {
                    match self.get_entity_by_id(entity_id) {
                        Some(EntityRef::Npc(_)) => {
                            return ActionKind::Attack(entity_id);
                        }
                        Some(EntityRef::ItemSprite(_)) => {
                            return ActionKind::PickUpItem(entity_id);
                        }
                        _ => {}
                    }
                }

                ActionKind::Move(direction)
            }
            PlayerInput::Wait => ActionKind::Wait,
        }
    }
}
