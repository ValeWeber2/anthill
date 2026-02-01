#![allow(dead_code)]

use std::{fmt, io};

use ron::de::SpannedError;

use crate::{
    core::{
        entity_logic::EntityId,
        game_items::{GameItemId, GameItemKindDef},
    },
    data::{item_defs::GameItemDefId, npc_defs::NpcDefId},
    world::coordinate_system::Point,
};

/// Game Outcome is its own data type to be used for outcomes within the rules of the game.
/// When the rules of the game check if something is allowed, this should be used.
///
/// Example: A player tries to move into a wall. The game returns `GameOutcome::Fail(FailReason::TileNotWalkable)`, stating that this movement is illegal.
pub enum GameOutcome {
    /// The action is successful and can be completed.
    Success,

    /// The action was unsuccessful for the given reason.
    /// The action will not be completed.
    Fail(FailReason),
}

/// Reasons for which a [GameOutcome] returns a `GameOutcome::Fail`.
///
/// Any thing forbidden by the game rules are implemented here.
pub enum FailReason {
    /// Action cannot be completed because the specified points is out of bounds.
    /// (e.g. teleporting to negative coordinates like x: -1, y: 1)
    PointOutOfBounds(Point),

    /// Action cannot be completed because the target tile is not walkable.
    /// (e.g. walking against a wall)
    TileNotWalkable(Point),

    /// Action cannot be completed the inventory cannot take in any more items.
    /// (e.g. picking up an item while the inventory is full)
    InventoryFull,

    /// Action cannot be completed because the slot is already empty. Used in unequipping logic.
    /// (e.g. trying to unequip armor while not wearing armor)
    CannotUnequipEmptySlot,
}

impl FailReason {
    /// This is a function that defines how (if at all) a user should be notified of the `GameOutcome::Fail(_)`.
    /// Some things should be told to the player (like the inventory being full).
    /// Other things go without saying (lik enot being able to walk into a wall).
    pub fn notify_user(&self) -> Option<String> {
        match self {
            FailReason::PointOutOfBounds(_) => None,
            FailReason::TileNotWalkable(_) => None,
            FailReason::InventoryFull => {
                Some("Your inventory is full. Cannot add another item.".to_string())
            }
            FailReason::CannotUnequipEmptySlot => {
                Some("The equipment slot is already empty. Cannot unequip.".to_string())
            }
        }
    }
}

/// Result type, which is used for cases where both [GameOutcome] is tested and a [GameError] can occur.
///
/// The cases are ternary:
/// * Program fails: `Err(...)`
/// * Program succeeds + game rule is violated: `Ok(GameOutcome::Fail(...))`
/// * Program succeeds + game succeeds: `Ok(GameOutcome::Success)`
pub type GameResult = Result<GameOutcome, GameError>;

/// Central errors that can occur in the game, wrapped by this type.
#[derive(Debug)]
pub enum GameError {
    /// Wrapper for [EngineError]
    ///
    /// Failure of the game's central engine representing the player, npcs, world and more.
    Engine(EngineError),

    /// Wrapper for [DataError]
    ///
    /// Failure of the game's data. Like if game objects with invalid ids or non-existent static data is accessed.
    Data(DataError),

    /// Wrapper for [IoError]
    ///
    /// Failure of the game's IO. (Loading, saving, parsing save files)
    Io(IoError),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::Engine(error) => {
                write!(f, "Engine Error: {}", error)
            }
            GameError::Data(error) => {
                write!(f, "Data Error: {}", error)
            }
            GameError::Io(error) => {
                write!(f, "Io Error: {}", error)
            }
        }
    }
}

/// Failure of the game's central engine representing the player, npcs, world and more.
#[derive(Debug)]
pub enum EngineError {
    /// An item of the given id is not regstered in `GameState::items`, meaning it doesn't exist.
    UnregisteredItem(GameItemId),

    /// An item of the given kind is not of a valid kind for an action.
    ///
    /// Example: A sword cannot be put into the armor slot.
    InvalidItem(GameItemKindDef),

    /// An Npc of the given id is not registered in `GameState::npcs`, meaning it doesn't exist.
    NpcNotFound(EntityId),

    /// An ItemSprite of the given id is not registered in `GameState::item_sprites`, meaning it doesn't exist.
    ItemSpriteNotFound(EntityId),

    /// An Item that is being used by the player is not in their inventory.
    ItemNotInInventory(GameItemId),

    /// Spawning an entity at the given point failed.
    SpawningError(Point),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::UnregisteredItem(item_id) => {
                write!(f, "No item of id {} registered", item_id)
            }
            EngineError::InvalidItem(item_kind) => {
                write!(f, "Item of kind {:?} cannot be used for this action.", item_kind)
            }
            EngineError::NpcNotFound(npc_id) => {
                write!(f, "Npc of id {} does not exist", npc_id)
            }
            EngineError::ItemSpriteNotFound(sprite_id) => {
                write!(f, "Item Sprite of id {} does not exist", sprite_id)
            }
            EngineError::ItemNotInInventory(item_id) => {
                write!(
                    f,
                    "Inventory operation not possible, because item of id {} is not in inventory.",
                    item_id
                )
            }
            EngineError::SpawningError(point) => {
                write!(
                    f,
                    "Could not spawn entity, because point (x: {}, y: {}) is not available",
                    point.x, point.y
                )
            }
        }
    }
}

impl From<EngineError> for GameError {
    fn from(engine_error: EngineError) -> Self {
        GameError::Engine(engine_error)
    }
}

/// Failure of the game's data. Like if game objects with invalid ids or non-existent static data is accessed.
#[derive(Debug)]
pub enum DataError {
    /// The item of the given [GameItemDefId] does not exist in the game.
    MissingItemDefinition(GameItemDefId),

    /// The npc of the given [NpcDefId] does not exist in the game.
    MissingNpcDefinition(NpcDefId),

    /// Tried to load static world, but no static world defined for id
    StaticWorldNotFound(usize),

    /// World needs to fit requirements to be loaded.
    /// * Cannot be larger than `WORLD_WIDTH`x`WORLD_HEIGHT`
    InvalidWorldFormat(usize),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::MissingItemDefinition(item_def_id) => {
                write!(f, "Item of def_id {} not defined", item_def_id)
            }
            DataError::MissingNpcDefinition(npc_def_id) => {
                write!(f, "Npc of def_id {} not defined", npc_def_id)
            }
            DataError::StaticWorldNotFound(static_world_id) => {
                write!(f, "No static world definied for id {}", static_world_id)
            }
            DataError::InvalidWorldFormat(static_world_id) => {
                write!(f, "WorldData for {} does not fit requirements", static_world_id)
            }
        }
    }
}

impl From<DataError> for GameError {
    fn from(data_error: DataError) -> Self {
        GameError::Data(data_error)
    }
}

/// Failure of the game's IO. (Loading, saving, parsing save files)
#[derive(Debug)]
pub enum IoError {
    /// Reading the map file from the assets has failed.
    MapReadFailed(io::Error),

    /// Parsing the map file from the assets for its .ron structure failed.
    MapParseFailed(SpannedError),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::MapReadFailed(error) => {
                write!(f, "Couldn't load map file: {}", error)
            }
            IoError::MapParseFailed(error) => {
                write!(f, "Couldn't parse map file: {}", error)
            }
        }
    }
}

impl From<IoError> for GameError {
    fn from(io_error: IoError) -> Self {
        GameError::Io(io_error)
    }
}
