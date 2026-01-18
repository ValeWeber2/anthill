use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

use crate::{
    App, State,
    ascii_art::HELP_CONTENT,
    core::player_actions::PlayerInput,
    render::{
        menu_display::{InventoryAction, MenuMode},
        modal_display::ModalInterface,
    },
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

    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

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

    fn handle_global_hotkeys(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            // Control: Open game closing modal (SHIFT+q)
            KeyCode::Char('Q') => {
                self.ui.modal = Some(ModalInterface::ConfirmQuit);
                true
            }
            // Control: Open help window
            KeyCode::Char('H') => {
                self.ui.modal = Some(ModalInterface::TextDisplay {
                    title: " Help ".into(),
                    paragraphs: HELP_CONTENT.lines().map(|l| l.to_string()).collect(),
                });
                true
            }
            _ => false,
        }
    }

    fn handle_start_screen_input(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Enter {
            self.state = State::Playing
        }
    }

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

    fn handle_game_over_input(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Enter {
            self.restart()
        }
    }

    fn handle_world_key_event(&mut self, key_event: KeyEvent) {
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

            // Control: Open Inventory without any intentions
            KeyCode::Char('i') => {
                self.focus_menu(MenuMode::Inventory(InventoryAction::View));
            }
            // Control: Open Inventory with intention to Action: Use Item (shifts focus to menu)
            KeyCode::Char('u') => {
                self.focus_menu(MenuMode::Inventory(InventoryAction::Use));
            }
            // Control: Open Inventory with intention to Action: Leave Item (shifts focus to menu)
            KeyCode::Char('l') => {
                self.focus_menu(MenuMode::Inventory(InventoryAction::Drop));
            }

            // Debug: Print player pos
            KeyCode::Char('p') => self.game.log.print(format!(
                "Player at position x: {}, y: {}",
                self.game.player.character.base.pos.x, self.game.player.character.base.pos.y
            )),

            // Debug: Print game iten register
            KeyCode::Char('o') => {
                for (item_id, item) in self.game.items.iter() {
                    self.game.log.print(format!("Item ID: {} DEF: {}", item_id, item.def_id))
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
            _ => {}
        }
    }

    fn handle_menu_key_event(&mut self, key_event: KeyEvent) {
        match &self.ui.menu.mode {
            MenuMode::Inventory(_) => self.handle_inventory_key_event(key_event),
            MenuMode::Log => {}
        }
    }

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
                ModalInterface::ConfirmChooseItem { item_id } => match key_event.code {
                    KeyCode::Char('y') | KeyCode::Enter => {
                        match self.ui.menu.mode {
                            MenuMode::Inventory(InventoryAction::Use) => {
                                self.game.resolve_player_action(PlayerInput::UseItem(*item_id))
                            }
                            MenuMode::Inventory(InventoryAction::Drop) => {
                                self.game.resolve_player_action(PlayerInput::DropItem(*item_id))
                            }
                            _ => {}
                        }

                        // close inventory after using
                        self.focus_reset();

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

    fn handle_inventory_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.focus_reset();
            }
            KeyCode::Char('W') => {
                if let Err(e) = self.game.unequip_weapon() {
                    self.game.log.print(format!("{}", e));
                }
            }
            KeyCode::Char('A') => {
                if let Err(e) = self.game.unequip_armor() {
                    self.game.log.print(format!("{}", e));
                }
            }
            KeyCode::Char(c) => {
                if let Some(index) = App::letter_to_index(c) {
                    if let Some(item_id) = self.game.player.character.inventory.get(index) {
                        self.ui.modal =
                            Some(ModalInterface::ConfirmChooseItem { item_id: *item_id });
                    }
                }
            }

            _ => {}
        }
    }

    fn letter_to_index(c: char) -> Option<usize> {
        if c.is_ascii_lowercase() { Some((c as u8 - b'a') as usize) } else { None }
    }
}
