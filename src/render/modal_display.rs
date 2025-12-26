#![allow(dead_code)]

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Borders, Clear, Paragraph},
};

pub enum ModalInterface {
    ConfirmQuit,
    CommandInput { buffer: String },
    TextDisplay { title: String, paragraphs: Vec<String> },
}

impl ModalInterface {
    pub fn render(&self, rect: Rect, buf: &mut Buffer) {
        match self {
            ModalInterface::ConfirmQuit => render_confirm_quit(rect, buf),
            ModalInterface::CommandInput { buffer } => render_command_input(buffer, rect, buf),
            ModalInterface::TextDisplay { title, paragraphs } => {
                render_text_display(title, paragraphs, rect, buf)
            }
        }
    }
}

fn render_text_display(title: &str, paragraphs: &[String], rect: Rect, buf: &mut Buffer) {
    // Making the Window
    let modal_area = render_modal_window(150, 30, title.to_string(), rect, buf);

    let page_text = Text::from(
        paragraphs.iter().map(|paragraph| Line::from(paragraph.as_str())).collect::<Vec<Line>>(),
    );

    let paragraph = Paragraph::new(page_text);
    paragraph.render(modal_area, buf);
}

fn render_confirm_quit(rect: Rect, buf: &mut Buffer) {
    // Making the Window
    let modal_area = render_modal_window(50, 5, "Confirm Quit".to_string(), rect, buf);

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

fn render_command_input(buffer: &str, rect: Rect, buf: &mut Buffer) {
    // Making the Window
    let modal_area = render_modal_window(50, 5, "Execute a Command".to_string(), rect, buf);

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
