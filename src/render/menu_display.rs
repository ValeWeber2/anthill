#![allow(dead_code)]

use std::fmt;

use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};

use crate::core::game::GameState;

pub enum MenuMode {
    Log,
    Inventory,
}

impl fmt::Display for MenuMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuMode::Log => write!(f, "Log"),
            MenuMode::Inventory => write!(f, "Inventory"),
        }
    }
}

pub struct Menu {
    pub mode: MenuMode,
}

impl Menu {
    pub fn new() -> Self {
        Self { mode: MenuMode::Log }
    }
    pub fn render(&self, game_state: &GameState, rect: Rect, buf: &mut Buffer) {
        match self.mode {
            MenuMode::Log => self.render_log(&game_state.log.messages, rect, buf),
            MenuMode::Inventory => self.render_inventory(game_state, rect, buf),
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

    pub fn render_inventory(&self, game_state: &GameState, rect: Rect, buf: &mut Buffer) {
        let inventory = &game_state.player.character.inventory;

        let height = rect.height as usize;
        let start = inventory.len().saturating_sub(height);

        let item_list_def_ids: Vec<String> = inventory
            .iter()
            .map(|item_id| {
                if let Some(game_item) = game_state.get_item_by_id(*item_id) {
                    game_item.def_id.clone()
                } else {
                    "Unregistered Item".to_string()
                }
            })
            .collect();

        let item_list_names: Vec<String> = item_list_def_ids
            .iter()
            .map(|item_id| {
                if let Some(item_def) = game_state.get_item_def_by_id(item_id) {
                    item_def.name.clone()
                } else {
                    "Unknown Item".to_string()
                }
            })
            .collect();

        let lines: Vec<Line> = item_list_names[start..]
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let list_letter = (b'a' + i as u8) as char;
                Line::raw(format!("{list_letter} - {}", item))
            })
            .collect();

        let paragraph = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: true });
        paragraph.render(rect, buf);
    }
}
