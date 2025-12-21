#![allow(dead_code)]

use std::collections::HashMap;

use ratatui::style::Color;

use crate::core::game::{EntityId, GameState, ItemSprite, Npc};

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

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn center(&self) -> Point {
        Point {
            x: self.origin.x + self.width / 2,
            y: self.origin.y + self.height / 2,
        }
    }

    pub fn left(&self) -> usize { self.origin.x }
    pub fn right(&self) -> usize { self.origin.x + self.width - 1 }
    pub fn top(&self) -> usize { self.origin.y }
    pub fn bottom(&self) -> usize { self.origin.y + self.height - 1 }
}

// ----------------------------------------------
//                World Struct
// ----------------------------------------------

pub struct World {
    pub width: usize,
    pub height: usize,
    pub tiles: [Tile; WORLD_WIDTH * WORLD_HEIGHT], // Grid is 100 wide and 25 high.
    pub npcs: Vec<Npc>,
    pub npc_index: HashMap<EntityId, usize>,
    pub items: Vec<ItemSprite>,
    pub item_index: HashMap<EntityId, usize>
}

impl World {
    pub fn new(_game: &mut GameState) -> Self {
        Self {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles: [Tile::default(); WORLD_WIDTH * WORLD_HEIGHT],
            npcs: Vec::new(),
            npc_index: HashMap::new(),
            items: Vec::new(),
            item_index: HashMap::new(),
        }
    }

    // helper constructor to create a placeholder world
    pub fn default() -> Self {
        Self {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles: [Tile::default(); WORLD_WIDTH * WORLD_HEIGHT],
            npcs: Vec::new(),
            npc_index: HashMap::new(),
            items: Vec::new(),
            item_index: HashMap::new(),
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

    pub fn is_available(&self, pos: Point) -> bool {
        self.is_in_bounds(pos.x as isize, pos.y as isize)
            && self.npcs.iter().all(|npc| npc.base.pos != pos)
            && self.items.iter().all(|item| item.base.pos != pos)
            && self.get_tile(pos.x, pos.y).tile_type.is_walkable()
    }

    // could be used in combat system or graphics
    pub fn get_points_in_radius(&self, pos: Point, radius: usize) -> Vec<Point> {
        let mut points = Vec::new();
        let x = pos.x;
        let y = pos.y;
        const TOLERANCE: f32 = 0.5;

        for i in x - radius..=x + radius {
            for j in y - radius..=y + radius {
                if self.is_in_bounds(i as isize, j as isize)
                    && ((x - i).pow(2) + (y - j).pow(2) - radius.pow(2)) as f32 <= TOLERANCE
                {
                    points.push(Point::new(i, j));
                }
            }
        }

        points
    }

    pub fn carve_room(&mut self, room: &Room) {
        let ox = room.origin.x;
        let oy = room.origin.y;
        let w = room.width;
        let h = room.height;

        // fill interior with floor
        for y in oy+1 .. oy + h - 1 {
            for x in ox+1 .. ox + w - 1 {
                self.get_tile_mut(x, y).tile_type = TileType::Floor;
            }
        }

        // vertical walls
        for y in oy .. oy + h {
            self.get_tile_mut(ox, y).tile_type = TileType::Wall;
            self.get_tile_mut(ox + w - 1, y).tile_type = TileType::Wall;
        }

        // horizontal walls
        for x in ox .. ox + w {
            self.get_tile_mut(x, oy).tile_type = TileType::Wall;
            self.get_tile_mut(x, oy + h - 1).tile_type = TileType::Wall;
        }
    }

    // create an L-shaped corridor between two rooms and add doors
    pub fn connect_rooms(&mut self, room1: &Room, room2: &Room) {
        self.carve_h_corridor(room1.center().x, room2.center().x, room1.center().y);
        self.carve_v_corridor(room1.center().y, room2.center().y, room2.center().x);
        
        self.place_doors_between(room1, room2);
    }

    pub fn carve_h_corridor(&mut self, x1: usize, x2: usize, y: usize) {
        let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        for x in start..=end {
            self.get_tile_mut(x, y).tile_type = TileType::Floor;
        }
    }

    pub fn carve_v_corridor(&mut self, y1: usize, y2: usize, x: usize) {
        let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        for y in start..=end {
            self.get_tile_mut(x, y).tile_type = TileType::Floor;
        }
    }

    pub fn place_doors_between(&mut self, room1: &Room, room2: &Room) {
        let c1 = room1.center();
        let c2 = room2.center();

        // horizontal corridor entry points
        let hx1 = if c1.x < c2.x { room1.right() } else { room1.left() };
        let hx2 = if c1.x < c2.x { room2.left() } else { room2.right() };

        // vertical corridor entry points
        let vy1 = if c1.y < c2.y { room1.bottom() } else { room1.top() };
        let vy2 = if c1.y < c2.y { room2.top() } else { room2.bottom() };

        // try placing doors
        self.try_place_door(hx1, c1.y);
        self.try_place_door(hx2, c1.y);
        self.try_place_door(c2.x, vy1);
        self.try_place_door(c2.x, vy2);
    }

    // places a door on a wall tile if it has two opposite walkable neighbours
    pub fn try_place_door(&mut self, x: usize, y: usize) {
        if self.get_tile(x, y).tile_type != TileType::Wall {
            return;
        }

        let walkable_left  = self.get_tile(x - 1, y).tile_type.is_walkable();
        let walkable_right = self.get_tile(x + 1, y).tile_type.is_walkable();
        let walkable_up    = self.get_tile(x, y - 1).tile_type.is_walkable();
        let walkable_down  = self.get_tile(x, y + 1).tile_type.is_walkable();

        let horizontal = walkable_left && walkable_right;
        let vertical   = walkable_up && walkable_down;

        if horizontal || vertical {
            self.get_tile_mut(x, y).tile_type = TileType::Door { open: false };
        }
    }

}
