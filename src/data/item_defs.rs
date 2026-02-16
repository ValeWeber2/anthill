use std::collections::HashMap;
use std::sync::OnceLock;

use ratatui::style::{Color, Style};

use crate::{
    core::{buff_effects::PotionEffectDef, game_items::GameItemKindDef},
    util::rng::{DieSize, Roll},
};

pub type GameItemDefId = String;

#[derive(Clone)]
pub struct GameItemDef {
    pub name: &'static str,
    pub glyph: char,
    pub style: Style,
    pub kind: GameItemKindDef,
}

pub fn item_defs() -> &'static HashMap<GameItemDefId, GameItemDef> {
    static ITEM_DEFS: OnceLock<HashMap<GameItemDefId, GameItemDef>> = OnceLock::new();
    ITEM_DEFS.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(
            "weapon_sword_rusty".to_string(),
            GameItemDef {
                name: "Rusty Sword",
                glyph: '/',
                style: Style::default().fg(Color::Gray),
                kind: GameItemKindDef::Weapon {
                    damage: Roll::new(1, DieSize::D10),
                    crit_chance: 5,
                    ranged: false,
                },
            },
        );
        m.insert(
            "weapon_bow_short".to_string(),
            GameItemDef {
                name: "Shortbow",
                glyph: 'D',
                style: Style::default().fg(Color::Gray),
                kind: GameItemKindDef::Weapon {
                    damage: Roll::new(1, DieSize::D6),
                    crit_chance: 5,
                    ranged: true,
                },
            },
        );
        m.insert(
            "weapon_bow_long".to_string(),
            GameItemDef {
                name: "Longbow",
                glyph: 'D',
                style: Style::default().fg(Color::DarkGray),
                kind: GameItemKindDef::Weapon {
                    damage: Roll::new(1, DieSize::D10),
                    crit_chance: 7,
                    ranged: true,
                },
            },
        );
        m.insert(
            "weapon_mace".to_string(),
            GameItemDef {
                name: "Iron Mace",
                glyph: '/',
                style: Style::default().fg(Color::Yellow),
                kind: GameItemKindDef::Weapon {
                    damage: Roll::new(2, DieSize::D10),
                    crit_chance: 5,
                    ranged: false,
                },
            },
        );
        m.insert(
            "weapon_axe_iron".to_string(),
            GameItemDef {
                name: "Iron Axe",
                glyph: '/',
                style: Style::default().fg(Color::DarkGray),
                kind: GameItemKindDef::Weapon {
                    damage: Roll::new(2, DieSize::D6),
                    crit_chance: 7,
                    ranged: false,
                },
            },
        );
        m.insert(
            "weapon_dagger".to_string(),
            GameItemDef {
                name: "Sharp Dagger",
                glyph: '\\',
                style: Style::default().fg(Color::White),
                kind: GameItemKindDef::Weapon {
                    damage: Roll::new(1, DieSize::D8),
                    crit_chance: 15,
                    ranged: false,
                },
            },
        );
        m.insert(
            "weapon_warhammer".to_string(),
            GameItemDef {
                name: "Warhammer",
                glyph: '/',
                style: Style::default().fg(Color::Red),
                kind: GameItemKindDef::Weapon {
                    damage: Roll::new(2, DieSize::D12),
                    crit_chance: 5,
                    ranged: false,
                },
            },
        );
        m.insert(
            "weapon_short_sword".to_string(),
            GameItemDef {
                name: "Short Sword",
                glyph: '/',
                style: Style::default().fg(Color::Gray),
                kind: GameItemKindDef::Weapon {
                    damage: Roll::new(1, DieSize::D10).add_modifier(1),
                    crit_chance: 10,
                    ranged: false,
                },
            },
        );
        m.insert(
            "weapon_spear".to_string(),
            GameItemDef {
                name: "Spear",
                glyph: '/',
                style: Style::default().fg(Color::White),
                kind: GameItemKindDef::Weapon {
                    damage: Roll::new(2, DieSize::D6),
                    crit_chance: 8,
                    ranged: false,
                },
            },
        );
        m.insert(
            "armor_leather".to_string(),
            GameItemDef {
                name: "Leather Armor",
                glyph: 'A',
                style: Style::default().fg(Color::Yellow),
                kind: GameItemKindDef::Armor { mitigation: 2 },
            },
        );
        m.insert(
            "armor_chainmail".to_string(),
            GameItemDef {
                name: "Chainmail Armor",
                glyph: 'A',
                style: Style::default().fg(Color::LightBlue),
                kind: GameItemKindDef::Armor { mitigation: 4 },
            },
        );
        m.insert(
            "armor_plate".to_string(),
            GameItemDef {
                name: "Plate Armor",
                glyph: 'A',
                style: Style::default().fg(Color::Gray),
                kind: GameItemKindDef::Armor { mitigation: 6 },
            },
        );
        m.insert(
            "armor_helmet".to_string(),
            GameItemDef {
                name: "Steel Helmet",
                glyph: 'A',
                style: Style::default().fg(Color::Gray),
                kind: GameItemKindDef::Armor { mitigation: 2 },
            },
        );
        m.insert(
            "armor_gauntlets".to_string(),
            GameItemDef {
                name: "Iron Gauntlets",
                glyph: 'A',
                style: Style::default().fg(Color::DarkGray),
                kind: GameItemKindDef::Armor { mitigation: 1 },
            },
        );
        m.insert(
            "armor_shield".to_string(),
            GameItemDef {
                name: "Wooden Shield",
                glyph: 'A',
                style: Style::default().fg(Color::Yellow),
                kind: GameItemKindDef::Armor { mitigation: 3 },
            },
        );
        m.insert(
            "armor_cloak".to_string(),
            GameItemDef {
                name: "Cloak of Shadows",
                glyph: 'A',
                style: Style::default().fg(Color::Black),
                kind: GameItemKindDef::Armor { mitigation: 2 },
            },
        );
        m.insert(
            "food_cake".to_string(),
            GameItemDef {
                name: "Cake",
                glyph: '%',
                style: Style::default().fg(Color::Red),
                kind: GameItemKindDef::Food { nutrition: 1 },
            },
        );
        m.insert(
            "food_grapefruit".to_string(),
            GameItemDef {
                name: "Grapefruit",
                glyph: '%',
                style: Style::default().fg(Color::LightRed),
                kind: GameItemKindDef::Food { nutrition: 2 },
            },
        );
        m.insert(
            "food_honey".to_string(),
            GameItemDef {
                name: "Honey Jar",
                glyph: '%',
                style: Style::default().fg(Color::Yellow),
                kind: GameItemKindDef::Food { nutrition: 4 },
            },
        );
        m.insert(
            "food_fish".to_string(),
            GameItemDef {
                name: "Cooked Fish",
                glyph: '%',
                style: Style::default().fg(Color::Blue),
                kind: GameItemKindDef::Food { nutrition: 6 },
            },
        );
        m.insert(
            "food_mushroom".to_string(),
            GameItemDef {
                name: "Mushroom",
                glyph: '%',
                style: Style::default().fg(Color::Green),
                kind: GameItemKindDef::Food { nutrition: 1 },
            },
        );
        m.insert(
            "food_meat".to_string(),
            GameItemDef {
                name: "Cooked Meat",
                glyph: '%',
                style: Style::default().fg(Color::Red),
                kind: GameItemKindDef::Food { nutrition: 7 },
            },
        );
        m.insert(
            "food_apple".to_string(),
            GameItemDef {
                name: "Apple",
                glyph: '%',
                style: Style::default().fg(Color::Red),
                kind: GameItemKindDef::Food { nutrition: 2 },
            },
        );
        m.insert(
            "food_bread".to_string(),
            GameItemDef {
                name: "Loaf of Bread",
                glyph: '%',
                style: Style::default().fg(Color::Yellow),
                kind: GameItemKindDef::Food { nutrition: 5 },
            },
        );
        m.insert(
            "food_cheese".to_string(),
            GameItemDef {
                name: "Cheese",
                glyph: '%',
                style: Style::default().fg(Color::LightYellow),
                kind: GameItemKindDef::Food { nutrition: 3 },
            },
        );
        m.insert(
            "potion_healing_small".to_string(),
            GameItemDef {
                name: "Small Healing Potion",
                glyph: '!',
                style: Style::default().fg(Color::Magenta),
                kind: GameItemKindDef::Potion { effect: PotionEffectDef::Heal { amount: 20 } },
            },
        );
        m.insert(
            "potion_strength".to_string(),
            GameItemDef {
                name: "Potion of Strength",
                glyph: '!',
                style: Style::default().fg(Color::Magenta),
                kind: GameItemKindDef::Potion {
                    effect: PotionEffectDef::Strength { amount: 3, duration: 100 },
                },
            },
        );
        m.insert(
            "potion_dexterity".to_string(),
            GameItemDef {
                name: "Potion of Dexterity",
                glyph: '!',
                style: Style::default().fg(Color::Blue),
                kind: GameItemKindDef::Potion {
                    effect: PotionEffectDef::Dexterity { amount: 2, duration: 100 },
                },
            },
        );
        m
    })
}
