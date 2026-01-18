use std::fmt;
use std::ops::{Add, Sub};

use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// Basic coordinate point in the coordinate system.
///
/// Forms a whole number vector space together with PointVector, which allows basic algebraic operations:
/// - Add: (Point, PointVector) -> Point
/// - Subtract: (Point, Point) -> PointVector
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn get_adjacent(self, direction: Direction) -> Point {
        self + PointVector::from(direction)
    }

    pub fn distance_squared_from(&self, other: Point) -> usize {
        let dx = self.x as isize - other.x as isize;
        let dy = self.y as isize - other.y as isize;
        (dx * dx + dy * dy) as usize
    }
}

impl Add<PointVector> for Point {
    type Output = Point;

    fn add(self, vector: PointVector) -> Self::Output {
        let new_x = self.x as isize + vector.x;
        let new_y = self.y as isize + vector.y;

        Point { x: new_x.max(0) as usize, y: new_y.max(0) as usize }
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        self + PointVector::from(rhs)
    }
}

impl Sub for Point {
    type Output = PointVector;

    fn sub(self, other: Point) -> PointVector {
        let new_x = self.x as isize - other.x as isize;
        let new_y = self.y as isize - other.y as isize;

        PointVector::new(new_x, new_y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x:{} y:{}", self.x, self.y)
    }
}

pub struct PointVector {
    pub x: isize,
    pub y: isize,
}

impl PointVector {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl From<Direction> for PointVector {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => PointVector { x: 0, y: -1 },
            Direction::Right => PointVector { x: 1, y: 0 },
            Direction::Down => PointVector { x: 0, y: 1 },
            Direction::Left => PointVector { x: -1, y: 0 },
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

impl TryFrom<PointVector> for Direction {
    type Error = &'static str;

    fn try_from(value: PointVector) -> Result<Self, Self::Error> {
        match value {
            PointVector { x: 0, y: -1 } => Ok(Direction::Up),
            PointVector { x: 1, y: 0 } => Ok(Direction::Right),
            PointVector { x: 0, y: 1 } => Ok(Direction::Down),
            PointVector { x: -1, y: 0 } => Ok(Direction::Left),
            _ => Err("Can't coerce PointDelta into a cardinal direction"),
        }
    }
}
