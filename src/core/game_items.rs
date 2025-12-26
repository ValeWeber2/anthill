#![allow(dead_code)]

use std::collections::HashMap;

use ratatui::style::Style;

use crate::{
    core::{
        entity_logic::{Entity, EntityBase, EntityId, Spawnable, SpawningError},
        game::GameState,
    },
    data::item_defs::item_defs,
    world::worldspace::Point,
};

// Static Item Definitions
// Layer 1. This is where items and their kinds and details are defined.
pub type GameItemDefId = &'static str;

pub struct GameItemDef {
    pub name: &'static str,
    pub glyph: char,
    pub style: Style,
    pub kind: GameItemKindDef,
}

pub enum GameItemKindDef {
    Weapon { damage: u32 },
    Armor { mitigation: u32 },
    Food,
}

// Item Proper
// Item instances as registered in the GameState.items.
pub type GameItemId = u32;

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

        self.log.print(format!("Registered item {} (ID: {})", def_id, id,));

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

    pub fn get_item_by_id(&self, item_id: GameItemId) -> Option<&GameItem> {
        self.items.get(&item_id)
    }

    pub fn get_item_def_by_id(&self, item_def_id: GameItemDefId) -> Option<&GameItemDef> {
        item_defs().get(item_def_id)
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
        &mut state.world.item_index
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
