use std::collections::HashMap;
use std::sync::OnceLock;

use ratatui::style::{Color, Style};

use crate::{
    core::entity_logic::{BaseStats, NpcStats},
    util::rng::{DieSize, Roll},
};

pub type NpcDefId = String;

#[derive(Clone)]
pub struct NpcDef {
    pub name: &'static str,
    pub glyph: char,
    pub style: Style,
    pub stats: NpcStats,
}

pub fn npc_defs() -> &'static HashMap<NpcDefId, NpcDef> {
    static NPC_DEFS: OnceLock<HashMap<NpcDefId, NpcDef>> = OnceLock::new();
    NPC_DEFS.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(
            "goblin".to_string(),
            NpcDef {
                name: "Goblin",
                glyph: 'g',
                style: Style::default().fg(Color::Green),
                stats: NpcStats {
                    base: BaseStats { hp_max: 10, hp_current: 10 },
                    damage: Roll::new(1, DieSize::D4),
                    dodge: 10,
                    mitigation: 0,
                },
            },
        );
        m.insert(
            "funny_frog".to_string(),
            NpcDef {
                name: "Funny Frog",
                glyph: 'F',
                style: Style::default().fg(Color::LightGreen),
                stats: NpcStats {
                    base: BaseStats { hp_max: 5, hp_current: 5 },
                    damage: Roll::new(0, DieSize::D4),
                    dodge: 20,
                    mitigation: 0,
                },
            },
        );
        m.insert(
            "orc".to_string(),
            NpcDef {
                name: "Orc",
                glyph: 'O',
                style: Style::default().fg(Color::Gray),
                stats: NpcStats {
                    base: BaseStats { hp_max: 20, hp_current: 20 },
                    damage: Roll::new(1, DieSize::D10),
                    dodge: 0,
                    mitigation: 2,
                },
            },
        );
        m.insert(
            "skeleton".to_string(),
            NpcDef {
                name: "Skeleton",
                glyph: 's',
                style: Style::default().fg(Color::Gray),
                stats: NpcStats {
                    base: BaseStats { hp_max: 12, hp_current: 12 },
                    damage: Roll::new(1, DieSize::D6),
                    dodge: 5,
                    mitigation: 1,
                },
            },
        );
        m.insert(
            "giant_rat".to_string(),
            NpcDef {
                name: "Giant Albino Rat",
                glyph: 'R',
                style: Style::default().fg(Color::White),
                stats: NpcStats {
                    base: BaseStats { hp_max: 8, hp_current: 8 },
                    damage: Roll::new(1, DieSize::D4),
                    dodge: 15,
                    mitigation: 0,
                },
            },
        );
        m.insert(
            "bandit".to_string(),
            NpcDef {
                name: "Bandit",
                glyph: 'B',
                style: Style::default().fg(Color::Yellow),
                stats: NpcStats {
                    base: BaseStats { hp_max: 16, hp_current: 16 },
                    damage: Roll::new(1, DieSize::D6).add_modifier(1),
                    dodge: 10,
                    mitigation: 1,
                },
            },
        );
        m.insert(
            "dark_mage".to_string(),
            NpcDef {
                name: "Dark Mage",
                glyph: 'M',
                style: Style::default().fg(Color::Magenta),
                stats: NpcStats {
                    base: BaseStats { hp_max: 10, hp_current: 10 },
                    damage: Roll::new(2, DieSize::D4).add_modifier(1),
                    dodge: 5,
                    mitigation: 0,
                },
            },
        );
        m.insert(
            "wolf".to_string(),
            NpcDef {
                name: "Wolf",
                glyph: 'W',
                style: Style::default().fg(Color::Gray),
                stats: NpcStats {
                    base: BaseStats { hp_max: 14, hp_current: 14 },
                    damage: Roll::new(1, DieSize::D6).add_modifier(1),
                    dodge: 20,
                    mitigation: 0,
                },
            },
        );
        m.insert(
            "slime".to_string(),
            NpcDef {
                name: "Slime",
                glyph: 'S',
                style: Style::default().fg(Color::Blue),
                stats: NpcStats {
                    base: BaseStats { hp_max: 18, hp_current: 18 },
                    damage: Roll::new(1, DieSize::D6),
                    dodge: 0,
                    mitigation: 3,
                },
            },
        );
        m.insert(
            "zombie".to_string(),
            NpcDef {
                name: "Zombie",
                glyph: 'Z',
                style: Style::default().fg(Color::Green),
                stats: NpcStats {
                    base: BaseStats { hp_max: 22, hp_current: 22 },
                    damage: Roll::new(1, DieSize::D8),
                    dodge: 0,
                    mitigation: 2,
                },
            },
        );
        m.insert(
            "assassin".to_string(),
            NpcDef {
                name: "Assassin",
                glyph: 'A',
                style: Style::default().fg(Color::Red),
                stats: NpcStats {
                    base: BaseStats { hp_max: 12, hp_current: 12 },
                    damage: Roll::new(2, DieSize::D6),
                    dodge: 25,
                    mitigation: 0,
                },
            },
        );
        m.insert(
            "cultist".to_string(),
            NpcDef {
                name: "Cultist",
                glyph: 'C',
                style: Style::default().fg(Color::Red),
                stats: NpcStats {
                    base: BaseStats { hp_max: 14, hp_current: 14 },
                    damage: Roll::new(2, DieSize::D6).add_modifier(3),
                    dodge: 8,
                    mitigation: 1,
                },
            },
        );
        m.insert(
            "ferris".to_string(),
            NpcDef {
                name: "Ferris, the Rustacean",
                glyph: 'U',
                style: Style::default().fg(Color::Red),
                stats: NpcStats {
                    base: BaseStats { hp_max: 10, hp_current: 10 },
                    damage: Roll::new(1, DieSize::D10),
                    dodge: 0,
                    mitigation: 4,
                },
            },
        );
        m.insert(
            "martin".to_string(),
            NpcDef {
                name: "Martin, the Explorer",
                glyph: 'M',
                style: Style::default().fg(Color::Blue),
                stats: NpcStats {
                    base: BaseStats { hp_max: 25, hp_current: 25 },
                    damage: Roll::new(2, DieSize::D12).add_modifier(1),
                    dodge: 5,
                    mitigation: 6,
                },
            },
        );
        m.insert(
            "borrowchecker".to_string(),
            NpcDef {
                name: "Borrow Checker",
                glyph: '&',
                style: Style::default().fg(Color::Blue),
                stats: NpcStats {
                    base: BaseStats { hp_max: 1, hp_current: 1 },
                    damage: Roll::new(1, DieSize::D6),
                    dodge: 50,
                    mitigation: 0,
                },
            },
        );
        m
    })
}
