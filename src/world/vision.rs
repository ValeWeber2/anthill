#![warn(dead_code)]
/// Tranlated from a python algorithm from https://www.albertford.com/shadowcasting/.
use num_rational::*;
use strum::IntoEnumIterator;

use crate::{
    core::{entity_logic::Entity, game::GameState},
    world::{
        coordinate_system::{Direction, Point},
        worldspace::{WORLD_HEIGHT, WORLD_WIDTH, World},
    },
};

type Rational = Ratio<isize>;

#[derive(Clone, Copy, Debug)]
pub struct IPoint {
    x: isize,
    y: isize,
}

impl IPoint {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl From<Point> for IPoint {
    fn from(value: Point) -> Self {
        IPoint { x: value.x as isize, y: value.y as isize }
    }
}

impl From<IPoint> for Point {
    fn from(value: IPoint) -> Self {
        Point { x: value.x as usize, y: value.y as usize }
    }
}

/// The entrypoint to the program. Call this function to compute the field of view from an origin tile.
pub fn compute_fov(origin: Point, world: &mut World) {
    world.mark_visible(origin);

    // Make all tiles invisible
    for i in 0..WORLD_WIDTH {
        for j in 0..WORLD_HEIGHT {
            world.tiles[world.index(i, j)].visible = false;
        }
    }

    // Determine which tiles to make visible
    for direction in Direction::iter() {
        let quadrant = Quadrant::new(direction, origin.into());

        let first_row = Row::new(1, Rational::new(-1, 1), Rational::new(1, 1));
        scan(origin, first_row, quadrant, world);
    }
}

/// Scan a row and recursively scan all of its children. If you think of each quadrant as a tree of rows, this essentially is a depth-first tree traversal.
fn scan(origin: Point, row: Row, quadrant: Quadrant, world: &mut World) {
    let mut prev_tile: Option<IPoint> = None;
    let mut row = row;

    let row_tiles: Vec<_> = row.tiles().collect(); // Cloning was required since I change values.

    for tile in row_tiles {
        let tile_is_wall = world.is_blocking(quadrant.transform(tile).into());
        let tile_is_floor = !tile_is_wall;

        let prev_tile_is_wall =
            prev_tile.is_some_and(|prev| world.is_blocking(quadrant.transform(prev).into()));
        let prev_tile_is_floor =
            prev_tile.is_some_and(|prev| !world.is_blocking(quadrant.transform(prev).into()));

        // Vision Range = 5 tiles
        if (Point::from(quadrant.transform(tile)).distance_squared_from(origin) as f32).sqrt()
            >= 30.0
        {
            continue;
        }

        // Tile is in both start and end slope
        if tile_is_wall || is_symmetric(row, tile) {
            let pos = quadrant.transform(tile);
            world.mark_visible(pos.into());
        }

        // Covered by wall
        if prev_tile_is_wall && tile_is_floor {
            row.start_slope = slope(tile);
        }

        // Tile is not covered => Continue scanning from there
        if prev_tile_is_floor && tile_is_wall {
            let mut next_row = row.next();
            next_row.end_slope = slope(tile);
            scan(origin, next_row, quadrant, world);
        }
        prev_tile = Some(tile);
    }
    if prev_tile.is_some_and(|tile| !world.is_blocking(quadrant.transform(tile).into())) {
        scan(origin, row.next(), quadrant, world);
    }
}

trait FieldOfView {
    fn is_blocking(&self, p: Point) -> bool;
    fn mark_visible(&mut self, p: Point);
}

impl FieldOfView for World {
    fn is_blocking(&self, point: Point) -> bool {
        let tile = self.get_tile(point.x, point.y);
        tile.tile_type.is_opaque()
    }
    fn mark_visible(&mut self, point: Point) {
        self.get_tile_mut(point.x, point.y).visible = true;
    }
}

impl GameState {
    pub fn compute_fov(&mut self) {
        compute_fov(*self.player.character.pos(), &mut self.world);
    }
}

#[derive(Clone, Copy, Debug)]
struct Quadrant {
    direction: Direction,
    origin: IPoint,
}

impl Quadrant {
    pub fn new(direction: Direction, origin: IPoint) -> Self {
        Self { direction, origin }
    }
    /// Convert a Point representing a position relative to the current quadrant into a Point representing an absolute position in the grid.
    pub fn transform(&self, tile: IPoint) -> IPoint {
        let IPoint { x: row, y: col } = tile;
        match self.direction {
            Direction::Up => IPoint {
                x: self.origin.x.saturating_add(col),
                y: self.origin.y.saturating_sub(row),
            },
            Direction::Right => IPoint {
                x: self.origin.x.saturating_add(row),
                y: self.origin.y.saturating_add(col),
            },
            Direction::Down => IPoint {
                x: self.origin.x.saturating_add(col),
                y: self.origin.y.saturating_add(row),
            },
            Direction::Left => IPoint {
                x: self.origin.x.saturating_sub(row),
                y: self.origin.y.saturating_add(col),
            },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
/// A Row represents a segment of tiles bound between a start and end slope. depth represents the distance between the row and the quadrant’s origin.
struct Row {
    depth: isize,
    start_slope: Rational,
    end_slope: Rational,
}

impl Row {
    pub fn new(depth: isize, start_slope: Rational, end_slope: Rational) -> Self {
        Self { depth, start_slope, end_slope }
    }

    /// Returns an iterator over the tiles in the row. This function considers a tile to be in the row if the sector swept out by the row’s start and end slopes overlaps with a diamond inscribed in the tile. If the diamond is only tangent to the sector, it does not become part of the row.
    fn tiles(&self) -> impl Iterator<Item = IPoint> {
        let depth_times_start = Rational::new(self.depth, 1) * self.start_slope;
        let depth_times_end = Rational::new(self.depth, 1) * self.end_slope;

        let min_col = round_ties_up(depth_times_start);

        let max_col = round_ties_down(depth_times_end);

        let depth = self.depth;

        (min_col..=max_col).map(move |col| IPoint::new(depth, col))
    }

    fn next(&self) -> Row {
        Row::new(self.depth + 1, self.start_slope, self.end_slope)
    }
}

/// Calculates new start and end slopes. It’s used in two situations:
/// [1], if prev_tile (on the left) was a wall tile and tile (on the right) is a floor tile, then the slope represents a start slope and should be tangent to the right edge of the wall tile.
/// [2], if prev_tile was a floor tile and tile is a wall tile, then the slope represents an end slope and should be tangent to the left edge of the wall tile.
/// In both situations, the line is tangent to the left edge of the current tile, so we can use a single slope function for both start and end slopes.
fn slope(tile: IPoint) -> Rational {
    let IPoint { x: row_depth, y: col } = tile;
    Rational::new(2 * (col) - 1, 2 * row_depth)
}

/// Checks if a given floor tile can be seen symmetrically from the origin. It returns true if the central point of the tile is in the sector swept out by the row’s start and end slopes. Otherwise, it returns false.
fn is_symmetric(row: Row, tile: IPoint) -> bool {
    let IPoint { x: _row_depth, y: col } = tile;

    let depth_times_start = Rational::new(row.depth, 1) * row.start_slope;
    let depth_times_end = Rational::new(row.depth, 1) * row.end_slope;

    let col_as_rat = Rational::new(col, 1);

    col_as_rat >= depth_times_start && col_as_rat <= depth_times_end
}

/// Rounds n to the nearest integer. If n ends in .5, rounds up.
fn round_ties_up(n: Rational) -> isize {
    (n + Rational::new(1, 2)).floor().to_integer()
}

/// Rounds n to the nearest integer. If n ends in .5, rounds down.
fn round_ties_down(n: Rational) -> isize {
    (n - Rational::new(1, 2)).ceil().to_integer()
}
