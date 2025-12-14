#![allow(dead_code)]

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
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

        // Character Info
        let block_character = Block::default().title("Character Info").borders(Borders::ALL);
        block_character.render(area_character, buf);

        // World
        let block_world_outer = Block::default().title("World").borders(Borders::ALL);
        block_world_outer.render(area_world, buf);

        // World Space
        // (Space actually occupied by tiles)
        let block_world = Block::default().title("World Space").borders(Borders::ALL);
        block_world.render(area_world_inner, buf);

        // Menu (Log, menus, tables)
        let block_menu = Block::default().title("Menu").borders(Borders::ALL);
        let block_menu_inner = block_menu.inner(area_menu);
        block_menu.render(area_menu, buf);

        self.ui.menu.render(
            MenuData {
                log: &self.game.log.messages,
                inventory: &[], // TODO Inventory
            },
            block_menu_inner,
            buf,
        );
    }
}

pub struct UserInterface {
    pub menu: Menu,
}

impl UserInterface {
    pub fn new() -> Self {
        Self { menu: Menu::new() }
    }
}

pub enum MenuMode {
    Log,
    Inventory,
}

pub struct MenuData<'a> {
    pub log: &'a [String],
    pub inventory: &'a [String],
}

pub struct Menu {
    pub mode: MenuMode,
}

impl Menu {
    pub fn new() -> Self {
        Self { mode: MenuMode::Log }
    }
    pub fn render(&self, data: MenuData<'_>, rect: Rect, buf: &mut Buffer) {
        match self.mode {
            MenuMode::Log => self.render_log(data.log, rect, buf),
            MenuMode::Inventory => self.render_inventory(data.inventory, rect, buf),
        }
    }
}

impl Menu {
    pub fn render_log(&self, messages: &[String], rect: Rect, buf: &mut Buffer) {
        let height = rect.height as usize;
        let start = messages.len().saturating_sub(height);

        let lines: Vec<Line> =
            messages[start..].iter().map(|msg| Line::raw(msg.as_str())).collect();

        let paragraph = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: true });
        paragraph.render(rect, buf);
    }
    pub fn render_inventory(&self, _inventory: &[String], rect: Rect, buf: &mut Buffer) {
        let height = rect.height as usize;
        let inventory_mock = ["Apple", "Sword"];
        let start = inventory_mock.len().saturating_sub(height);

        let lines: Vec<Line> =
            inventory_mock[start..].iter().map(|item| Line::raw(item.to_string())).collect();

        let paragraph = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: true });
        paragraph.render(rect, buf);
    }
}
