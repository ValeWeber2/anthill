#![allow(dead_code)]

use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

use crate::{
    App, KeyboardFocus,
    render::{menu_display::Menu, world_display::WorldDisplay},
};

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

        let outer_width = world_width_u16 + 2;
        let outer_height = world_height_u16 + 2;
        let area_worldspace = Layout::vertical([Constraint::Length(outer_height)])
            .horizontal_margin((area_world.width.saturating_sub(outer_width)) / 2)
            .vertical_margin((area_world.height.saturating_sub(outer_height)) / 2)
            .split(area_world)[0];

        // Character Info
        let block_character = Block::default().title("Character Info").borders(Borders::ALL);
        block_character.render(area_character, buf);

        // World
        let block_world = Block::default()
            .title("World")
            .border_style(if self.keyboard_focus == KeyboardFocus::FocusWorld {
                Style::default().fg(Color::LightBlue)
            } else {
                Style::default()
            })
            .borders(Borders::ALL);
        block_world.render(area_world, buf);

        // World Space
        // (Space actually occupied by tiles)
        let block_world = Block::default().title("World Space").borders(Borders::ALL);
        let block_world_inner = block_world.inner(area_worldspace);
        block_world.render(area_worldspace, buf);

        // Z-layer 0
        self.ui.world_display.render(&self.game.world, block_world_inner, buf);
        // Z-layer 1
        self.ui.world_display.render_items(&self.game.world.item_sprites, block_world_inner, buf);
        // Z-layer 2
        self.ui.world_display.render_npcs(&self.game.world.npcs, block_world_inner, buf);
        // Z-layer 3
        self.ui.world_display.render_player(&self.game.player.character, block_world_inner, buf);

        // Menu (Log, menus, tables)
        let block_menu = Block::default()
            .title("Menu")
            .border_style(if self.keyboard_focus == KeyboardFocus::FocusMenu {
                Style::default().fg(Color::LightBlue)
            } else {
                Style::default()
            })
            .borders(Borders::ALL);
        let block_menu_inner = block_menu.inner(area_menu);
        block_menu.render(area_menu, buf);

        self.ui.menu.render(&self.game, block_menu_inner, buf);
    }
}

pub struct UserInterface {
    pub menu: Menu,
    pub world_display: WorldDisplay,
}

impl UserInterface {
    pub fn new() -> Self {
        Self { menu: Menu::new(), world_display: WorldDisplay {} }
    }
}
