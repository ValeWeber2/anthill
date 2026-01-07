use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

use crate::{
    App, ModalAction,
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

impl KeyboardFocus {
    pub fn cycle(self) -> Self {
        match self {
            Self::FocusWorld => Self::FocusMenu,
            Self::FocusMenu => Self::FocusWorld,
        }
    }
}

impl App {
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
        if self.ui.modal.is_some() {
            self.handle_modal_key_event(key_event);
            return;
        }
        match self.keyboard_focus {
            KeyboardFocus::FocusWorld => self.handle_world_key_event(key_event),
            KeyboardFocus::FocusMenu => self.handle_menu_key_event(key_event),
        }
    }

    fn handle_world_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.ui.modal = Some(ModalInterface::ConfirmQuit),
            // It is currently allowed to manually switch focus. This will later be handled by the game directly.
            KeyCode::Tab => self.keyboard_focus = self.keyboard_focus.cycle(),
            KeyCode::Char('w') => {
                self.game.resolve_player_action(PlayerInput::Direction(Direction::Up));
                // self.game.world.move_entity(&mut self.game.player.character, 0, -1)
            }
            KeyCode::Char('s') => {
                self.game.resolve_player_action(PlayerInput::Direction(Direction::Down));
            }
            KeyCode::Char('a') => {
                self.game.resolve_player_action(PlayerInput::Direction(Direction::Left));
            }
            KeyCode::Char('d') => {
                self.game.resolve_player_action(PlayerInput::Direction(Direction::Right));
            }
            KeyCode::Char('.') => {
                self.game.resolve_player_action(PlayerInput::Wait);
            }
            KeyCode::Char(':') => {
                self.ui.modal = Some(ModalInterface::CommandInput { buffer: "".to_string() })
            }
            KeyCode::Char('p') => self.game.log.print(format!(
                "Player at position x: {}, y: {}",
                self.game.player.character.base.pos.x, self.game.player.character.base.pos.y
            )),
            KeyCode::Char('o') => {
                for (item_id, item) in self.game.items.iter() {
                    self.game.log.print(format!("Item ID: {} DEF: {}", item_id, item.def_id))
                }
            }
            KeyCode::Char('l') => {
                self.game.resolve_player_action(PlayerInput::DropItem(1));
            }
            KeyCode::Char('i') => match self.ui.menu.mode {
                MenuMode::Log => self.ui.menu.mode = MenuMode::Inventory,
                MenuMode::Inventory => self.ui.menu.mode = MenuMode::Log,
            },
            KeyCode::Char('9') => {
                self.ui.modal = Some(ModalInterface::TextDisplay {
                    title: "Test Display".to_string(),
                    paragraphs: vec![
                        "Das ist ein Test".to_string(),
                        "Hier ein weiterer Paragraph".to_string(),
                    ],
                })
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
            _ => {}
        }
    }

    fn handle_menu_key_event(&mut self, key_event: KeyEvent) {
        match self.ui.menu.mode {
            MenuMode::Inventory => self.handle_inventory_key_event(key_event),
            MenuMode::Log => self.handle_log_key_event(key_event),
        }

        match key_event.code {
            KeyCode::Char('q') => self.ui.modal = Some(ModalInterface::ConfirmQuit),
            KeyCode::Char('i') => match self.ui.menu.mode {
                MenuMode::Log => self.ui.menu.mode = MenuMode::Inventory,
                MenuMode::Inventory => self.ui.menu.mode = MenuMode::Log,
            },
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
                        self.ui.menu.mode = MenuMode::Log;
                        self.keyboard_focus = KeyboardFocus::FocusWorld;

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
            KeyCode::Tab => {
                self.keyboard_focus = KeyboardFocus::FocusWorld;
                self.ui.menu.mode = MenuMode::Log;
            }
            KeyCode::Esc => {
                self.ui.menu.mode = MenuMode::Log;
                self.keyboard_focus = KeyboardFocus::FocusWorld;
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

    fn handle_log_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Tab => {
                self.keyboard_focus = KeyboardFocus::FocusWorld;
            }

            KeyCode::Esc => {
                self.keyboard_focus = KeyboardFocus::FocusWorld;
            }
            _ => {}
        }
    }
}
