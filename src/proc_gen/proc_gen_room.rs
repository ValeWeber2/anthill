use std::cmp;

use rand::Rng;

use crate::{
    proc_gen::{bsp::GRID_SIZE, bsp_nodes::MapBSPNode},
    world::{
        coordinate_system::{Point, PointVector},
        level_data::RoomData,
    },
};

/// In the shrinking stage of the algorithm, the binary search partitions are shrunk, so there's spacing between rooms and so it looks more natual.
/// From this range, a random value is pulled for each room and used to shrink the room to that fraction.
pub const SHRINK_FACTOR_RANGE: std::ops::Range<f32> = 0.5..0.9;

/// Minimum dimensions a MapNode should have after shrinking.
///
/// This is a separate value, so that a buffer is always possible for rooms to shrink by at least 1.
///
/// # Note
/// Walls are counted as part of the MapNode! So a MapNode of dimensions 5x5 only has a walkable area of 3x3, surrounded by walls.
pub const MIN_ROOM_DIM_SHRUNK: usize = 5;

/// Data structure that contains the dimensions of a procedurally generated room.
///
/// This is a slimmed down version of a [MapNode], stripped of its data which was used for the Binary Search Partition algorithm.
#[derive(Clone)]
pub struct ProcGenRoom {
    /// Point of origin (top left) of the room.
    pub point_a: Point,

    /// Point of the end (bottom right) of the room.
    pub point_b: Point,
}

impl From<MapBSPNode> for ProcGenRoom {
    fn from(value: MapBSPNode) -> Self {
        Self { point_a: value.point_a, point_b: value.point_b }
    }
}

// To convert a MapNode (BSP data structure) into RoomData (data structure used by the world save files)
impl From<ProcGenRoom> for RoomData {
    fn from(value: ProcGenRoom) -> Self {
        let dimensions = value.point_b - value.point_a;
        RoomData {
            x: value.point_a.x,
            y: value.point_a.y,
            width: dimensions.x as usize,
            height: dimensions.y as usize,
        }
    }
}

impl ProcGenRoom {
    /// Shrinks a room to a random size that is bounded by [MIN_ROOM_DIM_SHRUNK].
    ///
    /// Since the results of the Binary Space Partitioning Algorithm are [MapNode]s that are too large, this shrinking will make the map look more natural.
    pub fn shrink<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let width = self.point_b.x - self.point_a.x;
        let height = self.point_b.y - self.point_a.y;

        // Shrunk width
        let new_width = shrink_dimension(width, rng.random_range(SHRINK_FACTOR_RANGE));
        // Shrunk height. Max 1.5 times as large as the width. This avoids weird long rooms (due to terminal grid not being 1:1)
        let new_height = cmp::min(
            shrink_dimension(height, rng.random_range(SHRINK_FACTOR_RANGE)),
            (new_width as f32 * 1.5) as usize,
        );

        let new_origin_x =
            rng.random_range((self.point_a.x + 1)..=(self.point_b.x - new_width - 1));
        let new_origin_y =
            rng.random_range((self.point_a.y + 1)..=(self.point_b.y - new_height - 1));

        self.point_a.x = new_origin_x;
        self.point_b.x = new_origin_x + new_width;
        self.point_a.y = new_origin_y;
        self.point_b.y = new_origin_y + new_height;
    }

    /// Returns all [Point]s that make up the room's floor.
    pub fn floor_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        let x_range = (self.point_a.x + GRID_SIZE)..(self.point_b.x - GRID_SIZE);
        let y_range = (self.point_a.y + GRID_SIZE)..(self.point_b.y - GRID_SIZE);

        for x in x_range {
            for y in y_range.clone() {
                points.push(Point::new(x, y));
            }
        }

        points
    }

    /// Returns the center point of given room.
    ///
    /// # Note
    /// The result is rounded down to an integer, so it is not a perfect center.
    pub fn center(&self) -> Point {
        let dimensions: PointVector = self.point_b - self.point_a;
        self.point_a + dimensions.map(|n| n / 2)
    }

    /// Returns the corners of the room.
    ///
    /// # Note
    /// Since this game only has square rooms, the result is always four corners.
    pub fn corner_points(&self) -> Vec<Point> {
        vec![
            self.point_a,
            Point::new(self.point_b.x, self.point_a.y),
            self.point_b,
            Point::new(self.point_a.x, self.point_b.y),
        ]
    }

    /// Returns all points that make up the room's walls.
    pub fn wall_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        let top = self.point_a.y;
        let right = self.point_b.x;
        let bottom = self.point_b.y;
        let left = self.point_a.x;

        for x in left..=right {
            points.push(Point::new(x, top));
            points.push(Point::new(x, bottom));
        }

        for y in top..=bottom {
            points.push(Point::new(left, y));
            points.push(Point::new(right, y));
        }

        points
    }
}

/// Helper function to shrink dimensions of a room (height or width).
///
/// Gives the room 1 layer of padding first and then shrinks it relatively. Cannot shrink further than the [MIN_NODE_DIM_SHRUNK]
fn shrink_dimension(dimension: usize, factor: f32) -> usize {
    let padded = dimension.saturating_sub(2);
    let shrunken = (padded as f32 * factor) as usize;

    cmp::max(MIN_ROOM_DIM_SHRUNK, shrunken)
}
