use ratatui::{prelude::*, widgets::Paragraph};

use crate::core::game::GameState;

pub struct InfoDisplay;

impl InfoDisplay {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, game: &GameState, rect: Rect, buf: &mut Buffer) {
        let player_str = 14; // Mock
        let player_agi = 18; // Mock
        let player_hp_current = game.player.character.stats.base.hp_current;
        let player_hp_max = game.player.character.stats.base.hp_max;

        let dungeon_level = 1; // Mock
        let experience_points = 0; // Mock
        let round_number = game.round_nr;

        let lines: Vec<Line> = vec![
            Line::raw(format!(
                "STR:{} AGI:{} HP:{}({})",
                player_str, player_agi, player_hp_current, player_hp_max,
            )),
            Line::raw(format!(
                "Dungeon Floor:{} Exp:{} Round:{}",
                dungeon_level, experience_points, round_number,
            )),
        ];

        let paragraph = Paragraph::new(Text::from(lines));
        paragraph.render(rect, buf);
    }
}
