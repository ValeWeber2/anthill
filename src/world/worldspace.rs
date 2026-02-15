#![allow(dead_code)]

use std::fmt::{self, Display, Formatter};

use crate::world::coordinate_system::Point;
use crate::world::tiles::{DoorType, Tile, TileType};

pub const WORLD_WIDTH: usize = 100;
pub const WORLD_HEIGHT: usize = 25;

// ----------------------------------------------
//                     Rooms
// ----------------------------------------------
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
        Point::new(self.origin.x + self.width / 2, self.origin.y + self.height / 2)
    }

    pub fn left(&self) -> usize {
        self.origin.x
    }
    pub fn right(&self) -> usize {
        self.origin.x + self.width - 1
    }
    pub fn top(&self) -> usize {
        self.origin.y
    }
    pub fn bottom(&self) -> usize {
        self.origin.y + self.height - 1
    }
}

// ----------------------------------------------
//                World Struct
// ----------------------------------------------

pub struct World {
    pub width: usize,
    pub height: usize,
    pub tiles: [Tile; WORLD_WIDTH * WORLD_HEIGHT], // Grid is 100 wide and 25 high.
}

impl World {
    pub fn new() -> Self {
        Self {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles: [Tile::default(); WORLD_WIDTH * WORLD_HEIGHT],
        }
    }

    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_tile(&self, point: Point) -> &Tile {
        let index = self.index(point.x, point.y);
        &self.tiles[index]
    }

    pub fn get_tile_mut(&mut self, point: Point) -> &mut Tile {
        let index = self.index(point.x, point.y);
        &mut self.tiles[index]
    }

    pub fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        let in_lower_bounds: bool = x >= 0 && y >= 0;
        let in_upper_bounds: bool = (x as usize) < self.width && (y as usize) < self.height;

        in_lower_bounds && in_upper_bounds
    }

    pub fn get_points_in_radius(&self, pos: Point, radius: isize) -> Vec<Point> {
        let mut points = Vec::new();
        let x = pos.x as isize;
        let y = pos.y as isize;

        let min_x = (x - radius).max(0);
        let max_x = (x + radius).min(self.width as isize - 1);
        let min_y = (y - radius).max(0);
        let max_y = (y + radius).min(self.height as isize - 1);

        for i in min_x..=max_x {
            for j in min_y..=max_y {
                if ((x - i).pow(2) + (y - j).pow(2) - radius.pow(2)) <= radius.pow(2) {
                    points.push(Point::new(i as usize, j as usize));
                }
            }
        }

        points
    }

    pub fn open_room_for_hallway(&mut self) {
        let dirs: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for y in 0..self.height {
            for x in 0..self.width {
                let hall = Point::new(x, y);
                if self.get_tile(hall).tile_type != TileType::Hallway {
                    continue;
                }

                for (dx, dy) in dirs {
                    let wx = x as isize + dx;
                    let wy = y as isize + dy;
                    if !self.is_in_bounds(wx, wy) {
                        continue;
                    }
                    let wall_p = Point::new(wx as usize, wy as usize);

                    if self.get_tile(wall_p).tile_type != TileType::Wall {
                        continue;
                    }

                    let bx = wx + dx;
                    let by = wy + dy;
                    if !self.is_in_bounds(bx, by) {
                        continue;
                    }
                    let behind = Point::new(bx as usize, by as usize);

                    if self.get_tile(behind).tile_type == TileType::Floor {
                        self.get_tile_mut(wall_p).tile_type = TileType::Door(DoorType::Archway);
                    }
                }
            }
        }
    }

    pub fn add_walls_around_walkables(&mut self) {
        let mut to_wall: Vec<Point> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let tt = self.get_tile(Point::new(x, y)).tile_type;

                if matches!(tt, TileType::Floor | TileType::Door(_)) {
                    let y0 = y.saturating_sub(1);
                    let y1 = (y + 1).min(self.height - 1);
                    let x0 = x.saturating_sub(1);
                    let x1 = (x + 1).min(self.width - 1);

                    for ny in y0..=y1 {
                        for nx in x0..=x1 {
                            if nx == x && ny == y {
                                continue;
                            }

                            let new_point = Point::new(nx, ny);
                            if self.get_tile(new_point).tile_type == TileType::Void {
                                to_wall.push(new_point);
                            }
                        }
                    }
                }
            }
        }

        for point in to_wall {
            if self.get_tile(point).tile_type == TileType::Void {
                self.get_tile_mut(point).tile_type = TileType::Wall;
            }
        }
    }

    /// Carves a rectangular room into the map.
    ///
    /// Fills the interior with `Floor` tiles and surrounds it with `Wall` tiles
    /// based on the room’s origin, width, and height.
    pub fn carve_room(&mut self, room: &Room) {
        let ox = room.origin.x;
        let oy = room.origin.y;
        let w = room.width;
        let h = room.height;

        for y in oy + 1..oy + h {
            for x in ox + 1..ox + w {
                self.get_tile_mut(Point::new(x, y)).tile_type = TileType::Floor;
            }
        }

        for y in oy..=oy + h {
            self.get_tile_mut(Point::new(ox, y)).tile_type = TileType::Wall;
            self.get_tile_mut(Point::new(ox + w, y)).tile_type = TileType::Wall;
        }

        for x in ox..=ox + w {
            self.get_tile_mut(Point::new(x, oy)).tile_type = TileType::Wall;
            self.get_tile_mut(Point::new(x, oy + h)).tile_type = TileType::Wall;
        }
    }
}

impl Default for World {
    /// helper constructor to create a placeholder world
    fn default() -> Self {
        Self {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles: [Tile::default(); WORLD_WIDTH * WORLD_HEIGHT],
        }
    }
}

#[derive(Debug)]
pub enum MovementError {
    OutOfBounds { x: isize, y: isize },
    NotWalkable { x: isize, y: isize },
}

impl Display for MovementError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MovementError::OutOfBounds { x, y } => {
                write!(f, "Position x: {} y: {} out of bounds", x, y)
            }
            MovementError::NotWalkable { x, y } => {
                write!(f, "Position x: {} y: {} not walkable", x, y)
            }
        }
    }
}
