#![allow(dead_code)]

use std::fmt::{self, Display, Formatter};

use ratatui::style::Style;

use crate::util::errors_results::{FailReason, GameOutcome, GameResult};
use crate::world::coordinate_system::Point;
use crate::{
    core::{
        entity_logic::{Entity, Movable},
        game::GameState,
    },
    world::tiles::{DoorType, Tile, TileType},
};

use crate::world::world_data::{TileTypeData, WorldData};

pub const WORLD_WIDTH: usize = 100;
pub const WORLD_HEIGHT: usize = 25;

pub trait Drawable {
    fn glyph(&self) -> char;
    fn style(&self) -> Style;
}

pub trait Collision {
    fn is_walkable(&self) -> bool;
}

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
    pub fn new(_game: &mut GameState) -> Self {
        Self {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles: [Tile::default(); WORLD_WIDTH * WORLD_HEIGHT],
        }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_tile(&self, pos: Point) -> &Tile {
        let index = self.index(pos.x, pos.y);
        &self.tiles[index]
    }

    pub fn get_tile_mut(&mut self, pos: Point) -> &mut Tile {
        let index = self.index(pos.x, pos.y);
        &mut self.tiles[index]
    }

    pub fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        let in_lower_bounds: bool = x >= 0 && y >= 0;
        let in_upper_bounds: bool = (x as usize) < self.width && (y as usize) < self.height;

        in_lower_bounds && in_upper_bounds
    }

    // could be used in combat system or graphics
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

    pub fn carve_room(&mut self, room: &Room) {
        let ox = room.origin.x;
        let oy = room.origin.y;
        let w = room.width;
        let h = room.height;

        for y in oy + 1..oy + h - 1 {
            for x in ox + 1..ox + w - 1 {
                self.get_tile_mut(Point::new(x, y)).tile_type = TileType::Floor;
            }
        }

        for y in oy..oy + h {
            self.get_tile_mut(Point::new(ox, y)).tile_type = TileType::Wall;
            self.get_tile_mut(Point::new(ox + w - 1, y)).tile_type = TileType::Wall;
        }

        for x in ox..ox + w {
            self.get_tile_mut(Point::new(x, oy)).tile_type = TileType::Wall;
            self.get_tile_mut(Point::new(x, oy + h - 1)).tile_type = TileType::Wall;
        }
    }

    pub fn move_player_character<E: Entity + Movable>(
        &self,
        entity: &mut E,
        dx: i32,
        dy: i32,
    ) -> GameResult {
        let new_x = entity.pos().x as isize + dx as isize;
        let new_y = entity.pos().y as isize + dy as isize;
        let new_point = Point::new(new_x as usize, new_y as usize);

        if !self.is_in_bounds(new_x, new_y) {
            return Ok(GameOutcome::Fail(FailReason::PointOutOfBounds(new_point)));
        }

        if !self.get_tile(Point::new(new_x as usize, new_y as usize)).tile_type.is_walkable() {
            return Ok(GameOutcome::Fail(FailReason::TileNotWalkable(new_point)));
        }

        entity.move_to(new_point);

        Ok(GameOutcome::Success)
    }

    pub fn apply_world_data(&mut self, data: &WorldData) -> Result<(), &'static str> {
        if data.width != self.width || data.height != self.height {
            return Err("WorldData dimensions do not match current ones");
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
                return Err("WorldData contains tile out of bounds");
            }

            let idx = self.index(td.x, td.y);

            let tile_type = match td.tile_type {
                TileTypeData::Floor => TileType::Floor,
                TileTypeData::Wall => TileType::Wall,
                TileTypeData::Hallway => TileType::Hallway,
                TileTypeData::Stair => TileType::Stair,
                TileTypeData::Door { open } => {
                    TileType::Door(if open { DoorType::Open } else { DoorType::Closed })
                }
            };

            self.tiles[idx] = Tile::new(tile_type);
        }

        self.open_room_for_hallway();
        self.add_walls_around_walkables();

        Ok(())
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

impl GameState {
    pub fn is_available(&self, pos: Point) -> bool {
        self.world.is_in_bounds(pos.x as isize, pos.y as isize)
            && self.npcs.iter().all(|npc| npc.base.pos != pos)
            && self.item_sprites.iter().all(|item| item.base.pos != pos)
            && self.world.get_tile(pos).tile_type.is_walkable()
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
