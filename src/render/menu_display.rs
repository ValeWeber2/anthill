#![allow(dead_code)]

use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};

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
