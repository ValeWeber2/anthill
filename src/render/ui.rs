#![allow(dead_code)]

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};

use crate::{
    App, KeyboardFocus, State,
    core::{entity_logic::Entity, game::GameState},
    data::ascii_art::{GRAVESTONE, STARTSCREEN_ASCII},
    render::{menu_display::Menu, modal_display::ModalInterface, world_display::WorldDisplay},
};
use crate::{
    render::info_display::InfoDisplay,
    world::worldspace::{WORLD_HEIGHT, WORLD_WIDTH},
};

const MIN_WIDTH: u16 = 150;
const MIN_HEIGHT: u16 = 33; // Technically just 30

impl Widget for &App {
    /// Implements [Widget] trait for the App.
    /// The area is divided into sub-areas first and then filled with the render output of the components.
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Size Check
        if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
            render_window_size_warning(area, buf);
        } else {
            match self.state {
                State::StartScreen => {
                    render_start_screen(area, buf);
                }
                State::Playing => {
                    self.render_game(area, buf);
                }
                State::GameOver => {
                    render_game_over(area, buf, &self.game);
                }
            }
        }

        // Modal
        if let Some(modal) = &self.ui.modal {
            modal.render(area, buf, &self.game);
        }
    }
}

impl App {
    fn render_game(&self, rect: Rect, buf: &mut Buffer) {
        // Normal
        let world_width_u16: u16 = WORLD_WIDTH.try_into().unwrap();
        let world_height_u16: u16 = WORLD_HEIGHT.try_into().unwrap();

        // Layout from top to bottom. Divided into:
        // +-------------------------+
        // |                         |
        // | World + Menu            |
        // |                         |
        // +-------------------------+
        // | Info Display            |
        // +-------------------------+
        let layout_top_bottom = Layout::vertical([Constraint::Min(0), Constraint::Length(4)]);
        let [area_game, area_info] = layout_top_bottom.areas(rect);

        // +----------------+--------+
        // |                |        |
        // | World          | Menu   |
        // |                |        |
        // +----------------+--------+
        let layout_left_right = Layout::horizontal([
            Constraint::Percentage(70),
            Constraint::Length(1),
            Constraint::Percentage(30),
        ]);
        let [area_world, _empty, area_menu] = layout_left_right.areas(area_game);

        // Calculating automatic padding for fixed-size worldspace
        let outer_width = world_width_u16 + 2;
        let outer_height = world_height_u16 + 2;
        let area_worldspace = Layout::vertical([Constraint::Length(outer_height)])
            .horizontal_margin((area_world.width.saturating_sub(outer_width)) / 2)
            .vertical_margin((area_world.height.saturating_sub(outer_height)) / 2)
            .split(area_world)[0];

        // AREA: Character Info
        let block_info = Block::default().title(" Character Info ").borders(Borders::ALL);
        let block_info_inner = block_info.inner(area_info);
        block_info.render(area_info, buf);

        self.ui.info.render(&self.game, block_info_inner, buf);

        // AREA: World
        let block_world = Block::default()
            .title(" World ")
            .border_style(if self.keyboard_focus == KeyboardFocus::FocusWorld {
                Style::default().fg(Color::LightBlue)
            } else {
                Style::default()
            })
            .borders(Borders::ALL);
        block_world.render(area_world, buf);

        // AREA: World Space
        // (Space actually occupied by tiles)
        let block_world = Block::default().title(" World Space ").borders(Borders::ALL);
        let block_world_inner = block_world.inner(area_worldspace);
        block_world.render(area_worldspace, buf);

        // Z-layer 0
        self.ui.world_display.render(&self.game, block_world_inner, buf);
        // Z-layer 1
        self.ui.world_display.render_items(&self.game, block_world_inner, buf);
        // Z-layer 2
        self.ui.world_display.render_npcs(&self.game, block_world_inner, buf);
        // Z-layer 3
        self.ui.world_display.render_player(&self.game.player.character, block_world_inner, buf);
        // Z-layer 4
        self.ui.world_display.render_cursor(&self.game, block_world_inner, buf);

        // AREA: Menu (Log, menus, tables)
        let block_menu = Block::default()
            .title(format!(" Menu:{} ", self.ui.menu.mode))
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

/// Renders a text warning in the middle of the screen, which also blanks the background.
///
/// # Note
/// The game is still accessible while a warning is displayed, meaning a player can still make inputs (e.g. 'q', 'wasd')
fn render_warning(text: String, rect: Rect, buf: &mut Buffer) {
    let center_rect = get_centered_rect(50, 10, rect);
    let paragraph = Paragraph::new(Text::from(text))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(" Warning ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );

    paragraph.render(center_rect, buf);
}

fn render_window_size_warning(rect: Rect, buf: &mut Buffer) {
    render_warning(
        format!(
            "Your Terminal window is too small.\nIn order to play the game, your Terminal must at least have the dimensions of {}x{} characters.\n(Current {}x{})\n\nIncrease window size of your terminal or decrease your font size (Ctrl + -) to continue.",
            MIN_WIDTH, MIN_HEIGHT, rect.width, rect.height,
        ),
        rect,
        buf,
    );
}

/// Struct representing the UI state.
pub struct UserInterface {
    /// State of the Menu (which contains log, inventory, ...)
    pub menu: Menu,

    /// Empty struct to hold the render method for the world display.
    pub world_display: WorldDisplay,

    /// Optional Modal interface. By default `None`, but if a modal interface is displayed, it becomes `Some(_)`
    pub modal: Option<ModalInterface>,

    /// Empty struct to hold the render method for the info display.
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
///
/// Use this method to center a window to the view.
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

/// Render the main menu screen that is displayed when starting the game.
fn render_start_screen(area: Rect, buf: &mut Buffer) {
    let center_rect = get_centered_rect(150, 33, area);
    let block = Block::default().borders(Borders::NONE);

    let block_inner = block.inner(center_rect);

    block.render(center_rect, buf);

    Paragraph::new(Text::from(STARTSCREEN_ASCII)).render(block_inner, buf);
}

/// Render the Game Over Screen that appears when you lose the game (when the player character die).
fn render_game_over(area: Rect, buf: &mut Buffer, game: &GameState) {
    Block::default().borders(Borders::ALL).title(" Game Over ").render(area, buf);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left = columns[0];
    let right = columns[1];

    let art_height = GRAVESTONE.lines().count() as u16;

    let left_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min((left.height.saturating_sub(art_height)) / 2),
            Constraint::Length(art_height),
            Constraint::Min(0),
        ])
        .split(left);

    Paragraph::new(GRAVESTONE)
        .alignment(Alignment::Right)
        .block(Block::default().padding(Padding::new(10, 0, 0, 0)))
        .render(left_vertical[1], buf);

    let lines = [
        format!("Goodbye, {}", game.player.character.name()),
        "You have died in the Anthill".into(),
        "".into(),
        "Press ENTER to start a new game".into(),
        "Press Q to quit".into(),
    ];

    let text = Text::from(lines.iter().map(|l| Line::from(l.as_str())).collect::<Vec<Line>>());

    let right_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Min(0), Constraint::Percentage(40)])
        .split(right);

    Paragraph::new(text).alignment(Alignment::Left).render(right_vertical[1], buf);
}
