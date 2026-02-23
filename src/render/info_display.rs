use ratatui::{
    prelude::*,
    widgets::{Cell, Row, Table},
};

use crate::core::{entity_logic::Entity, game::GameState, game_items::GameItemKindDef};

pub struct InfoDisplay;

impl InfoDisplay {
    pub fn new() -> Self {
        Self
    }

    /// Renders the Info Display
    ///
    /// The info display displays character info and information about the game.
    /// * Character Info
    ///     * Character Strength
    ///     * Character Dexterity
    ///     * Character Vitality
    ///     * Character Perception
    ///     * Character Hit Points
    ///     * Character equipped armor
    ///     * Character equipped weapon
    ///     * Character position
    /// * Game Info
    ///     * Dungeon Floor the character is currently on
    ///     * Experience points collected
    ///     * Current game round
    ///     * Current game level
    pub fn render(&self, game: &GameState, rect: Rect, buf: &mut Buffer) {
        let player_hp_current = self.format_hp(game);
        let player_hp_max = game.player.character.stats.base.hp_max;
        let weapon = self.format_weapon(game);
        let armor = self.format_armor(game);

        let info_rows = [
            Row::new(vec![
                Cell::from(Line::from(vec![
                    Span::raw("HP: "),
                    player_hp_current.clone(),
                    Span::raw(format!("/{}", player_hp_max)),
                ])),
                Cell::from(format!("Weapon: {}", weapon)),
                Cell::from(format!(
                    "EXP: {}/{}, Round: {}",
                    game.player.character.stats.experience,
                    (game.player.character.stats.level as u32) * 100,
                    game.round_nr
                )),
                Cell::from(format!(
                    "x: {}, y: {}",
                    game.player.character.pos().x,
                    game.player.character.pos().y
                )),
            ]),
            Row::new(vec![
                Cell::from(Line::from(vec![
                    Span::styled("STR: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{}, ", game.player.character.stats.strength)),
                    Span::styled("DEX: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{}, ", game.player.character.stats.dexterity)),
                    Span::styled("VIT: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{}, ", game.player.character.stats.vitality)),
                    Span::styled("PER: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{}", game.player.character.stats.perception)),
                ])),
                Cell::from(format!("Armor: {}", armor)),
                Cell::from(format!("Level: {}", game.player.character.stats.level)),
                Cell::from(format!("Dungeon Floor: {}", game.level_nr)),
            ]),
        ];

        const INFO_WIDTHS: [Constraint; 4] = [
            Constraint::Percentage(28),
            Constraint::Percentage(28),
            Constraint::Percentage(28),
            Constraint::Percentage(16),
        ];

        let info_table = Table::new(info_rows, INFO_WIDTHS);

        Widget::render(info_table, rect, buf);
    }

    /// Render the currently equipped armor into a String, displaying its stats.
    fn format_armor(&self, game: &GameState) -> String {
        match &game.player.character.armor {
            Some(w) => {
                // look up the instance by GameItemId
                let instance = match game.items.get(&w.0) {
                    Some(i) => i,
                    None => return "Invalid armor".to_string(),
                };

                // look up the definition by def_id
                let def = match game.get_item_def_by_id(&instance.def_id) {
                    Some(d) => d,
                    None => return "Invalid armor".to_string(),
                };

                // extract stats from GameItemKindDef
                match def.kind {
                    GameItemKindDef::Armor { mitigation } => {
                        format!("{} <{} MIT>", def.name, mitigation)
                    }
                    _ => "Invalid armor".to_string(),
                }
            }
            None => "None".to_string(),
        }
    }

    /// Render the currently equipped weapon into a String, displaying its stats.
    pub fn format_weapon(&self, game: &GameState) -> String {
        match &game.player.character.weapon {
            Some(w) => {
                // look up the instance by GameItemId
                let instance = match game.items.get(&w.0) {
                    Some(i) => i,
                    None => return "Invalid weapon".to_string(),
                };

                // look up the definition by def_id
                let def = match game.get_item_def_by_id(&instance.def_id) {
                    Some(d) => d,
                    None => return "Invalid weapon".to_string(),
                };

                // extract stats from GameItemKindDef
                match def.kind {
                    GameItemKindDef::Weapon { damage, crit_chance, range: _range } => {
                        format!("{} <{} DMG, {}% CRIT>", def.name, damage, crit_chance)
                    }
                    _ => "Invalid weapon".to_string(),
                }
            }
            None => "Fist <1d4 DMG, 5% CRIT>".to_string(),
        }
    }

    /// Format the player's current hit points for display.
    pub fn format_hp(&self, game: &GameState) -> Span<'_> {
        let hp_current = game.player.character.stats.base.hp_current;
        let hp_max = game.player.character.stats.base.hp_max;
        let mut color = Color::Gray;

        if hp_current * 5 <= hp_max {
            color = Color::Red;
        }

        Span::styled(hp_current.to_string(), Style::default().fg(color))
    }
}
