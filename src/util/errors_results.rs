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
/// Example: A player tries to move into a wall. The game returns GameResultFail(_), stating that this movement is illegal.
pub enum GameOutcome {
    Success,
    Fail(FailReason),
}

pub enum FailReason {
    PointOutOfBounds(Point),
    TileNotWalkable(Point),
    InventoryFull,
    CannotUnequipEmptySlot,
}

impl FailReason {
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

/// Generally used for actions where both a GameOutcome is tested and a GameError can be thrown.
pub type GameResult = Result<GameOutcome, GameError>;

#[derive(Debug)]
pub enum GameError {
    Engine(EngineError),
    Data(DataError),
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

#[derive(Debug)]
pub enum EngineError {
    UnregisteredItem(GameItemId),
    InvalidItem(GameItemKindDef),
    NpcNotFound(EntityId),
    ItemSpriteNotFound(EntityId),
    ItemNotInInventory(GameItemId),
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

#[derive(Debug)]
pub enum DataError {
    MissingItemDefinition(GameItemDefId),
    MissingNpcDefinition(NpcDefId),
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
        }
    }
}

impl From<DataError> for GameError {
    fn from(data_error: DataError) -> Self {
        GameError::Data(data_error)
    }
}

#[derive(Debug)]
pub enum IoError {
    FileReading(io::Error),
    MapParsing(SpannedError),
    FileCreation(io::Error),
    MapWriting(ron::Error),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::FileReading(error) => {
                write!(f, "Couldn't load map file: {}", error)
            }
            IoError::MapParsing(error) => {
                write!(f, "Couldn't parse map file: {}", error)
            }
            IoError::FileCreation(error) => {
                write!(f, "Couldn't open map file to save: {}", error)
            }
            IoError::MapWriting(error) => {
                write!(f, "Couldn't open map file to save: {}", error)
            }
        }
    }
}

impl From<IoError> for GameError {
    fn from(io_error: IoError) -> Self {
        GameError::Io(io_error)
    }
}
