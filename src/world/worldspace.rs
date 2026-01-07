#![allow(dead_code)]

use std::fmt::{self, Display, Formatter};
use std::{collections::HashMap, ops::Add};

use ratatui::style::Style;

use crate::ai::npc_ai::NpcAiError;
use crate::{
    core::{
        entity_logic::{Entity, EntityId, Movable, Npc},
        game::GameState,
        game_items::GameItemSprite,
    },
    world::tiles::{Tile, TileType},
};

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

    pub fn get_adjacent(self, direction: Direction) -> Point {
        self + PointDelta::from(direction)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Add<PointDelta> for Point {
    type Output = Point;

    fn add(self, delta: PointDelta) -> Point {
        let new_x = self.x as isize + delta.x;
        let new_y = self.y as isize + delta.y;

        Point { x: new_x.max(0) as usize, y: new_y.max(0) as usize }
    }
}

pub struct PointDelta {
    pub x: isize,
    pub y: isize,
}

impl From<Direction> for PointDelta {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => PointDelta { x: 0, y: -1 },
            Direction::Right => PointDelta { x: 1, y: 0 },
            Direction::Down => PointDelta { x: 0, y: 1 },
            Direction::Left => PointDelta { x: -1, y: 0 },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
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
        Point { x: self.origin.x + self.width / 2, y: self.origin.y + self.height / 2 }
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
    pub npcs: Vec<Npc>,
    pub npc_index: HashMap<EntityId, usize>,
    pub item_sprites: Vec<GameItemSprite>,
    pub item_sprites_index: HashMap<EntityId, usize>,
}

impl World {
    pub fn new(_game: &mut GameState) -> Self {
        Self {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles: [Tile::default(); WORLD_WIDTH * WORLD_HEIGHT],
            npcs: Vec::new(),
            npc_index: HashMap::new(),
            item_sprites: Vec::new(),
            item_sprites_index: HashMap::new(),
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
            && self.item_sprites.iter().all(|item| item.base.pos != pos)
            && self.get_tile(pos.x, pos.y).tile_type.is_walkable()
    }

    pub fn get_npc(&self, id: EntityId) -> Option<&Npc> {
        self.npc_index.get(&id).map(|&index| &self.npcs[index])
    }

    pub fn get_npc_mut(&mut self, id: EntityId) -> Option<&mut Npc> {
        self.npc_index.get(&id).map(|&index| &mut self.npcs[index])
    }

    pub fn get_item_sprite(&self, id: EntityId) -> Option<&GameItemSprite> {
        self.item_sprites_index.get(&id).map(|&index| &self.item_sprites[index])
    }

    pub fn get_item_sprite_mut(&mut self, id: EntityId) -> Option<&mut GameItemSprite> {
        self.item_sprites_index.get(&id).map(|&index| &mut self.item_sprites[index])
    }

    // could be used in combat system or graphics
    pub fn get_points_in_radius(&self, pos: &Point, radius: isize) -> Vec<Point> {
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

    pub fn carve_room(&mut self, room: &Room) {
        let ox = room.origin.x;
        let oy = room.origin.y;
        let w = room.width;
        let h = room.height;

        for y in oy + 1..oy + h - 1 {
            for x in ox + 1..ox + w - 1 {
                self.get_tile_mut(x, y).tile_type = TileType::Floor;
            }
        }

        for y in oy..oy + h {
            self.get_tile_mut(ox, y).tile_type = TileType::Wall;
            self.get_tile_mut(ox + w - 1, y).tile_type = TileType::Wall;
        }

        for x in ox..ox + w {
            self.get_tile_mut(x, oy).tile_type = TileType::Wall;
            self.get_tile_mut(x, oy + h - 1).tile_type = TileType::Wall;
        }
    }

    pub fn move_player_character<E: Entity + Movable>(
        &self,
        entity: &mut E,
        dx: i32,
        dy: i32,
    ) -> Result<(), MovementError> {
        let new_x = entity.pos().x as isize + dx as isize;
        let new_y = entity.pos().y as isize + dy as isize;

        if !self.is_in_bounds(new_x, new_y) {
            return Err(MovementError::OutOfBounds { x: new_x, y: new_y });
        }

        if !self.get_tile(new_x as usize, new_y as usize).tile_type.is_walkable() {
            return Err(MovementError::NotWalkable { x: new_x, y: new_y });
        }

        entity.move_to(Point { x: new_x as usize, y: new_y as usize });
        Ok(())
    }

    pub fn move_npc(&mut self, npc_id: EntityId, dx: isize, dy: isize) -> Result<(), NpcAiError> {
        let (new_x, new_y) = {
            let npc = self.get_npc(npc_id).ok_or(NpcAiError::NpcNotFound)?;

            let new_x = npc.pos().x as isize + dx;
            let new_y = npc.pos().y as isize + dy;

            if !self.is_in_bounds(new_x, new_y) {
                return Err(NpcAiError::MovementError(MovementError::OutOfBounds {
                    x: new_x,
                    y: new_y,
                }));
            }

            if !self.get_tile(new_x as usize, new_y as usize).tile_type.is_walkable() {
                return Err(NpcAiError::MovementError(MovementError::NotWalkable {
                    x: new_x,
                    y: new_y,
                }));
            }

            (new_x, new_y)
        };

        let npc = self.get_npc_mut(npc_id).ok_or(NpcAiError::NpcNotFound)?;
        npc.move_to(Point { x: new_x as usize, y: new_y as usize });

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
            npcs: Vec::new(),
            npc_index: HashMap::new(),
            item_sprites: Vec::new(),
            item_sprites_index: HashMap::new(),
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
