#![allow(dead_code)]

use core::fmt;
use ratatui::style::Style;

use crate::{
    core::{
        buff_effects::PotionEffectDef,
        entity_logic::{Entity, EntityBase, EntityId},
        game::GameState,
    },
    data::item_defs::{GameItemDef, GameItemDefId, item_defs},
    util::{
        errors_results::{DataError, EngineError, GameError},
        rng::Roll,
    },
    world::coordinate_system::Point,
};

// Static Item Definitions
// Layer 1. This is where items and their kinds and details are defined.

#[derive(Clone, Debug)]
pub enum GameItemKindDef {
    Weapon { damage: Roll, crit_chance: u8, ranged: bool },
    Armor { mitigation: u16 },
    Food { nutrition: u16 },
    Potion { effect: PotionEffectDef },
}

#[derive(Clone, Copy)]
pub struct ArmorItem(pub GameItemId);

#[derive(Clone, Copy)]
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
    pub fn register_item(&mut self, def_id: &GameItemDefId) -> GameItemId {
        let id: GameItemId = self.id_system.next_item_id();
        self.items.insert(id, GameItem { def_id: def_id.clone() });
        self.log.debug_info(format!("Registered item {} (ID: {})", def_id, id));

        id
    }

    pub fn deregister_item(&mut self, item_id: GameItemId) -> Result<(), GameError> {
        match self.items.remove(&item_id) {
            Some(_) => {
                self.log.debug_info(format!("Deregistered item {}", item_id));
                Ok(())
            }
            None => Err(GameError::from(EngineError::UnregisteredItem(item_id))),
        }
    }

    /// Creates a new entity of type `GameItemSprite`.
    ///
    /// # Returns
    /// The Npcs [EntityId], which can then be used to get access to the newly spawned Npc.
    ///
    /// # Errors
    /// - [EngineError::UnregisteredItem()] if the item is not registered in the game state yet.
    /// - [DataError::MissingItemDefinition()] if npc is not defined in game data.
    /// - [EngineError::SpawningError()] if the position is not available.
    pub fn create_item_sprite(
        &mut self,
        item_id: GameItemId,
        pos: Point,
    ) -> Result<GameItemSprite, GameError> {
        // Checking if item is registerd.
        let item = self.get_item_by_id(item_id).ok_or(EngineError::UnregisteredItem(item_id))?;

        // Checking if item_def exists.
        let item_def = self
            .get_item_def_by_id(&item.def_id)
            .ok_or(DataError::MissingItemDefinition(item.def_id))?;

        // Creating item_sprite and assigning id.
        let entity_id = self.id_system.next_entity_id();
        let item_sprite = GameItemSprite::new(
            entity_id,
            item_def.name.to_string(),
            pos,
            item_def.glyph,
            item_def.style,
            item_id,
        );

        Ok(item_sprite)
    }

    pub fn get_item_by_id(&self, item_id: GameItemId) -> Option<GameItem> {
        self.items.get(&item_id).cloned()
    }

    pub fn get_item_def_by_id(&self, item_def_id: &GameItemDefId) -> Option<GameItemDef> {
        item_defs().get(item_def_id).cloned()
    }
}

// Item Sprite
// Items lying on the ground in the world as entities.
#[derive(Clone)]
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
