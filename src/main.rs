mod core;
mod render;
mod world;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;

use crate::core::game::GameState;

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
        Self { should_quit: false, game: GameState::new() }
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
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('w') => self.game.player.character.move_by(0, -1),
            KeyCode::Char('s') => self.game.player.character.move_by(0, 1),
            KeyCode::Char('a') => self.game.player.character.move_by(-1, 0),
            KeyCode::Char('d') => self.game.player.character.move_by(1, 0),
            KeyCode::Char('p') => self.game.log.messages.push(format!("Player at position x: {}, y: {}", self.game.player.character.base.pos.x, self.game.player.character.base.pos.y)),
            _ => {}
        }
    }
}
