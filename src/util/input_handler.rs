use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

use crate::{
    App, State,
    core::{
        entity_logic::Entity,
        game::{CursorMode, CursorState},
        player_actions::PlayerInput,
    },
    render::{
        menu_display::{InventoryAction, MenuMode},
        modal_display::{ModalInterface, SelectionAction},
    },
    util::{errors_results::GameOutcome, text_log::LogData},
    world::coordinate_system::Direction,
};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum KeyboardFocus {
    #[default]
    FocusWorld,
    FocusMenu,
}

pub enum ModalAction {
    Idle,
    CloseModal,
    RunCommand(String),
}

impl App {
    /// Sets the Application Focus to the given menu type (e.g. Inventory)
    fn focus_menu(&mut self, mode: MenuMode) {
        self.ui.menu.mode = mode;
        self.keyboard_focus = KeyboardFocus::FocusMenu;
    }

    /// Resets the Application Focus to the default (World) and resets the menu to display the Log.
    fn focus_reset(&mut self) {
        self.keyboard_focus = KeyboardFocus::FocusWorld;
        self.ui.menu.mode = MenuMode::Log;
    }

    /// Central event handler.
    ///
    /// Currently, only takes keyboard events into consideration.
    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    /// Central event handler for keyboard input.
    ///
    /// Here it switches the event handling logic depending on what menu or ui-section the user is interacting with.
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        // 1. Prioritise Modal
        if self.ui.modal.is_some() {
            self.handle_modal_key_event(key_event);
            return;
        }

        // 2. Global hotkeys (work in all states)
        if self.handle_global_hotkeys(key_event) {
            return;
        }

        // 3. State-specific input
        match self.state {
            State::StartScreen => {
                self.handle_start_screen_input(key_event);
            }
            State::Playing => {
                self.handle_playing_input(key_event);
            }
            State::GameOver => {
                self.handle_game_over_input(key_event);
            }
        }
    }

    /// Hotkeys that are always available regardless of ui state.
    fn handle_global_hotkeys(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            // Control: Open game closing modal (SHIFT+q)
            KeyCode::Char('Q') => {
                self.ui.modal = Some(ModalInterface::ConfirmQuit);
                true
            }
            // Control: Open help window
            KeyCode::Char('H') => {
                self.ui.modal = Some(ModalInterface::HelpDisplay);
                true
            }
            _ => false,
        }
    }

    /// Handling input in the starting screen.
    fn handle_start_screen_input(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Enter {
            self.state = State::Playing
        }
    }

    /// Handling while playing the game.
    ///
    /// Here it switches the event handling logic depending on if the UI focus is on the world or the menu.
    fn handle_playing_input(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(':') => {
                self.ui.modal = Some(ModalInterface::CommandInput { buffer: "".to_string() })
            }
            // Other key events depending on keyboard focus
            _ => match self.keyboard_focus {
                KeyboardFocus::FocusWorld => self.handle_world_key_event(key_event),
                KeyboardFocus::FocusMenu => self.handle_menu_key_event(key_event),
            },
        }
    }

    /// Handling input in the Game Over screen.
    fn handle_game_over_input(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Enter {
            self.restart()
        }
    }

    /// Handling input while the focus is on the world. This is where the game's main controls for the player character are.
    fn handle_world_key_event(&mut self, key_event: KeyEvent) {
        if self.game.cursor.is_some() {
            self.handle_cursor_key_event(key_event);
            return;
        }

        match key_event.code {
            // Action: Move up
            KeyCode::Char('w') => {
                self.game.resolve_player_action(PlayerInput::Direction(Direction::Up));
            }
            // Action: Move down
            KeyCode::Char('s') => {
                self.game.resolve_player_action(PlayerInput::Direction(Direction::Down));
            }
            // Action: Move left
            KeyCode::Char('a') => {
                self.game.resolve_player_action(PlayerInput::Direction(Direction::Left));
            }
            // Action: Move right
            KeyCode::Char('d') => {
                self.game.resolve_player_action(PlayerInput::Direction(Direction::Right));
            }
            // Action: Wait
            KeyCode::Char('.') => {
                self.game.resolve_player_action(PlayerInput::Wait);
            }
            // Action: Unequip Weapon
            KeyCode::Char('W') => {
                self.game.resolve_player_action(PlayerInput::UnequipWeapon);
            }
            // Action: Unequip Armor
            KeyCode::Char('A') => {
                self.game.resolve_player_action(PlayerInput::UnequipArmor);
            }

            // Control: Open Inventory with intention to Action: Use Item (shifts focus to menu)
            KeyCode::Char('i') => {
                self.focus_menu(MenuMode::Inventory(InventoryAction::Use));
            }
            // Control: Open Inventory with intention to Action: Leave Item (shifts focus to menu)
            KeyCode::Char('D') => {
                self.focus_menu(MenuMode::Inventory(InventoryAction::Drop));
            }

            // Control: Start Look mode
            KeyCode::Char('l') => {
                self.game.cursor = Some(CursorState {
                    kind: CursorMode::Look,
                    point: self.game.player.character.pos(),
                });
            }

            // Control: Start Ranged Attack modej
            KeyCode::Char('r') => {
                self.game.cursor = Some(CursorState {
                    kind: CursorMode::RangedAttack,
                    point: self.game.player.character.pos(),
                });
            }

            // Debug: Print player pos
            KeyCode::Char('p') => self.game.log.debug_info(format!(
                "Player at position x: {}, y: {}",
                self.game.player.character.base.pos.x, self.game.player.character.base.pos.y
            )),

            // Debug: Print game iten register
            KeyCode::Char('o') => {
                for (item_id, item) in self.game.items.iter() {
                    self.game.log.debug_info(format!("Item ID: {} DEF: {}", item_id, item.def_id))
                }
            }
            // Debug: Open Test Modal
            KeyCode::Char('9') => {
                self.ui.modal = Some(ModalInterface::TextDisplay {
                    title: "Test Display".to_string(),
                    paragraphs: vec![
                        "Das ist ein Test".to_string(),
                        "Hier ein weiterer Paragraph".to_string(),
                    ],
                })
            }
            KeyCode::Char('8') => {
                self.ui.modal = Some(ModalInterface::SelectPrompt {
                    selection_action: SelectionAction::Debug,
                    options: vec!["Message 1".into(), "Message 2".into(), "Message 3".into()],
                })
            }
            _ => {}
        }
    }

    /// Handling input while the focus is on the menu.
    ///
    /// Here it switches the event handling logic depending on if the inventory was opened or the log. The log has no controls and is generally not accessible to the player.
    fn handle_menu_key_event(&mut self, key_event: KeyEvent) {
        match &self.ui.menu.mode {
            MenuMode::Inventory(_) => self.handle_inventory_key_event(key_event),
            MenuMode::Log => {}
        }
    }

    /// Handling the input while a modal display is opened.
    ///
    /// Handling input for each of the different modal types.
    fn handle_modal_key_event(&mut self, key_event: KeyEvent) {
        let modal_action = if let Some(modal) = &mut self.ui.modal {
            match modal {
                ModalInterface::ConfirmQuit => match key_event.code {
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                        ModalAction::Idle
                    }
                    _ => ModalAction::CloseModal,
                },
                ModalInterface::ConfirmUseItem { item_id } => match key_event.code {
                    KeyCode::Char('y') | KeyCode::Enter => {
                        self.game.resolve_player_action(PlayerInput::UseItem(*item_id));

                        ModalAction::CloseModal
                    }
                    KeyCode::Char('n') | KeyCode::Esc => ModalAction::CloseModal,
                    _ => ModalAction::Idle,
                },
                ModalInterface::ConfirmDropItem { item_id } => match key_event.code {
                    KeyCode::Char('y') | KeyCode::Enter => {
                        self.game.resolve_player_action(PlayerInput::DropItem(*item_id));

                        ModalAction::CloseModal
                    }
                    KeyCode::Char('n') | KeyCode::Esc => ModalAction::CloseModal,
                    _ => ModalAction::Idle,
                },
                ModalInterface::CommandInput { buffer } => match key_event.code {
                    KeyCode::Char(c) => {
                        buffer.push(c);
                        ModalAction::Idle
                    }
                    KeyCode::Backspace => {
                        buffer.pop();
                        ModalAction::Idle
                    }
                    KeyCode::Esc => ModalAction::CloseModal,
                    KeyCode::Enter => ModalAction::RunCommand(buffer.to_string()),
                    _ => ModalAction::Idle,
                },
                ModalInterface::TextDisplay { .. } => match key_event.code {
                    KeyCode::Esc => ModalAction::CloseModal,
                    KeyCode::Enter => ModalAction::CloseModal,
                    _ => ModalAction::Idle,
                },
                ModalInterface::HelpDisplay => match key_event.code {
                    KeyCode::Esc => ModalAction::CloseModal,
                    KeyCode::Enter => ModalAction::CloseModal,
                    _ => ModalAction::Idle,
                },
                ModalInterface::SelectPrompt { selection_action, options } => {
                    match key_event.code {
                        KeyCode::Esc => ModalAction::CloseModal,
                        KeyCode::Char(c) => {
                            // Getting the selected option
                            if let Some(index) = letter_to_index(c) {
                                if let Some(option) = options.get(index) {
                                    // Appying the selection action to the selected option
                                    match selection_action {
                                        SelectionAction::Debug => {
                                            self.game.log.debug_info(option.to_string())
                                        }
                                    }
                                }
                            }
                            ModalAction::Idle
                        }
                        _ => ModalAction::Idle,
                    }
                }
            }
        } else {
            return;
        };

        match modal_action {
            ModalAction::Idle => {}
            ModalAction::CloseModal => self.ui.modal = None,
            ModalAction::RunCommand(command) => {
                self.run_command(command);
                self.ui.modal = None;
            }
        }
    }

    /// Handling input while the menu is focused and the inventory is open. Allows interaction with the inventory.
    fn handle_inventory_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.focus_reset();
            }
            KeyCode::Char('W') => {
                if let Err(e) = self.game.unequip_weapon() {
                    self.game.log.debug_warn(format!("{}", e));
                }
            }
            KeyCode::Char('A') => {
                if let Err(e) = self.game.unequip_armor() {
                    self.game.log.debug_warn(format!("{}", e));
                }
            }
            KeyCode::Char(c) => {
                if let Some(index) = letter_to_index(c) {
                    if let Some(item_id) = self.game.player.character.inventory.get(index) {
                        match self.ui.menu.mode {
                            MenuMode::Inventory(InventoryAction::Use) => {
                                self.ui.modal =
                                    Some(ModalInterface::ConfirmUseItem { item_id: *item_id });
                            }
                            MenuMode::Inventory(InventoryAction::Drop) => {
                                self.ui.modal =
                                    Some(ModalInterface::ConfirmDropItem { item_id: *item_id });
                            }
                            _ => {}
                        }
                    }
                }
            }

            _ => {}
        }
    }

    /// Handling input while there is an instance of the cursor. Allows moving the cursor and performing actions with the cursor.
    fn handle_cursor_key_event(&mut self, key_event: KeyEvent) {
        if let Some(cursor) = &self.game.cursor {
            match key_event.code {
                KeyCode::Char(c) => {
                    let cursor_move_result = match c {
                        'w' => self.game.move_cursor(Direction::Up),
                        's' => self.game.move_cursor(Direction::Down),
                        'a' => self.game.move_cursor(Direction::Left),
                        'd' => self.game.move_cursor(Direction::Right),
                        _ => Ok(GameOutcome::Success),
                    };

                    if let Err(error) = cursor_move_result {
                        self.game.log.debug_warn(error.to_string());
                        self.game.cursor = None;
                    }
                }

                // Run cursor action
                KeyCode::Enter => {
                    // Non-visible target points can't be interacted with.
                    if !self.game.current_world().get_tile(cursor.point).visible {
                        self.game.log.info(LogData::TileNotVisible);
                        return;
                    }

                    match cursor.kind {
                        CursorMode::Look => {
                            // Unoccupied target points only output tile type.
                            if !self.game.current_level().is_occupied(cursor.point) {
                                let tile = self.game.current_world().get_tile(cursor.point);
                                self.game
                                    .log
                                    .info(LogData::LookAt { name: tile.tile_type.to_string() });
                                return;
                            }

                            // Otherwise, a target point is occupied, so info about NPCs and/or Item Sprites is displayed.
                            if let Some(entity_id) =
                                self.game.current_level().get_npc_at(cursor.point)
                            {
                                if let Some(npc) = self.game.current_level().get_npc(entity_id) {
                                    self.game
                                        .log
                                        .info(LogData::LookAt { name: npc.name().to_string() });
                                }
                            }

                            if let Some(entity_id) =
                                self.game.current_level().get_item_sprite_at(cursor.point)
                            {
                                if let Some(item_sprite) =
                                    self.game.current_level().get_item_sprite(entity_id)
                                {
                                    self.game.log.info(LogData::LookAt {
                                        name: item_sprite.name().to_string(),
                                    });
                                }
                            }
                        }
                        CursorMode::RangedAttack => {
                            if let Some(entity_id) =
                                self.game.current_level().get_npc_at(cursor.point)
                            {
                                self.game
                                    .resolve_player_action(PlayerInput::RangedAttack(entity_id));
                            }
                        }
                    }
                }

                KeyCode::Esc => self.game.cursor = None,
                _ => {}
            };
        }
    }
}

/// Helper function to convert letter input [a-z] into a number [0-25] to access item indices in the inventory.
fn letter_to_index(c: char) -> Option<usize> {
    if c.is_ascii_lowercase() { Some((c as u8 - b'a') as usize) } else { None }
}
