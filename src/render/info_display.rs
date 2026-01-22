use ratatui::{prelude::*, widgets::Paragraph};

use crate::core::{entity_logic::Entity, game::GameState, game_items::GameItemKindDef};

pub struct InfoDisplay;

impl InfoDisplay {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, game: &GameState, rect: Rect, buf: &mut Buffer) {
        let player_hp_current = game.player.character.stats.base.hp_current;
        let player_hp_max = game.player.character.stats.base.hp_max;
        let player_armor = self.format_armor(game);
        let player_weapon = self.format_weapon(game);

        let lines: Vec<Line> = vec![
            Line::raw(format!(
                "STR:{} DEX:{}, VIT:{}, PER:{} HP:{}({})",
                game.player.character.stats.strength,
                game.player.character.stats.dexterity,
                game.player.character.stats.vitality,
                game.player.character.stats.perception,
                player_hp_current,
                player_hp_max,
            )),
            Line::raw(format!("Armor:{} Weapon:{}", player_armor, player_weapon)),
            Line::raw(format!(
                "Dungeon Floor:{} x:{} y:{} Exp:{} Round:{} ",
                game.level_nr,
                game.player.character.pos().x,
                game.player.character.pos().y,
                game.player.character.stats.experience,
                game.round_nr,
            )),
        ];

        let paragraph = Paragraph::new(Text::from(lines));
        paragraph.render(rect, buf);
    }

    pub fn format_weapon(&self, game: &GameState) -> String {
        match &game.player.character.weapon {
            Some(w) => {
                // look up the instance by GameItemId
                let instance = match game.items.get(&w.0) {
                    Some(i) => i,
                    None => return "Invalid weapon".to_string(),
                };

                // look up the definition by def_id
                let def = match game.get_item_def_by_id(instance.def_id.clone()) {
                    Some(d) => d,
                    None => return "Invalid weapon".to_string(),
                };

                // extract stats from GameItemKindDef
                match def.kind {
                    GameItemKindDef::Weapon { damage, crit_chance } => {
                        format!("{} (damage {}, crit chance {})", def.name, damage, crit_chance)
                    }
                    _ => "Invalid weapon".to_string(),
                }
            }
            None => "None".to_string(),
        }
    }

    pub fn format_armor(&self, game: &GameState) -> String {
        match &game.player.character.armor {
            Some(a) => {
                let instance = match game.items.get(&a.0) {
                    Some(i) => i,
                    None => return "Invalid armor".to_string(),
                };

                let def = match game.get_item_def_by_id(instance.def_id.clone()) {
                    Some(d) => d,
                    None => return "Invalid armor".to_string(),
                };

                match def.kind {
                    GameItemKindDef::Armor { mitigation } => {
                        format!("{} (mitigation {})", def.name, mitigation)
                    }
                    _ => "Invalid armor".to_string(),
                }
            }
            None => "None".to_string(),
        }
    }
}
