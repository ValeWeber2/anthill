mod core;
mod render;
mod world;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, style::Color};
use std::io;

use crate::{
    core::game::{BaseStats, GameItem, GameState, NpcStats},
    render::ui::UserInterface,
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

        // Example item
        let _ = game.spawn_item(
            "Rusty Sword".into(),
            Point::new(50, 11),
            '/',
            Color::White.into(),
            GameItem::Weapon { name: "Rusty Sword".into(), damage: 5 },
        );

        // Example: Spawning on a Wall Tile
        let _ = game.spawn_npc(
            "Funny Frog".into(),
            Point::new(35, 7),
            'f',
            Color::LightGreen.into(),
            NpcStats { base: BaseStats { hp_max: 5, hp_current: 5 }, damage: 0 },
        );

        Self { should_quit: false, game, ui: UserInterface::new() }
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
        match key_event.code {
            KeyCode::Char('q') => self.should_quit = true,
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
            KeyCode::Char('p') => self.game.log.messages.push(format!(
                "Player at position x: {}, y: {}",
                self.game.player.character.base.pos.x, self.game.player.character.base.pos.y
            )),
            _ => {}
        }
    }
}
