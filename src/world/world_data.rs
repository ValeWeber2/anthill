use serde::{Deserialize, Serializ};

#[derive(Debug, Clone, Serialize, Deserialze)]
pub struct WorldData {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileData {
    pub x: usize,
    pub y: usize,
    pub tile_type: TileTypeData,
}

#[derive(Debug, Clone, Serialize, Deserialze)]
pub enum TileTypeData {
    Floor,
    Wall,
    Door {open: bool},
}