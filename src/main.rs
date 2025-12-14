mod core;
mod render;
mod world;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, style::Color};
use std::io;

use crate::{
    core::game::{BaseStats, GameItem, GameState, NpcStats},
    world::worldspace::Point,
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
}

impl App {
    fn new() -> Self {
        let mut game = GameState::new();

        game.world.add_wall_border();

        // Example: Spawn NPCs after game state is initialized
        for i in 1..10 {
            let _ = game.spawn_npc(
                format!("Goblin {}", i),
                Point::new(i, 5),
                'g',
                Color::Green,
                NpcStats { base: BaseStats { hp_max: 10, hp_current: 10 }, damage: 2 },
            );
        }

        // Example item
        let _ = game.spawn_item(
            "Rusty Sword".into(),
            Point::new(9, 5),
            '/',
            Color::White,
            GameItem::Weapon { name: "Rusty Sword".into(), damage: 5 },
        );

        Self { should_quit: false, game }
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
        let world = &self.game.world;
        let player = &mut self.game.player.character;

        match key_event.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('w') => player.move_by(0, -1, world),
            KeyCode::Char('s') => player.move_by(0, 1, world),
            KeyCode::Char('a') => player.move_by(-1, 0, world),
            KeyCode::Char('d') => player.move_by(1, 0, world),
            KeyCode::Char('p') => self.game.log.messages.push(format!(
                "Player with ID {} at position x: {}, y: {}",
                self.game.player.character.base.id,
                self.game.player.character.base.pos.x,
                self.game.player.character.base.pos.y
            )),
            _ => {}
        }
    }
}
