mod core;
mod data;
mod render;
mod util;
mod world;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, style::Color};
use std::io;

use crate::{
    core::{
        entity_logic::{BaseStats, NpcStats},
        game::GameState,
    },
    render::{menu_display::MenuMode, modal_display::ModalInterface, ui::UserInterface},
    world::worldspace::{Point, Room},
};

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    should_quit: bool,
    keyboard_focus: KeyboardFocus,
    game: GameState,
    ui: UserInterface,
}

impl App {
    fn new() -> Self {
        let mut game = GameState::new();

        game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        // Example: Spawn NPC after game state is initialized
        let _ = game.spawn_npc(
            "Goblin".into(),
            Point::new(50, 10),
            'g',
            Color::Green.into(),
            NpcStats { base: BaseStats { hp_max: 10, hp_current: 10 }, damage: 2 },
        );

        // Example: item in world
        let example_sword_def_id: &'static str = "weapon_sword_rusty";
        let example_sword_id = game.register_item(example_sword_def_id);
        let _ = game.spawn_item(example_sword_id, Point::new(50, 11));

        // Example: item in inventory
        let example_food_dev_id: &'static str = "food_cake";
        let example_food_id = game.register_item(example_food_dev_id);
        let _ = game.add_item_to_inv(example_food_id);

        let example_armor_dev_id: &'static str = "armor_leather";
        let example_armor_id = game.register_item(example_armor_dev_id);
        let _ = game.add_item_to_inv(example_armor_id);

        // Example: Spawning on a Wall Tile
        let _ = game.spawn_npc(
            "Funny Frog".into(),
            Point::new(35, 7),
            'f',
            Color::LightGreen.into(),
            NpcStats { base: BaseStats { hp_max: 5, hp_current: 5 }, damage: 0 },
        );

        Self {
            should_quit: false,
            keyboard_focus: KeyboardFocus::FocusWorld,
            game,
            ui: UserInterface::new(),
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
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
                self.game.world.move_entity(&mut self.game.player.character, 0, -1)
            }
            KeyCode::Char('s') => {
                self.game.world.move_entity(&mut self.game.player.character, 0, 1)
            }
            KeyCode::Char('a') => {
                self.game.world.move_entity(&mut self.game.player.character, -1, 0)
            }
            KeyCode::Char('d') => {
                self.game.world.move_entity(&mut self.game.player.character, 1, 0)
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
                    self.game
                        .log
                        .messages
                        .push(format!("Item ID: {} DEF: {}", item_id, item.def_id,))
                }
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
            _ => {}
        }
    }

    fn handle_menu_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.should_quit = true,
            // It is currently allowed to manually switch focus. This will later be handled by the game directly.
            KeyCode::Tab => self.keyboard_focus = self.keyboard_focus.cycle(),
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
}

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

pub enum ModalAction {
    Idle,
    CloseModal,
    RunCommand(String),
}
