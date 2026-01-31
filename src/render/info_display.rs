use ratatui::{
    prelude::*,
    text::Line,
    widgets::{Cell, Row, Table},
};

use crate::{
    core::{entity_logic::Entity, game::GameState, game_items::AsItemId},
    render::ui::format_item,
};

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
    /// * Game Info
    ///     * Dungeon Floor the character is currently on
    ///     * Experience points collected
    ///     * Current game round
    pub fn render(&self, game: &GameState, rect: Rect, buf: &mut Buffer) {
        let player_hp_current = game.player.character.stats.base.hp_current;
        let player_hp_max = game.player.character.stats.base.hp_max;
        let weapon_line = {
            let mut line = Line::from("Weapon:");
            line.spans.extend(self.format_weapon(game).spans);
            line
        };
        let armor_line = {
            let mut line = Line::from("Armor:");
            line.spans.extend(self.format_armor(game).spans);
            line
        };

        let info_rows = [
            Row::new(vec![
                Cell::from(format!("HP:{}({})", player_hp_current, player_hp_max)),
                Cell::from(weapon_line),
                Cell::from(format!(
                    "Exp:{} Round:{}",
                    game.player.character.stats.experience, game.round_nr
                )),
                Cell::from(format!(
                    "x:{} y:{}",
                    game.player.character.pos().x,
                    game.player.character.pos().y
                )),
            ]),
            Row::new(vec![
                Cell::from(format!(
                    "STR:{} DEX:{}, VIT:{}, PER:{}",
                    game.player.character.stats.strength,
                    game.player.character.stats.dexterity,
                    game.player.character.stats.vitality,
                    game.player.character.stats.perception
                )),
                Cell::from(armor_line),
                Cell::from(format!("Dungeon Floor:{}", game.level_nr)),
            ]),
        ];

        const INFO_WIDTHS: [Constraint; 4] = [
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ];

        let info_table = Table::new(info_rows, INFO_WIDTHS);

        Widget::render(info_table, rect, buf);
    }

    fn format_armor(&self, game: &GameState) -> Line<'static> {
        self.format_equipped_item(game, &game.player.character.armor, "Invalid armor")
    }

    fn format_weapon(&self, game: &GameState) -> Line<'static> {
        self.format_equipped_item(game, &game.player.character.weapon, "Invalid weapon")
    }

    fn format_equipped_item<T: AsItemId>(
        &self,
        game: &GameState,
        slot: &Option<T>,
        invalid_label: &str,
    ) -> Line<'static> {
        match slot {
            Some(wrapper) => {
                let instance = match game.items.get(wrapper.id()) {
                    Some(i) => i,
                    None => return Line::raw(invalid_label.to_string()),
                };

                let def = match game.get_item_def_by_id(instance.def_id.clone()) {
                    Some(d) => d,
                    None => return Line::raw(invalid_label.to_string()),
                };

                format_item(&def)
            }
            None => Line::raw("None"),
        }
    }
}
