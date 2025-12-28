#![allow(dead_code)]

use std::{collections::HashMap, ops::Add};

use ratatui::style::Style;

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

    pub fn get_neighbour(self, direction: Direction) -> Point {
        match direction {
            Direction::Up => Point { x: self.x, y: self.y.saturating_sub(1) },
            Direction::Right => Point { x: self.x + 1, y: self.y },
            Direction::Down => Point { x: self.x, y: self.y + 1 },
            Direction::Left => Point { x: self.x.saturating_sub(1), y: self.y },
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
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
    pub fn move_entity<E: Entity + Movable>(
        &mut self,
        entity: &mut E,
        dx: i32,
        dy: i32,
    ) -> Result<(), &'static str> {
        let new_x = entity.pos().x as isize + dx as isize;
        let new_y = entity.pos().y as isize + dy as isize;

        if !self.is_in_bounds(new_x, new_y) {
            return Err("Target Point out of bounds.");
        }

        if !self.get_tile(new_x as usize, new_y as usize).tile_type.is_walkable() {
            return Err("Target Point is not walkable.");
        }

        entity.move_to(Point { x: new_x as usize, y: new_y as usize });
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
