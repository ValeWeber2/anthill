use std::collections::HashMap;
use std::sync::OnceLock;

use ratatui::style::{Color, Style};

use crate::core::entity_logic::{BaseStats, NpcStats};

pub type NpcDefId = String;

#[derive(Clone)]
#[allow(dead_code)]
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
                    damage: 2,
                    dodge: 10,
                    mitigation: 0,
                },
            },
        );
        m.insert(
            "funny_frog".to_string(),
            NpcDef {
                name: "Funny Frog",
                glyph: 'f',
                style: Style::default().fg(Color::LightGreen),
                stats: NpcStats {
                    base: BaseStats { hp_max: 5, hp_current: 5 },
                    damage: 0,
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
                    damage: 5,
                    dodge: 0,
                    mitigation: 2,
                },
            },
        );
        m
    })
}
