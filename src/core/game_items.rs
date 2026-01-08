#![allow(dead_code)]

use core::fmt;
use std::collections::HashMap;

use ratatui::style::Style;

use crate::{
    core::{
        entity_logic::{Entity, EntityBase, EntityId, Spawnable, SpawningError},
        game::GameState,
    },
    data::item_defs::item_defs,
    world::coordinate_system::Point,
};

// Static Item Definitions
// Layer 1. This is where items and their kinds and details are defined.
pub type GameItemDefId = &'static str;

#[derive(Clone)]
pub struct GameItemDef {
    pub name: &'static str,
    pub glyph: char,
    pub style: Style,
    pub kind: GameItemKindDef,
}

#[derive(Clone)]
pub enum GameItemKindDef {
    Weapon { damage: u32 },
    Armor { mitigation: u32 },
    Food { nutrition: u32 },
}

pub struct ArmorItem(pub GameItemId);
pub struct WeaponItem(pub GameItemId);

impl ArmorItem {
    pub fn try_new(def: GameItemDef, id: GameItemId) -> Option<Self> {
        match def.kind {
            GameItemKindDef::Armor { .. } => Some(Self(id)),
            _ => None,
        }
    }
}

impl WeaponItem {
    pub fn try_new(def: GameItemDef, id: GameItemId) -> Option<Self> {
        match def.kind {
            GameItemKindDef::Weapon { .. } => Some(Self(id)),
            _ => None,
        }
    }
}

impl fmt::Display for ArmorItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for WeaponItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Item Proper
// Item instances as registered in the GameState.items.
pub type GameItemId = u32;

#[derive(Clone)]
pub struct GameItem {
    pub def_id: GameItemDefId,
}

// Implementation into Game State
impl GameState {
    pub fn next_item_id(&mut self) -> GameItemId {
        let id = self.item_id_counter;
        self.item_id_counter += 1;

        id
    }

    pub fn register_item(&mut self, def_id: GameItemDefId) -> GameItemId {
        let id: GameItemId = self.next_item_id();
        self.items.insert(id, GameItem { def_id });

        self.log.debug_print(format!("Registered item {} (ID: {})", def_id, id,));

        id
    }

    pub fn spawn_item(
        &mut self,
        item_id: GameItemId,
        pos: Point,
    ) -> Result<EntityId, SpawningError> {
        let item = self.get_item_by_id(item_id).ok_or(SpawningError::ItemNotRegistered(item_id))?;

        let item_def = self
            .get_item_def_by_id(item.def_id)
            .ok_or(SpawningError::ItemNotDefined(item.def_id))?;

        self.spawn::<GameItemSprite>(
            item_def.name.into(),
            pos,
            item_def.glyph,
            item_def.style,
            item_id,
        )
    }

    pub fn get_item_by_id(&self, item_id: GameItemId) -> Option<GameItem> {
        self.items.get(&item_id).cloned()
    }

    pub fn get_item_def_by_id(&self, item_def_id: GameItemDefId) -> Option<GameItemDef> {
        item_defs().get(item_def_id).cloned()
    }

    pub fn format_weapon(&self) -> String {
        match &self.player.character.weapon {
            Some(w) => {
                // look up the instance by GameItemId
                let instance = match self.items.get(&w.0) {
                    Some(i) => i,
                    None => return "Invalid weapon".to_string(),
                };

                // look up the definition by def_id
                let def = match self.get_item_def_by_id(instance.def_id) {
                    Some(d) => d,
                    None => return "Invalid weapon".to_string(),
                };

                // extract stats from GameItemKindDef
                match def.kind {
                    GameItemKindDef::Weapon { damage } => {
                        format!("{} (damage {})", def.name, damage)
                    }
                    _ => "Invalid weapon".to_string(),
                }
            }
            None => "None".to_string(),
        }
    }

    pub fn format_armor(&self) -> String {
        match &self.player.character.armor {
            Some(a) => {
                let instance = match self.items.get(&a.0) {
                    Some(i) => i,
                    None => return "Invalid armor".to_string(),
                };

                let def = match self.get_item_def_by_id(instance.def_id) {
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

// Item Sprite
// Items lying on the ground in the world as entities.
pub struct GameItemSprite {
    pub base: EntityBase,
    pub item_id: GameItemId,
}

impl Entity for GameItemSprite {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn id(&self) -> EntityId {
        self.base.id
    }

    fn pos(&self) -> &Point {
        &self.base.pos
    }
}

impl Spawnable for GameItemSprite {
    type Extra = GameItemId;

    fn new(
        id: EntityId,
        name: String,
        pos: Point,
        glyph: char,
        style: Style,
        item_id: GameItemId,
    ) -> Self {
        GameItemSprite::new(id, name, pos, glyph, style, item_id)
    }

    fn storage_mut(state: &mut GameState) -> &mut Vec<Self> {
        &mut state.world.item_sprites
    }

    fn index_mut(state: &mut GameState) -> &mut HashMap<EntityId, usize> {
        &mut state.world.item_sprites_index
    }
}

impl GameItemSprite {
    pub fn new(
        id: EntityId,
        name: String,
        pos: Point,
        glyph: char,
        style: Style,
        item_id: GameItemId,
    ) -> Self {
        Self { base: EntityBase { id, name, pos, glyph, style }, item_id }
    }
}
