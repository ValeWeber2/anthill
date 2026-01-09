use std::collections::HashMap;
use std::sync::OnceLock;

use ratatui::style::{Color, Style};

use crate::core::game_items::{GameItemDef, GameItemDefId, GameItemKindDef};

pub fn item_defs() -> &'static HashMap<GameItemDefId, GameItemDef> {
    static ITEM_DEFS: OnceLock<HashMap<GameItemDefId, GameItemDef>> = OnceLock::new();
    ITEM_DEFS.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(
            "weapon_sword_rusty".to_string(),
            GameItemDef {
                name: "Rusty Sword".to_string(),
                glyph: '/',
                style: Style::default().fg(Color::Gray),
                kind: GameItemKindDef::Weapon { damage: 5 },
            },
        );
        m.insert(
            "armor_leather".to_string(),
            GameItemDef {
                name: "Leather Armor".to_string(),
                glyph: 'A',
                style: Style::default().fg(Color::Yellow),
                kind: GameItemKindDef::Armor { mitigation: 2 },
            },
        );
        m.insert(
            "food_cake".to_string(),
            GameItemDef {
                name: "Cake".to_string(),
                glyph: '%',
                style: Style::default().fg(Color::Red),
                kind: GameItemKindDef::Food,
            },
        );
        m
    })
}
