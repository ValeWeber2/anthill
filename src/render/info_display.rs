use ratatui::{prelude::*, widgets::Paragraph};

use crate::core::game::GameState;

pub struct InfoDisplay;

impl InfoDisplay {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, game: &GameState, rect: Rect, buf: &mut Buffer) {
        let player_hp_current = game.player.character.stats.base.hp_current;
        let player_hp_max = game.player.character.stats.base.hp_max;
        let player_armor = game.format_armor();
        let player_weapon = game.format_weapon();

        let lines: Vec<Line> = vec![
            Line::raw(format!(
                "STR:{} DEX:{}, VIT:{}, PER:{} HP:{}({})    Armor:{} Weapon:{}",
                game.player.character.stats.strength,
                game.player.character.stats.dexterity,
                game.player.character.stats.vitality,
                game.player.character.stats.perception,
                player_hp_current,
                player_hp_max,
                player_armor,
                player_weapon
            )),
            Line::raw(format!(
                "Dungeon Floor:{} Exp:{} Round:{}",
                game.level_nr, game.player.character.stats.experience, game.round_nr,
            )),
        ];

        let paragraph = Paragraph::new(Text::from(lines));
        paragraph.render(rect, buf);
    }
}
