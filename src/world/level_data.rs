use serde::{Deserialize, Serialize};

use crate::{
    util::errors_results::{DataError, GameError},
    world::{
        coordinate_system::Point,
        tiles::{DoorType, Tile, TileType},
        worldspace::{Room, World},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    pub width: usize,
    pub height: usize,

    #[serde(default)]
    pub tiles: Vec<TileData>,

    #[serde(default)]
    pub rooms: Vec<RoomData>,

    #[serde(default)]
    pub corridors: Vec<Point>,

    #[serde(default)]
    pub entry: Point,

    #[serde(default)]
    pub exit: Point,

    #[serde(default)]
    pub spawns: Vec<SpawnData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomData {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileData {
    pub x: usize,
    pub y: usize,
    pub tile_type: TileTypeData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileTypeData {
    Floor,
    Wall,
    Hallway,
    Door(DoorTypeData),
    StairsDown,
    StairsUp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DoorTypeData {
    Open,
    Closed,
    Archway,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnData {
    pub kind: SpawnKind,
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnKind {
    Npc { def_id: String },
    Item { def_id: String },
}

impl World {
    /// Applies level data to a world.
    ///
    /// This level data might come from one of the world level files or a procedurally generated world.
    ///
    /// # Errors
    /// * [DataError::InvalidWorldFormat] if the world format is corrupted and cannot be read.
    pub fn apply_level_data(&mut self, data: &LevelData, index: usize) -> Result<(), GameError> {
        if data.width != self.width || data.height != self.height {
            return Err(GameError::from(DataError::InvalidWorldFormat(index)));
        }

        for t in self.tiles.iter_mut() {
            *t = Tile::default();
        }

        for r in &data.rooms {
            let room = Room::new(Point::new(r.x, r.y), r.width, r.height);
            self.carve_room(&room);
        }

        for td in &data.tiles {
            if td.x >= self.width || td.y >= self.height {
                return Err(GameError::from(DataError::InvalidWorldFormat(index)));
            }

            let idx = self.index(td.x, td.y);

            let tile_type = match td.tile_type {
                TileTypeData::Floor => TileType::Floor,
                TileTypeData::Wall => TileType::Wall,
                TileTypeData::Hallway => TileType::Hallway,
                TileTypeData::StairsDown => TileType::StairsDown,
                TileTypeData::StairsUp => TileType::StairsUp,
                TileTypeData::Door(DoorTypeData::Archway) => TileType::Door(DoorType::Archway),
                TileTypeData::Door(DoorTypeData::Open) => TileType::Door(DoorType::Open),
                TileTypeData::Door(DoorTypeData::Closed) => TileType::Door(DoorType::Closed),
            };

            self.tiles[idx] = Tile::new(tile_type);
        }

        for corridor_point in &data.corridors {
            let updated_tile = match self.get_tile(*corridor_point).tile_type {
                TileType::Void => TileType::Hallway,
                TileType::Wall => TileType::Door(DoorType::Archway),
                other => other,
            };

            self.get_tile_mut(*corridor_point).tile_type = updated_tile;
        }

        Ok(())
    }
}
