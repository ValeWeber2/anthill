#![allow(dead_code)]

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{
    App, KeyboardFocus,
    render::{menu_display::Menu, modal_display::ModalInterface, world_display::WorldDisplay},
};
use crate::{
    render::info_display::InfoDisplay,
    world::worldspace::{WORLD_HEIGHT, WORLD_WIDTH},
};

const MIN_WIDTH: u16 = 150;
const MIN_HEIGHT: u16 = 33;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Size Check
        if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
            render_warning(
                format!(
                    "Your Terminal window is too small.\nIn order to play the game, your Terminal must at least have the dimensions of {}x{} characters.\n(Current {}x{})",
                    MIN_WIDTH, MIN_HEIGHT, area.width, area.height,
                ),
                area,
                buf,
            );
            return;
        }

        // Normal
        let world_width_u16: u16 = WORLD_WIDTH.try_into().unwrap();
        let world_height_u16: u16 = WORLD_HEIGHT.try_into().unwrap();

        let layout_top_bottom = Layout::vertical([Constraint::Min(0), Constraint::Length(4)]);
        let [area_game, area_info] = layout_top_bottom.areas(area);

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
        let block_info = Block::default().title("Character Info").borders(Borders::ALL);
        let block_info_inner = block_info.inner(area_info);
        block_info.render(area_info, buf);

        self.ui.info.render(&self.game, block_info_inner, buf);

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
            .title(format!("Menu─({})", self.ui.menu.mode))
            .border_style(if self.keyboard_focus == KeyboardFocus::FocusMenu {
                Style::default().fg(Color::LightBlue)
            } else {
                Style::default()
            })
            .borders(Borders::ALL);
        let block_menu_inner = block_menu.inner(area_menu);
        block_menu.render(area_menu, buf);

        self.ui.menu.render(&self.game, block_menu_inner, buf);

        // Modal
        if let Some(modal) = &self.ui.modal {
            modal.render(area, buf, &self.game);
        }
    }
}

fn render_warning(text: String, rect: Rect, buf: &mut Buffer) {
    let center_rect = get_centered_rect(50, 8, rect);
    let paragraph = Paragraph::new(Text::from(text))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Warning")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );

    paragraph.render(center_rect, buf);
}

pub struct UserInterface {
    pub menu: Menu,
    pub world_display: WorldDisplay,
    pub modal: Option<ModalInterface>,
    pub info: InfoDisplay,
}

impl UserInterface {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(),
            world_display: WorldDisplay {},
            modal: None,
            info: InfoDisplay::new(),
        }
    }
}

/// Creates a new, centered Rect of a given width and height in the given area.
pub fn get_centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(area.height.saturating_sub(height) / 2),
            Constraint::Length(height),
            Constraint::Length(area.height.saturating_sub(height) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((area.width.saturating_sub(width)) / 2),
            Constraint::Length(width),
            Constraint::Length((area.width.saturating_sub(width)) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}
