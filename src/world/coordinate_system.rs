use std::fmt;
use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

/// Basic coordinate point in the coordinate system.
///
/// `usize` has been chosen for the coordinates, so that negative space cannot be represented.
///
/// Forms a whole number vector space together with [PointVector], which allows basic algebraic operations:
/// - Addition: ([Add]): `(Point, PointVector) -> Point`
/// - Subtraction: ([Sub]): `(Point, Point) -> PointVector`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash, Serialize, Deserialize)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Retreives the neighbouring coordinates in the given [Direction].
    pub fn get_adjacent(self, direction: Direction) -> Point {
        self + PointVector::from(direction)
    }

    /// Calculates the distance squared to another `Point` using the Pythagorean Theorem: `a^2 + b^2`
    ///
    /// # Note
    /// Returns the squared distance, because for this purpose the square root isn't needed. For the true Pythagorean distance, you need to take the square root.
    pub fn distance_squared_from(&self, other: Point) -> usize {
        let delta = *self - other;
        delta.length_squared() as usize
    }
}

impl Add<PointVector> for Point {
    type Output = Point;

    /// Add a [PointVector] to a `Point`.
    ///
    /// # Returns
    /// A `Point` at the end of the `PointVector`
    ///
    /// # Note
    /// If the result would be negative, the coordinates are clamped at `0`, because negative positions are not representable.
    fn add(self, vector: PointVector) -> Self::Output {
        let new_x = self.x as isize + vector.x;
        let new_y = self.y as isize + vector.y;

        Point { x: new_x.max(0) as usize, y: new_y.max(0) as usize }
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    /// Add a [Direction] to a `Point`.
    ///
    /// # Returns
    /// The neighbouring `Point` in the given `Direction`.
    fn add(self, rhs: Direction) -> Self::Output {
        self + PointVector::from(rhs)
    }
}

impl Sub for Point {
    type Output = PointVector;

    /// Subtract two `Points` to get their distance as a `PointVector`.
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
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Calculates the length of a PointVector using the Pythagorean Theorem: `a^2 + b^2`
    ///
    /// # Note
    /// Returns the squared distance, because for this purpose the square root isn't needed. For the true Pythagorean distance, you need to take the square root (and convert to a float).
    fn length_squared(&self) -> isize {
        self.x * self.x + self.y * self.y
    }
}

impl From<Direction> for PointVector {
    /// Creates a `PointVector` that indicates one step in the given [Direction]
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => PointVector { x: 0, y: -1 },
            Direction::Right => PointVector { x: 1, y: 0 },
            Direction::Down => PointVector { x: 0, y: 1 },
            Direction::Left => PointVector { x: -1, y: 0 },
        }
    }
}

/// Represents the 4 cardinal directions Up, Right, Down, Left.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<PointVector> for Direction {
    type Error = &'static str;

    /// Creates a `Direction` from a given `PointVector`.
    ///
    /// Only works for `PointVector`s with a length of 1 and only works in 4 cardinal directions.
    fn try_from(value: PointVector) -> Result<Self, Self::Error> {
        match value {
            PointVector { x: 0, y: -1 } => Ok(Direction::Up),
            PointVector { x: 1, y: 0 } => Ok(Direction::Right),
            PointVector { x: 0, y: 1 } => Ok(Direction::Down),
            PointVector { x: -1, y: 0 } => Ok(Direction::Left),
            _ => Err("Can't coerce PointVector into a cardinal direction"),
        }
    }
}
