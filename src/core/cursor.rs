use crate::{
    core::{entity_logic::Entity, game::GameState, player_actions::PlayerInput},
    util::{
        errors_results::{EngineError, FailReason, GameError, GameOutcome, GameResult},
        text_log::LogData,
    },
    world::coordinate_system::{Direction, Point},
};

/// Tracks the cursor mode in the game.
///
/// Usually the player controls the player character in the world, but if a Cursor State is set in [GameState], then the player controls the cursor.
pub struct CursorState {
    /// Mode of the Cursor. Determins which actions can be taken with the cursor.
    pub kind: CursorMode,

    /// Coordinates of the Cursor.
    pub point: Point,
}

/// Contains all modes for a cursor.
pub enum CursorMode {
    /// Look mode is used to get a description of what the cursor is pointing at.
    Look,

    /// Ranged attack mode allows the player to attack at long range (provided a ranged weapon is equipped)
    RangedAttack,
}

impl GameState {
    /// Moves the cursor's position in the given direction.
    ///
    /// # Errors
    /// * [EngineError::CursorNotSet] if no cursor instance could be found. This happens when [GameState::cursor] == `None`.
    ///
    /// # Returns
    /// * [GameOutcome::Success] if the movement was successful
    /// * [GameOutcome::Fail] with [FailReason::PointOutOfBounds] if the movement was successful
    pub fn move_cursor(&mut self, direction: Direction) -> GameResult {
        let Some(point) = self.cursor.as_ref().map(|cursor_state| cursor_state.point) else {
            return Err(GameError::from(EngineError::CursorNotSet));
        };

        let new_point = point + direction;

        if !self.current_world().is_in_bounds(new_point.x as isize, new_point.y as isize) {
            return Ok(GameOutcome::Fail(FailReason::PointOutOfBounds(new_point)));
        }

        if let Some(cursor) = self.cursor.as_mut() {
            cursor.point = new_point;
        }

        Ok(GameOutcome::Success)
    }

    /// Triggered when activating an action with the cursor (i.e. pressing ENTER)
    /// Differentiates cases between the different cursor modes.
    pub fn resolve_cursor_action(&mut self) -> GameResult {
        let Some(cursor) = &self.cursor else {
            return Err(GameError::from(EngineError::CursorNotSet));
        };

        // Non-visible target points can't be interacted with.
        if !self.current_world().get_tile(cursor.point).visible {
            self.log.info(LogData::TileNotVisible);
            return Ok(GameOutcome::Fail(FailReason::TileNotVisible(cursor.point)));
        }

        match cursor.kind {
            CursorMode::Look => {
                self.look_at_point(cursor.point);

                Ok(GameOutcome::Success)
            }
            CursorMode::RangedAttack => {
                if let Some(entity_id) = self.current_level().get_npc_at(cursor.point) {
                    self.resolve_player_action(PlayerInput::RangedAttack(entity_id));
                }

                Ok(GameOutcome::Success)
            }
        }
    }

    fn look_at_point(&mut self, point: Point) {
        // Unoccupied target points only output tile type.
        if !self.current_level().is_occupied(point) {
            let tile = self.current_world().get_tile(point);
            self.log.info(LogData::LookAt { name: tile.tile_type.to_string() });
            return;
        }

        // Otherwise, a target point is occupied, so info about NPCs and/or Item Sprites is displayed.
        if let Some(entity_id) = self.current_level().get_npc_at(point) {
            if let Some(npc) = self.current_level().get_npc(entity_id) {
                self.log.info(LogData::LookAt { name: npc.name().to_string() });
            }
        }

        if let Some(entity_id) = self.current_level().get_item_sprite_at(point) {
            if let Some(item_sprite) = self.current_level().get_item_sprite(entity_id) {
                self.log.info(LogData::LookAt { name: item_sprite.name().to_string() });
            }
        }
    }
}
