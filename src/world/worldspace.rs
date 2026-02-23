use crate::world::coordinate_system::Point;
use crate::world::tiles::{Tile, TileType};

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

    /// Function to get an index for the 1-dimensional [World::tiles] array using x- and y-coordinates.
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

    /// Checks whether coordinates are in bounds of the world or outside.
    ///
    /// Excplicitly requies isize, to check for negative values (which would be out of bounds).
    pub fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        let in_lower_bounds: bool = x >= 0 && y >= 0;
        let in_upper_bounds: bool = (x as usize) < self.width && (y as usize) < self.height;

        in_lower_bounds && in_upper_bounds
    }

    /// Returns a vector of [Point]s within the given radius around the given point of origin.
    pub fn get_points_in_radius(&self, point: Point, radius: isize) -> Vec<Point> {
        let mut points = Vec::new();
        let x = point.x as isize;
        let y = point.y as isize;

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
    fn default() -> Self {
        Self {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles: [Tile::default(); WORLD_WIDTH * WORLD_HEIGHT],
        }
    }
}
