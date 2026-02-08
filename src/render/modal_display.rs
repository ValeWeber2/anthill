#![allow(dead_code)]

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Borders, Clear, Padding, Paragraph, Row, Table, Wrap},
};

use crate::{
    core::{game::GameState, game_items::GameItemId},
    render::ui::get_centered_rect,
    util::command_handler::GameCommand,
    world::coordinate_system::Point,
};

pub enum ModalInterface {
    ConfirmQuit,
    ConfirmChooseItem { item_id: GameItemId },
    CommandInput { buffer: String },
    TextDisplay { title: String, paragraphs: Vec<String> },
    HelpDisplay,
    SelectPrompt { selection_action: SelectionAction, options: Vec<String> },
}

impl ModalInterface {
    /// Central handling for rendering modals.
    ///
    /// Switches to the [ModalInterface] kind that is open at the time.
    pub fn render(&self, rect: Rect, buf: &mut Buffer, game: &GameState) {
        match self {
            ModalInterface::ConfirmQuit => render_confirm_quit(rect, buf),
            ModalInterface::ConfirmChooseItem { item_id } => {
                render_confirm_select_item(rect, buf, game, *item_id)
            }
            ModalInterface::CommandInput { buffer } => render_command_input(buffer, rect, buf),
            ModalInterface::TextDisplay { title, paragraphs } => {
                render_text_display(title, paragraphs, rect, buf)
            }
            ModalInterface::HelpDisplay => render_help(rect, buf),
            ModalInterface::SelectPrompt { selection_action, options } => {
                render_select_prompt(rect, buf, selection_action, options)
            }
        }
    }
}

/// Displays a large modal window that can contain multiple text paragraphs.
pub fn render_text_display(title: &str, paragraphs: &[String], rect: Rect, buf: &mut Buffer) {
    // Making the Window
    let modal_area = render_modal_window(150, 33, title.to_string(), rect, buf);

    let page_text = Text::from(
        paragraphs.iter().map(|paragraph| Line::from(paragraph.as_str())).collect::<Vec<Line>>(),
    );

    Paragraph::new(page_text).render(modal_area, buf);
}

/// Displays the dialog where the user has to confirm that they want to quit the game.
fn render_confirm_quit(rect: Rect, buf: &mut Buffer) {
    // Making the Window
    let modal_area = render_modal_window(50, 5, " Confirm Quit ".to_string(), rect, buf);

    // Filling the Window
    let text = Text::from(vec![
        Line::from("Do you really want to quit?"),
        Line::from(""),
        Line::from("Press <q> again"),
    ]);

    let center_of_rect = get_centered_rect(50, 3, modal_area);

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    paragraph.render(center_of_rect, buf);
}

/// Displays the dialog where the user has to confirm the item that they selected (e.g. for using or dropping)
fn render_confirm_select_item(rect: Rect, buf: &mut Buffer, game: &GameState, item_id: GameItemId) {
    let modal_area = render_modal_window(50, 5, " Confirm Action ".to_string(), rect, buf);

    // look up item name
    let instance = &game.items[&item_id];
    let item_name = game
        .get_item_def_by_id(instance.def_id.clone())
        .map(|def| def.name)
        .unwrap_or("<unknown item>");

    let text = Text::from(vec![
        Line::from(format!("Selected: {}", item_name)),
        Line::from(""),
        Line::from("Press <y> to confirm, <n> to cancel"),
    ]);

    let center_of_rect = get_centered_rect(50, 3, modal_area);

    Paragraph::new(text).alignment(Alignment::Center).render(center_of_rect, buf);
}

/// Displays the dialog into which you can enter game commands to execute.
fn render_command_input(buffer: &str, rect: Rect, buf: &mut Buffer) {
    // Making the Window
    let modal_area = render_modal_window(50, 5, " Execute a Command ".to_string(), rect, buf);

    // Filling the window
    let input_area = Rect {
        x: modal_area.x + (modal_area.width.saturating_sub(30_u16)) / 2,
        y: modal_area.y + (modal_area.height.saturating_sub(5_u16)) / 2,
        width: 30,
        height: 3,
    };
    let input_block = Block::default().borders(Borders::ALL);
    let input_block_inner = input_block.inner(input_area);
    input_block.render(input_area, buf);

    let text = Text::from(buffer);

    let paragraph = Paragraph::new(text);
    paragraph.render(input_block_inner, buf);
}

/// Helper function that does the setup for a modal window.
///
/// It creates a rect that is centered and has its background cleared (so it is "above" the background).
fn render_modal_window(
    width: u16,
    height: u16,
    title: String,
    rect: Rect,
    buf: &mut Buffer,
) -> Rect {
    let area_modal = get_centered_rect(width, height, rect);

    Clear.render(area_modal, buf);

    let block_modal = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_set(border::DOUBLE);

    let block_modal_inner = block_modal.inner(area_modal);

    block_modal.render(area_modal, buf);

    block_modal_inner
}

/// Displays the help display, which explains the basics of controls in the game.
fn render_help(area: Rect, buf: &mut Buffer) {
    let center = get_centered_rect(150, 33, area);

    let block =
        Block::default().borders(Borders::ALL).title(" Help ").padding(Padding::new(1, 1, 1, 1));

    let inner = block.inner(center);

    Clear.render(center, buf);
    block.render(center, buf);

    // Data

    const CONTROLS_WIDTHS: [Constraint; 4] = [
        Constraint::Percentage(19),
        Constraint::Percentage(27),
        Constraint::Percentage(27),
        Constraint::Percentage(27),
    ];

    let controls_rows = [
        Row::new(vec![""]),
        Row::new(vec!["Start / Quit:", "ENTER - start game", "Q - quit game", "ESC - close menus"]),
        Row::new(vec!["Movement:", "w - up, a - left, s - down, d - right", ". - wait one turn"]),
        Row::new(vec!["Interaction:", "Walk into an NPC or item to interact"]),
        Row::new(vec![
            "Inventory:",
            "i - open inventory",
            "D - open inventory in drop mode",
            "a, b, c… - select item",
        ]),
        Row::new(vec!["Actions:", "W - unequip weapon", "A - unequip armor"]),
        Row::new(vec![
            "Command Input:",
            ": - open command prompt",
            "ENTER - run command",
            "ESC - cancel",
        ]),
        Row::new(vec![""]),
    ];

    const COMMAND_WIDTHS: [Constraint; 2] =
        [Constraint::Percentage(19), Constraint::Percentage(81)];

    let player_commands = [GameCommand::Quit, GameCommand::Help, GameCommand::PlayerInfo];

    let mut player_command_rows = Vec::with_capacity(player_commands.len() + 2);
    player_command_rows.push(Row::new(vec![""]));

    for cmd in player_commands {
        player_command_rows
            .push(Row::new(vec![cmd.name().to_string(), cmd.description().to_string()]));
    }

    player_command_rows.push(Row::new(vec![""]));

    let dev_commands = [
        GameCommand::MaxStats,
        GameCommand::MaxEquip,
        GameCommand::RngTest,
        GameCommand::Suicide,
        GameCommand::Teleport(Point::new(0, 0)), // dummy
        GameCommand::Give { item_def: "".into(), amount: 0 }, // dummy
        GameCommand::RevealAll,
    ];

    let mut dev_command_rows = Vec::with_capacity(dev_commands.len() + 3);
    dev_command_rows.push(Row::new(vec![""]));

    for cmd in dev_commands {
        dev_command_rows
            .push(Row::new(vec![cmd.name().to_string(), cmd.description().to_string()]));
    }

    dev_command_rows.push(Row::new(vec![""]));
    dev_command_rows.push(Row::new(vec![""]));

    // Layout

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),                                // controls header
            Constraint::Length(controls_rows.len() as u16),       // controls table
            Constraint::Length(1),                                // player commands header
            Constraint::Length(player_command_rows.len() as u16), // player commands table
            Constraint::Length(1),                                // dev commands header
            Constraint::Length(dev_command_rows.len() as u16),    // dev commands table
            Constraint::Length(1),                                // footer
        ])
        .split(inner);

    // Rendering

    Paragraph::new("=== BASIC CONTROLS ==================================================================================================================================")
        .render(chunks[0], buf);

    let controls_table = Table::new(controls_rows, CONTROLS_WIDTHS).column_spacing(1);

    Widget::render(controls_table, chunks[1], buf);

    Paragraph::new("=== PLAYER COMMANDS =================================================================================================================================")
        .render(chunks[2], buf);

    let player_command_table = Table::new(player_command_rows, COMMAND_WIDTHS).column_spacing(1);

    Widget::render(player_command_table, chunks[3], buf);

    Paragraph::new("=== DEVELOPER COMMANDS ==============================================================================================================================")
        .render(chunks[4], buf);

    let dev_command_table = Table::new(dev_command_rows, COMMAND_WIDTHS).column_spacing(1);

    Widget::render(dev_command_table, chunks[5], buf);

    Paragraph::new("Press ESC to close this window")
        .style(Style::default().add_modifier(Modifier::DIM))
        .render(chunks[6], buf);
}

pub enum SelectionAction {
    Debug,
}

fn render_select_prompt(
    rect: Rect,
    buf: &mut Buffer,
    selection_action: &SelectionAction,
    options: &[String],
) {
    let instruction = match selection_action {
        SelectionAction::Debug => "Choose a message to be displayed".to_string(),
    };

    let modal_area_width = instruction.len() as u16 + 4;
    let modal_area_height = options.len() as u16 + 5;
    let modal_area =
        render_modal_window(modal_area_width, modal_area_height, "Select".to_string(), rect, buf);
    let center_of_rect = get_centered_rect(modal_area_width, modal_area_height, modal_area);

    let mut lines: Vec<Line> = vec![Line::raw(instruction), Line::raw("")];
    for (i, option) in options.iter().enumerate() {
        let list_letter = (b'a' + i as u8) as char;
        lines.push(Line::raw(format!("{} - {}", list_letter, option)));
    }

    let paragraph =
        Paragraph::new(Text::from(lines)).alignment(Alignment::Center).wrap(Wrap { trim: true });
    paragraph.render(center_of_rect, buf);
}
