mod ai;
mod core;
mod data;
mod render;
mod util;
mod world;

use std::io;

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
}

impl App {
    fn new() -> Self {
        let game = GameState::new();
        // game.world.carve_room(&Room::new(Point { x: 35, y: 5 }, 30, 15));

        /*

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
        */

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
}
