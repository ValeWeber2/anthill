use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

use crate::{
    App,
    core::player_actions::PlayerInput,
    render::{menu_display::MenuMode, modal_display::ModalInterface},
    world::worldspace::Direction,
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
        // Prioritises Model
        if self.ui.modal.is_some() {
            self.handle_modal_key_event(key_event);
            return;
        }

        // Universal key_events (regardless of focus)
        match key_event.code {
            // Control: Open game closing modal (SHIFT+q)
            KeyCode::Char('Q') => self.ui.modal = Some(ModalInterface::ConfirmQuit),
            // Control: Open command input modal
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
            // Action: Leave item
            KeyCode::Char('l') => {
                self.game.resolve_player_action(PlayerInput::DropItem(1));
            }
            // Action: Unequip Weapon
            KeyCode::Char('W') => {
                if let Err(e) = self.game.unequip_weapon() {
                    self.game.log.print(format!("{}", e));
                }
            }
            // Action: Unequip Armor
            KeyCode::Char('A') => {
                if let Err(e) = self.game.unequip_armor() {
                    self.game.log.print(format!("{}", e));
                }
            }

            // Control: Open Inventory (shifts focus to menu)
            KeyCode::Char('i') => {
                self.focus_menu(MenuMode::Inventory);
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
        match self.ui.menu.mode {
            MenuMode::Inventory => self.handle_inventory_key_event(key_event),
            MenuMode::Log => {}
        }

        match key_event.code {
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
            _ => {}
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
                ModalInterface::ConfirmUseItem { item_id } => match key_event.code {
                    KeyCode::Char('y') => {
                        let result = self.game.use_item(*item_id);
                        if let Err(e) = result {
                            self.game.log.print(format!("Cannot use item: {}", e));
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
                        self.ui.modal = Some(ModalInterface::ConfirmUseItem { item_id: *item_id });
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
