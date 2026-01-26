use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldData {
    pub width: usize,
    pub height: usize,

    #[serde(default)]
    pub tiles: Vec<TileData>,

    #[serde(default)]
    pub rooms: Vec<RoomData>,

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
    Stair,
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
