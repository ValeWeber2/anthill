#![allow(dead_code)]

use std::fmt;

use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};

use crate::{
    core::{game::GameState, game_items::GameItemKindDef},
    data::item_defs::GameItemDef,
};

/// Different display modes for the menu
#[derive(Debug, Clone, Copy)]
pub enum MenuMode {
    /// Displaying the game log
    ///
    /// In this mode, the menu cannot be focused and there are no interactions with the log.
    Log,

    /// Displaying the player character's inventory.
    ///
    /// The inventory can be opened in different modes ([InventoryAction]), which are passed as an argument.
    Inventory(InventoryAction),
}

/// Different modes to use the inventory (Use or Drop)
#[derive(Debug, Clone, Copy)]
pub enum InventoryAction {
    /// The inventory is open with the intention of using an item.
    Use,

    /// The inventory is open with the intention of dropping an item.
    Drop,
}

impl fmt::Display for MenuMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuMode::Log => write!(f, "Log"),
            MenuMode::Inventory(InventoryAction::Use) => write!(f, "Inventory (use)"),
            MenuMode::Inventory(InventoryAction::Drop) => write!(f, "Inventory (drop)"),
        }
    }
}

/// Menu struct containing the state of the menu in the app.
pub struct Menu {
    pub mode: MenuMode,
}

impl Menu {
    pub fn new() -> Self {
        Self { mode: MenuMode::Log }
    }

    /// Renders the menu. Switches between log display and inventory display depending on state.
    pub fn render(&self, game_state: &GameState, rect: Rect, buf: &mut Buffer) {
        match self.mode {
            MenuMode::Log => self.render_log(game_state, rect, buf),
            MenuMode::Inventory(_) => self.render_inventory(game_state, rect, buf),
        }
    }

    /// Renders the menu in log mode.
    pub fn render_log(&self, game_state: &GameState, rect: Rect, buf: &mut Buffer) {
        let height = rect.height as usize;
        let width = rect.width as usize;
        let start = game_state.log.messages.len().saturating_sub(height);

        // Fetch only as many lines as can be displayed (rough estimation, not accurate if lines wrap)
        let lines: Vec<Line> =
            game_state.log.messages[start..].iter().map(|msg| msg.display()).collect();

        // Use a heuristic to count how many lines the texts actually take up.
        let mut used_height = 0;
        let mut first_line = 0;
        for (i, line) in lines.iter().enumerate().rev() {
            let line_length: usize =
                line.spans.iter().map(|span| span.content.to_string().len()).sum();
            let estimated_nr_of_lines: usize = line_length.div_ceil(width).max(1);

            if used_height + estimated_nr_of_lines >= height {
                first_line = i;
                break;
            }

            used_height += estimated_nr_of_lines;
            first_line = i;
        }

        let lines_to_display: &[Line] = &lines[first_line..];

        let paragraph =
            Paragraph::new(Text::from(lines_to_display.to_vec())).wrap(Wrap { trim: true });
        paragraph.render(rect, buf);
    }

    /// Renders the menu in inventory mode.
    pub fn render_inventory(&self, game_state: &GameState, rect: Rect, buf: &mut Buffer) {
        let inventory = &game_state.player.character.inventory;

        let height = rect.height as usize;
        let start = inventory.len().saturating_sub(height);

        let lines: Vec<Line> = inventory[start..]
            .iter()
            .enumerate()
            .map(|(i, item_id)| {
                let list_letter = (b'a' + i as u8) as char;

                let instance = match game_state.get_item_by_id(*item_id) {
                    Some(inst) => inst,
                    None => return Line::raw(format!("{list_letter} - <Invalid Item>")),
                };

                let def = match game_state.get_item_def_by_id(&instance.def_id) {
                    Some(d) => d,
                    None => return Line::raw(format!("{list_letter} - <Invalid Item>")),
                };

                let mut styled = format_item_inventory(&def);

                styled.spans.insert(0, Span::raw(format!("{list_letter} - ")));

                styled
            })
            .collect();

        let paragraph = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: true });
        paragraph.render(rect, buf);
    }
}

pub fn format_item_inventory(def: &GameItemDef) -> Line<'static> {
    let mut spans = vec![
        Span::raw("["),
        Span::styled(def.glyph.to_string(), def.style),
        Span::raw("] "),
        Span::raw(def.name),
    ];

    match &def.kind {
        GameItemKindDef::Armor { mitigation } => {
            spans.push(Span::raw(" <"));
            spans.push(Span::raw(format!("{} MIT", mitigation)));
            spans.push(Span::raw(">"));
        }
        GameItemKindDef::Weapon { damage, crit_chance, .. } => {
            spans.push(Span::raw(" <"));
            spans.push(Span::raw(format!("{} DMG", damage)));
            spans.push(Span::raw(", "));
            spans.push(Span::raw(format!("{:.0}% CRIT", crit_chance * 10)));
            spans.push(Span::raw(">"));
        }
        GameItemKindDef::Food { nutrition } => {
            spans.push(Span::raw(" <"));
            spans.push(Span::raw(format!("{} NUT", nutrition)));
            spans.push(Span::raw(">"));
        }
    }

    Line::from(spans)
}
