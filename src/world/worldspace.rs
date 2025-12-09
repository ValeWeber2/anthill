#![allow(dead_code)]

use ratatui::style::Color;

use crate::core::game::{ItemSprite, Npc};

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
    pub fn new() -> Self {
        Self {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles: [Tile::default(); WORLD_WIDTH * WORLD_HEIGHT],
            npcs: Vec::new(),
            items: Vec::new(),
        }
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

    pub fn carve_room(&mut self, room: &Room) {
        for y in room.origin.y..room.origin.y + room.height {
            for x in room.origin.x..room.origin.x + room.width {
                self.get_tile_mut(x, y).tile_type = TileType::Floor;
            }
        }
    }

    pub fn add_wall_border(&mut self) {
        for x in 0..self.width {
            self.get_tile_mut(x, 0).tile_type = TileType::Wall;
            self.get_tile_mut(x, self.height - 1).tile_type = TileType::Wall;
        }
        for y in 0..self.height {
            self.get_tile_mut(0, y).tile_type = TileType::Wall;
            self.get_tile_mut(self.width - 1, y).tile_type = TileType::Wall;
        }
    }

    pub fn add_npc(&mut self, npc: Npc) {
        self.npcs.push(npc);
    }

    pub fn add_item(&mut self, item: ItemSprite) {
        self.items.push(item);
    }

    pub fn can_move_to(&self, pos: Point) -> bool {
        self.is_in_bounds(pos.x as isize, pos.y as isize) && self.get_tile(pos.x, pos.y).tile_type.is_walkable()
    }
}
