use std::collections::HashMap;
use std::sync::OnceLock;

use ratatui::style::{Color, Style};

use crate::core::game_items::{GameItemDef, GameItemDefId, GameItemKindDef};

pub fn item_defs() -> &'static HashMap<GameItemDefId, GameItemDef> {
    static ITEM_DEFS: OnceLock<HashMap<GameItemDefId, GameItemDef>> = OnceLock::new();
    ITEM_DEFS.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(
            "weapon_sword_rusty",
            GameItemDef {
                name: "Rusty Sword",
                glyph: '/',
                style: Style::default().fg(Color::Gray),
                kind: GameItemKindDef::Weapon { damage: 5 },
            },
        );
        m.insert(
            "armor_leather",
            GameItemDef {
                name: "Leather Armor",
                glyph: 'A',
                style: Style::default().fg(Color::Yellow),
                kind: GameItemKindDef::Armor { mitigation: 2 },
            },
        );
        m.insert(
            "food_cake",
            GameItemDef {
                name: "Cake",
                glyph: '%',
                style: Style::default().fg(Color::Red),
                kind: GameItemKindDef::Food,
            },
        );
        m
    })
}
