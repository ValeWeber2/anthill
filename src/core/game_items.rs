#![allow(dead_code)]

use core::fmt;
use std::collections::HashMap;

use ratatui::style::Style;

use crate::{
    core::{
        entity_logic::{Entity, EntityBase, EntityId, Spawnable},
        game::GameState,
    },
    data::item_defs::{GameItemDef, GameItemDefId, item_defs},
    util::errors_results::{DataError, EngineError, GameError},
    world::coordinate_system::Point,
};

// Static Item Definitions
// Layer 1. This is where items and their kinds and details are defined.

#[derive(Clone, Debug)]
pub enum GameItemKindDef {
    Weapon { damage: u16, crit_chance: u8 },
    Armor { mitigation: u16 },
    Food { nutrition: u16 },
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

pub trait AsItemId {
    fn id(&self) -> &GameItemId;
}

impl AsItemId for ArmorItem {
    fn id(&self) -> &GameItemId {
        &self.0
    }
}

impl AsItemId for WeaponItem {
    fn id(&self) -> &GameItemId {
        &self.0
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
        self.items.insert(id, GameItem { def_id: def_id.clone() });

        self.log.debug_print(format!("Registered item {} (ID: {})", def_id, id,));

        id
    }

    pub fn deregister_item(&mut self, item_id: GameItemId) -> Result<(), GameError> {
        match self.items.remove(&item_id) {
            Some(_) => Ok(()),
            None => Err(GameError::from(EngineError::UnregisteredItem(item_id))),
        }
    }

    pub fn spawn_item(&mut self, item_id: GameItemId, pos: Point) -> Result<EntityId, GameError> {
        let item = self.get_item_by_id(item_id).ok_or(EngineError::UnregisteredItem(item_id))?;

        let item_def = self
            .get_item_def_by_id(item.def_id.clone())
            .ok_or(DataError::MissingItemDefinition(item.def_id))?;

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
        item_defs().get(&item_def_id).cloned()
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

    fn pos(&self) -> Point {
        self.base.pos
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
        &mut state.item_sprites
    }

    fn index_mut(state: &mut GameState) -> &mut HashMap<EntityId, usize> {
        &mut state.item_sprites_index
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
