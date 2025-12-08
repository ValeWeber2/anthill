use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

use crate::App;

use crate::world::worldspace::{WORLD_HEIGHT, WORLD_WIDTH};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let world_width_u16: u16 = WORLD_WIDTH.try_into().unwrap();
        let world_height_u16: u16 = WORLD_HEIGHT.try_into().unwrap();

        let layout_top_bottom = Layout::vertical([Constraint::Min(0), Constraint::Length(4)]);
        let [area_game, area_character] = layout_top_bottom.areas(area);

        let layout_left_right = Layout::horizontal([
            Constraint::Percentage(70),
            Constraint::Length(1),
            Constraint::Percentage(30),
        ]);
        let [area_world, _empty, area_menu] = layout_left_right.areas(area_game);

        let area_world_inner = Layout::vertical([Constraint::Length(world_height_u16)])
            .horizontal_margin((area_world.width.saturating_sub(world_width_u16)) / 2)
            .vertical_margin((area_world.height.saturating_sub(world_height_u16)) / 2)
            .split(area_world)[0];

        let block_character = Block::default().title("Character Info").borders(Borders::ALL);
        block_character.render(area_character, buf);

        let block_world_outer = Block::default().title("World").borders(Borders::ALL);
        block_world_outer.render(area_world, buf);

        let block_world = Block::default().title("World Space").borders(Borders::ALL);
        block_world.render(area_world_inner, buf);

        let block_menu = Block::default().title("Menu").borders(Borders::ALL);
        block_menu.render(area_menu, buf);
    }
}
