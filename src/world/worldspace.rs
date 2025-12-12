#![allow(dead_code)]

use ratatui::style::Color;

use crate::core::game::{GameState, ItemSprite, Npc};

pub const WORLD_WIDTH: usize = 100;
pub const WORLD_HEIGHT: usize = 25;

pub trait Drawable {
    fn glyph(&self) -> char;
    fn color(&self) -> Color;
}

pub trait Collision {
    fn is_walkable(&self) -> bool;
}

// ----------------------------------------------
//                     Tiles
//       Units of which the world is made
// ----------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub visible: bool,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self { tile_type, visible: false }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self { tile_type: TileType::Wall, visible: false }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Floor,
    Wall,
    Door { open: bool },
}

impl Collision for TileType {
    fn is_walkable(&self) -> bool {
        match self {
            TileType::Floor => true,
            TileType::Wall => false,
            TileType::Door { open: true } => true,
            TileType::Door { open: false } => false,
        }
    }
}

impl Drawable for TileType {
    fn glyph(&self) -> char {
        match self {
            TileType::Floor => '.',
            TileType::Wall => '#',
            TileType::Door { open: true } => '_',
            TileType::Door { open: false } => '+',
        }
    }
    fn color(&self) -> Color {
        match self {
            TileType::Floor => Color::White,
            TileType::Wall => Color::White,
            TileType::Door { open: _ } => Color::Yellow,
        }
    }
}

// ----------------------------------------------
//                Coordinates & Rooms
// ----------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Room {
    pub origin: Point,
    pub width: usize,
    pub height: usize,
}

impl Room {
    pub fn new(origin: Point, width: usize, height: usize) -> Self {
        Self { origin, width, height }
    }
}

// ----------------------------------------------
//                World Struct
// ----------------------------------------------

pub struct World {
    pub width: usize,
    pub height: usize,
    pub tiles: [Tile; WORLD_WIDTH * WORLD_HEIGHT], // Grid is 100 wide and 25 high.
    pub npcs: Vec<Npc>,
    pub items: Vec<ItemSprite>,
}

impl World {
    pub fn new(_game: &mut GameState) -> Self {
        Self {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles: [Tile::default(); WORLD_WIDTH * WORLD_HEIGHT],
            npcs: Vec::new(),
            items: Vec::new(),
        }
    }

    // helper constructor to create a placeholder world
    pub fn empty() -> Self {
        Self { width: WORLD_WIDTH, height: WORLD_HEIGHT, tiles: [Tile::default(); WORLD_WIDTH * WORLD_HEIGHT], npcs: Vec::new(), items: Vec::new() }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        let index = self.index(x, y);
        &self.tiles[index]
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        let index = self.index(x, y);
        &mut self.tiles[index]
    }

    pub fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        let in_lower_bounds: bool = x >= 0 && y >= 0;
        let in_upper_bounds: bool = (x as usize) < self.width && (y as usize) < self.height;

        in_lower_bounds && in_upper_bounds
    }

    pub fn is_taken(&self, pos: Point) -> bool {
        self.npcs.iter().any(|npc| npc.base.pos == pos) || self.items.iter().any(|item| item.base.pos == pos)
    }
}
