mod core;
mod render;
mod world;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::DefaultTerminal;

use crate::core::player::Player;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    should_quit: bool,
    player: Player,
}

impl App {
    fn new() -> Self {
        Self { 
            should_quit: false, 
            player: Player::new(),
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
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('w') => self.player.character.move_by(0, -1),
            KeyCode::Char('s') => self.player.character.move_by(0, 1),
            KeyCode::Char('a') => self.player.character.move_by(-1, 0),
            KeyCode::Char('d') => self.player.character.move_by(1, 0),
            _ => {}
        }
    }
}
