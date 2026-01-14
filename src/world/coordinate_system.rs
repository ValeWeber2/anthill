use std::ops::{Add, Sub};

use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
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

    pub fn distance_squared_from(&self, other: Point) -> usize {
        let dx = self.x as isize - other.x as isize;
        let dy = self.y as isize - other.y as isize;
        (dx * dx + dy * dy) as usize
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

impl Sub for Point {
    type Output = PointDelta;

    fn sub(self, other: Point) -> PointDelta {
        let new_x = self.x as isize - other.x as isize;
        let new_y = self.y as isize - other.y as isize;

        PointDelta { x: new_x, y: new_y }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<PointDelta> for Direction {
    type Error = &'static str;

    fn try_from(value: PointDelta) -> Result<Self, Self::Error> {
        match value {
            PointDelta { x: 0, y: -1 } => Ok(Direction::Up),
            PointDelta { x: 1, y: 0 } => Ok(Direction::Right),
            PointDelta { x: 0, y: 1 } => Ok(Direction::Down),
            PointDelta { x: -1, y: 0 } => Ok(Direction::Left),
            _ => Err("Can't coerce PointDelta into a cardinal direction"),
        }
    }
}
