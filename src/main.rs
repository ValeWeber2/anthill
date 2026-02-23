mod ai;
mod core;
mod data;
mod proc_gen;
mod render;
mod util;
mod world;

use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::enable_raw_mode,
};
use ratatui::DefaultTerminal;

use crate::{core::game::GameState, render::ui::UserInterface, util::input_handler::KeyboardFocus};

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
    state: State,
}

#[derive(PartialEq)]
enum State {
    StartScreen,
    Playing,
    GameOver,
}

impl App {
    fn new() -> Self {
        let game = GameState::new();

        Self {
            should_quit: false,
            keyboard_focus: KeyboardFocus::FocusWorld,
            game,
            ui: UserInterface::new(),
            state: State::StartScreen,
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(std::io::stdout(), EnableMouseCapture,)?;

        while !self.should_quit {
            if self.state == State::Playing && !self.game.player.character.is_alive() {
                self.state = State::GameOver;
            }
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }

        execute!(std::io::stdout(), DisableMouseCapture,)?;

        Ok(())
    }

    fn restart(&mut self) {
        *self = App::new();
    }
}
