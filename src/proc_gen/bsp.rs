#![allow(dead_code)]
use std::cmp;

use rand::{Rng, SeedableRng, rngs::StdRng};

/// Binary Space Partitioning to procedurally generate rooms
/// Inspired by: https://www.youtube.com/watch?v=Pj4owFPH1Hw (Java)
use crate::world::{
    coordinate_system::Point,
    world_data::{RoomData, WorldData},
    worldspace::{Room, WORLD_HEIGHT, WORLD_WIDTH, World},
};

const RNG_SEED: u64 = 44;
const GRID_SIZE: usize = 1;
const MIN_CELL_DIM: usize = 5; // was 2
const PADDING: usize = 2;
const ROOM_NUMBER: usize = 10;
const SHRINK_FACTOR_RANGE: std::ops::Range<f32> = 0.25..0.9;
const DIVIDER_RANGE: std::ops::Range<f32> = 0.3..0.6;

type NodeId = usize;

#[derive(Clone, Debug)]
struct MapCell {
    pub point_a: Point,
    pub point_b: Point,
    pub left: Option<NodeId>,
    pub right: Option<NodeId>,
    pub h_neighbors: Vec<usize>,
    pub v_neighbors: Vec<usize>,
    pub h_halls: Vec<usize>,
    pub v_halls: Vec<usize>,
}

impl MapCell {
    pub fn new(point_a: Point, point_b: Point) -> Self {
        Self {
            point_a,
            point_b,
            left: None,
            right: None,
            h_neighbors: Vec::new(),
            v_neighbors: Vec::new(),
            h_halls: Vec::new(),
            v_halls: Vec::new(),
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl Default for MapCell {
    fn default() -> Self {
        Self {
            point_a: Point::new(PADDING, PADDING), // Leaves 2 tile-wide padding as to not encroach upon the border.
            point_b: Point::new(WORLD_WIDTH - PADDING, WORLD_HEIGHT - PADDING),
            left: None,
            right: None,
            h_neighbors: Vec::new(),
            v_neighbors: Vec::new(),
            h_halls: Vec::new(),
            v_halls: Vec::new(),
        }
    }
}

impl From<MapCell> for Room {
    fn from(value: MapCell) -> Self {
        let dimensions = value.point_b - value.point_a;
        Room::new(value.point_a, dimensions.x as usize, dimensions.y as usize)
    }
}

impl From<MapCell> for RoomData {
    fn from(value: MapCell) -> Self {
        let dimensions = value.point_b - value.point_a;
        RoomData {
            x: value.point_a.x,
            y: value.point_a.y,
            width: dimensions.x as usize,
            height: dimensions.y as usize,
        }
    }
}

#[derive(Clone, Debug)]
struct MapHall {
    point_a: Point,
    point_b: Point,
}

impl MapHall {
    fn new(point_a: Point, point_b: Point) -> Self {
        Self { point_a, point_b }
    }
}

struct MapBSP {
    nodes: Vec<MapCell>,
    root: NodeId,
    halls: Vec<MapHall>,
    num_rooms: usize,
}

impl MapBSP {
    fn get_node(&self, id: usize) -> &MapCell {
        &self.nodes[id]
    }

    fn get_node_mut(&mut self, id: usize) -> &mut MapCell {
        &mut self.nodes[id]
    }

    fn divide<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let mut rooms: usize = 1;

        while rooms < self.num_rooms {
            if self.divide_node(self.root, rng) {
                rooms += 1;
            }
        }
    }

    fn divide_node<R: Rng + ?Sized>(&mut self, node_id: NodeId, rng: &mut R) -> bool {
        let (point_a, point_b) = {
            let node = &self.get_node(node_id);
            (node.point_a, node.point_b)
        };

        let room_dimensions = point_b - point_a;
        let room_width = room_dimensions.x as usize;
        let room_height = room_dimensions.y as usize;

        if room_width < MIN_CELL_DIM || room_height < MIN_CELL_DIM {
            return false;
        }

        if self.get_node(node_id).is_leaf() {
            if room_width > room_height {
                let new_midpoint =
                    (point_a.x as f32 + rng.random_range(DIVIDER_RANGE) * room_width as f32) as usize;

                let left_id = self.nodes.len();
                self.nodes.push(MapCell::new(point_a, Point::new(new_midpoint, point_b.y)));

                let right_id = self.nodes.len();
                self.nodes.push(MapCell::new(Point::new(new_midpoint, point_a.y), point_b));

                self.get_node_mut(node_id).left = Some(left_id);
                self.get_node_mut(node_id).right = Some(right_id);
            } else {
                let new_midpoint =
                    (point_a.y as f32 + rng.random_range(DIVIDER_RANGE) * room_height as f32) as usize;

                let left_id = self.nodes.len();
                self.nodes.push(MapCell::new(point_a, Point::new(point_b.x, new_midpoint)));

                let right_id = self.nodes.len();
                self.nodes.push(MapCell::new(Point::new(point_a.x, new_midpoint), point_b));

                self.get_node_mut(node_id).left = Some(left_id);
                self.get_node_mut(node_id).right = Some(right_id);
            }

            return true;
        }

        if rng.random_bool(0.5) {
            if let Some(left_node_id) = self.get_node(node_id).left {
                return self.divide_node(left_node_id, rng);
            }
        } else if let Some(right_node_id) = self.get_node(node_id).right {
            return self.divide_node(right_node_id, rng);
        }

        false
    }

    fn get_leaves(&self, node_id: NodeId, leaves: &mut Vec<NodeId>) {
        let node = self.get_node(node_id);

        if node.is_leaf() {
            leaves.push(node_id);
            return;
        }

        if let Some(left) = node.left {
            self.get_leaves(left, leaves);
        }
        if let Some(right) = node.right {
            self.get_leaves(right, leaves);
        }
    }

    fn find_neighbors(&mut self) {
        let mut leaves = Vec::new();
        self.get_leaves(self.root, &mut leaves);

        let mut h_neighbor_pairs = Vec::new();
        let mut v_neighbor_pairs = Vec::new();

        for cell_a_id in &leaves {
            for cell_b_id in &leaves {
                if *cell_a_id == *cell_b_id {
                    continue;
                } // A cell is already its own neighbour

                let cell_a = self.get_node(*cell_a_id);
                let cell_b = self.get_node(*cell_b_id);

                if cell_a.point_b.x == cell_b.point_a.x {
                    // Checking if cells' coordinates touch (horizontally)
                    if cell_a.point_a.x.max(cell_b.point_a.x)
                        < cell_a.point_b.y.min(cell_b.point_b.y)
                    {
                        // Check if cells overlap
                        // cell_a.h_neighbors.push(*cell_b_id);
                        h_neighbor_pairs.push((cell_a_id, cell_b_id));
                    }
                }
                if cell_a.point_b.y == cell_b.point_a.y {
                    // Checking if cell's coordinates touch (vertically)
                    if cell_a.point_a.x.max(cell_b.point_a.x)
                        < cell_a.point_b.x.min(cell_b.point_b.x)
                    {
                        // cell_a.v_neighbors.push(*cell_b_id);
                        v_neighbor_pairs.push((cell_a_id, cell_b_id));
                    }
                }
            }
        }

        for (cell_a_id, cell_b_id) in h_neighbor_pairs {
            self.get_node_mut(*cell_a_id).h_neighbors.push(*cell_b_id);
        }
        for (cell_a_id, cell_b_id) in v_neighbor_pairs {
            self.get_node_mut(*cell_a_id).v_neighbors.push(*cell_b_id);
        }
    }

    fn shrink_leaves<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let mut leaves = Vec::new();
        self.get_leaves(self.root, &mut leaves);

        for cell_id in leaves {
            let cell = self.get_node(cell_id);

            let width = cell.point_b.x - cell.point_a.x;
            let height = cell.point_b.y - cell.point_a.y;

            let new_width = MIN_CELL_DIM.max((width as f32 * rng.random_range(SHRINK_FACTOR_RANGE)) as usize);
            let new_height = MIN_CELL_DIM.max((height as f32 * rng.random_range(SHRINK_FACTOR_RANGE)) as usize);

            let cell = self.get_node_mut(cell_id);
            cell.point_a.x += (width - new_width) / 2;
            cell.point_b.x -= (width - new_width) / 2;
            cell.point_a.y += (height - new_height) / 2;
            cell.point_b.y -= (height - new_height) / 2;
        }
    }

    fn add_halls<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let mut leaves = Vec::new();
        self.get_leaves(self.root, &mut leaves);

        for cell_id in leaves {
            let mut h_halls = Vec::new();
            let mut v_halls = Vec::new();

            let cell = self.get_node(cell_id);
            for neighbor_id in cell.h_neighbors.clone() {
                let neighbor = self.get_node(neighbor_id);

                if cell.point_b.y.min(neighbor.point_b.y) - cell.point_a.y.max(neighbor.point_a.y)
                    > GRID_SIZE * 3 // If there is overlap in the y-space
                {
                    let y = rng.random_range(
                        cmp::max(cell.point_a.y, neighbor.point_a.y)
                            ..cmp::min(cell.point_b.y, neighbor.point_b.y) - GRID_SIZE,
                    );

                    // Reject corridors that are on corners of rooms
                    if y <= cell.point_a.y || y + GRID_SIZE >= cell.point_b.y {
                        continue;
                    }
                    if y <= neighbor.point_a.y || y + GRID_SIZE >= neighbor.point_b.y {
                        continue;
                    }

                    let new_hall = MapHall::new(
                        Point::new(cell.point_b.x, y),
                        Point::new(neighbor.point_a.x, y + GRID_SIZE),
                    );
                    h_halls.push((cell_id, new_hall));
                }
            }
            for neighbor_id in cell.v_neighbors.clone() {
                let neighbor = self.get_node(neighbor_id);

                if cell.point_b.x.min(neighbor.point_b.x) - cell.point_a.x.max(neighbor.point_a.x)
                    > GRID_SIZE * 3 // If there is overlap in the x-space
                {
                    let x = rng.random_range(
                        cmp::max(cell.point_a.x, neighbor.point_a.x)
                            ..cmp::min(cell.point_b.x, neighbor.point_b.x) - GRID_SIZE,
                    );

                    // Reject corridors that are on corners of rooms
                    if x <= cell.point_a.x || x + GRID_SIZE >= cell.point_b.x {
                        continue;
                    }
                    if x <= neighbor.point_a.x || x + GRID_SIZE >= neighbor.point_b.x {
                        continue;
                    }

                    let new_hall = MapHall::new(
                        Point::new(x, cell.point_b.y),
                        Point::new(x + GRID_SIZE, neighbor.point_a.y),
                    );
                    v_halls.push((cell_id, new_hall));
                }
            }

            // Creating new Halls
            for (cell_id, hall) in h_halls {
                let hall_id = self.halls.len();
                self.halls.push(hall);
                self.get_node_mut(cell_id).h_halls.push(hall_id);
            }
            for (cell_id, hall) in v_halls {
                let hall_id = self.halls.len();
                self.halls.push(hall);
                self.get_node_mut(cell_id).v_halls.push(hall_id);
            }
        }
    }

    fn make_worldspace(&self) -> World {
        let mut world = World::new();

        let mut leaves = Vec::new();
        self.get_leaves(self.root, &mut leaves);
        for cell_id in leaves.clone().into_iter() {
            let cell = self.get_node(cell_id).clone();
            world.carve_room(&Room::from(cell));
        }

        // for hallway in self.halls.clone().into_iter() {
        //     world.carve_corridor(hallway.point_a, hallway.point_b);
        // }

        world
    }

    fn make_world_data(&self) -> WorldData {
        let mut leaves = Vec::new();
        self.get_leaves(self.root, &mut leaves);

        let room_data = leaves
            .clone()
            .into_iter()
            .map(|leaf_id| RoomData::from(self.get_node(leaf_id).clone()))
            .collect();

        WorldData {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles: Vec::new(),
            rooms: room_data,
            spawns: Vec::new(),
        }
    }
}

impl Default for MapBSP {
    fn default() -> Self {
        let mut nodes = Vec::new();
        let root = nodes.len();
        nodes.push(MapCell::default());

        Self { nodes, root, halls: Vec::new(), num_rooms: ROOM_NUMBER }
    }
}

pub fn generate_map() -> (World, WorldData) {
    let mut rng = StdRng::seed_from_u64(RNG_SEED);

    let mut map = MapBSP::default();
    map.divide(&mut rng);
    map.find_neighbors();
    map.shrink_leaves(&mut rng);
    map.add_halls(&mut rng);

    (map.make_worldspace(), map.make_world_data())
}
