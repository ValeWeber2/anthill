#![allow(dead_code)]
use std::cmp;

use rand::{Rng, SeedableRng, rngs::StdRng};

/// Binary Space Partitioning to procedurally generate rooms
/// Inspired by: https://www.youtube.com/watch?v=Pj4owFPH1Hw (Java)
use crate::{
    proc_gen::bsp_nodes::{MapNode, NodeId},
    world::{
        coordinate_system::Point,
        world_data::{RoomData, SpawnData, WorldData},
        worldspace::{Room, WORLD_HEIGHT, WORLD_WIDTH, World},
    },
};

/// Constant seed, from which the world is generated.
///
/// # TO DO
/// Have it be generated and injected by the caller of this module.
pub const RNG_SEED: u64 = 44;

/// Size of the grid with which distances are calculated. Here, the grid is just 1:1.
pub const GRID_SIZE: usize = 1;

/// Minimum dimensions a MapNode should have
///
/// # Note
/// Walls are counted as part of the MapNode! So a MapNode of dimensions 5x5 only has a walkable area of 3x3, surrounded by walls.
pub const MIN_NODE_DIM: usize = 13;

/// Minimum dimensions a MapNode should have after shrinking.
///
/// This is a separate value, so that a buffer is always possible for rooms to shrink by at least 1.
///
/// # Note
/// Walls are counted as part of the MapNode! So a MapNode of dimensions 5x5 only has a walkable area of 3x3, surrounded by walls.
pub const MIN_NODE_DIM_SHRUNK: usize = 5;

/// Minimum Distance the generated rooms should have to the edge of the map.
pub const PADDING: usize = 2;

/// Number of rooms to be generated.
///
/// # TO DO
/// Randomize within a reasonable range later.
pub const ROOM_NUMBER: usize = 10;

/// In the shrinking stage of the algorithm, the binary search partitions are shrunk, so there's spacing between rooms and so it looks more natual.
/// From this range, a random value is pulled for each room and used to shrink the room to that fraction.
pub const SHRINK_FACTOR_RANGE: std::ops::Range<f32> = 0.5..0.9;

/// In the division stage of the algorithm, a binary search partition is divided in two.
/// From this range, a random value is pulled which represents at which fraction the division is made.
pub const DIVIDER_RANGE: std::ops::Range<f32> = 0.4..0.6;

/// Data structure representing a hallway connecting two rooms.
#[derive(Clone, Debug)]
pub struct MapHall {
    /// Point of origin for the hallway (often in the middle of a room)
    point_a: Point,
    /// Target point for the hallway (often in the middle of a room)
    point_b: Point,
}

impl MapHall {
    fn new(point_a: Point, point_b: Point) -> Self {
        Self { point_a, point_b }
    }
}

/// Central Data Structure that contains the vector of nodes for the binary search partition tree structure.
#[derive(Clone)]
pub struct MapBSP {
    /// All nodes of the tree structure in a linear vector. Ids in the tree structure reference indices of this vector.
    pub nodes: Vec<MapNode>,

    /// NodeId of the root MapNode of the tree structure. (Usually 0)
    pub root: NodeId,

    /// Vector of all hallways on the map.
    pub halls: Vec<MapHall>,

    /// Used to track how many rooms a map has. The BSP alorithm recurses until a certain number of rooms is reached.
    pub num_rooms: usize,

    /// Contains the lots of `SpawnData` for the entire world. (Items and Npcs)
    pub spawns: Vec<SpawnData>,
}

impl MapBSP {
    /// Getter for a `MapNode` of a given `id`
    pub fn get_node(&self, id: usize) -> &MapNode {
        &self.nodes[id]
    }

    /// Getter for a mutable `MapNode` of a given `id`
    pub fn get_node_mut(&mut self, id: usize) -> &mut MapNode {
        &mut self.nodes[id]
    }

    /// Initiates the BSp algorithm by subdividing the root [MapNode].
    fn divide<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let mut rooms: usize = 1;

        while rooms < self.num_rooms {
            if self.divide_node(self.root, rng) {
                rooms += 1;
            }
        }
    }

    /// Binary Search Partitioning Algorithm (BSP) that divides a map node into two sub-nodes.
    ///
    /// Recurses through the tree until it finds a leaf. If no leaf is found, it continues on a random child node.
    /// Once it reaches a leaf, it subdivides it along a line randomly placed in the middle.
    /// Does not divide if the room dimensions are too small, as defined by [MIN_NODE_DIM].
    ///
    /// # Returns
    /// Returns a `bool` value that denotes whether a subdivision has been done or not. (Successful subdivisions are counted by the caller [MapBSP::divide()])
    fn divide_node<R: Rng + ?Sized>(&mut self, node_id: NodeId, rng: &mut R) -> bool {
        let (point_a, point_b) = {
            let node = &self.get_node(node_id);
            (node.point_a, node.point_b)
        };

        let room_dimensions = point_b - point_a;
        let room_width = room_dimensions.x as usize;
        let room_height = room_dimensions.y as usize;

        // Do not divide if rooms are smaller than the minimum size.
        if room_width < MIN_NODE_DIM || room_height < MIN_NODE_DIM {
            return false;
        }

        // If a child node, divides the child node along whatever axis is longer at a randomly chosen midpoint.
        if self.get_node(node_id).is_leaf() {
            if room_width > room_height {
                let new_midpoint = (point_a.x as f32
                    + rng.random_range(DIVIDER_RANGE) * room_width as f32)
                    as usize;

                let left_id = self.nodes.len();
                self.nodes.push(MapNode::new(point_a, Point::new(new_midpoint, point_b.y)));

                let right_id = self.nodes.len();
                self.nodes.push(MapNode::new(Point::new(new_midpoint, point_a.y), point_b));

                self.get_node_mut(node_id).left = Some(left_id);
                self.get_node_mut(node_id).right = Some(right_id);
            } else {
                let new_midpoint = (point_a.y as f32
                    + rng.random_range(DIVIDER_RANGE) * room_height as f32)
                    as usize;

                let left_id = self.nodes.len();
                self.nodes.push(MapNode::new(point_a, Point::new(point_b.x, new_midpoint)));

                let right_id = self.nodes.len();
                self.nodes.push(MapNode::new(Point::new(point_a.x, new_midpoint), point_b));

                self.get_node_mut(node_id).left = Some(left_id);
                self.get_node_mut(node_id).right = Some(right_id);
            }

            return true;
        }

        // If no child node is reached, continue along a randomly selected child node.
        if rng.random_bool(0.5) {
            if let Some(left_node_id) = self.get_node(node_id).left {
                return self.divide_node(left_node_id, rng);
            }
        } else if let Some(right_node_id) = self.get_node(node_id).right {
            return self.divide_node(right_node_id, rng);
        }

        false
    }

    /// Collects all leaves of the tree structure in a vector that has been passed as an argument.
    ///
    /// # Returns
    /// Nothing. Instead collects the leaves in the mutable vector that has been passed as an argument.
    /// This is done non-tail-recursively, because tail recursion caused problems with borrowing. However, this should contain fewer allocations than using tail recursion.
    pub fn get_leaves(&self, node_id: NodeId, leaves: &mut Vec<NodeId>) {
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

    /// Algorithm that looks for horizontal and vertical neighbors for every node.
    /// The found adjacency relations are noted in [MapNode::h_neighbors] and [MapNode::v_neighbors] for every [MapNode].
    fn find_neighbors(&mut self) {
        let mut leaves = Vec::new();
        self.get_leaves(self.root, &mut leaves);

        let mut h_neighbor_pairs = Vec::new();
        let mut v_neighbor_pairs = Vec::new();

        // Iterate through all node, to find their neighbours.
        for node_a_id in &leaves {
            // Iterate through all nodes, to find all neighbours for the current node.
            for node_b_id in &leaves {
                // Reject. A node is already its own neighbour.
                if *node_a_id == *node_b_id {
                    continue;
                }

                let node_a = self.get_node(*node_a_id);
                let node_b = self.get_node(*node_b_id);

                if node_a.point_b.x == node_b.point_a.x {
                    // Checking if nodes' coordinates touch (horizontally)
                    if node_a.point_a.x.max(node_b.point_a.x)
                        < node_a.point_b.y.min(node_b.point_b.y)
                    {
                        h_neighbor_pairs.push((node_a_id, node_b_id));
                    }
                }
                if node_a.point_b.y == node_b.point_a.y {
                    // Checking if node's coordinates touch (vertically)
                    if node_a.point_a.x.max(node_b.point_a.x)
                        < node_a.point_b.x.min(node_b.point_b.x)
                    {
                        v_neighbor_pairs.push((node_a_id, node_b_id));
                    }
                }
            }
        }

        // For all neighbour-relations found, push them to their respective nodes.
        for (node_a_id, node_b_id) in h_neighbor_pairs {
            self.get_node_mut(*node_a_id).h_neighbors.push(*node_b_id);
        }
        for (node_a_id, node_b_id) in v_neighbor_pairs {
            self.get_node_mut(*node_a_id).v_neighbors.push(*node_b_id);
        }
    }

    /// Takes all leaves of the BSP tree structure and shrinks their dimensions. This is done to make the map appear more natural and to create space between nodes.
    fn shrink_leaves<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let mut leaves = Vec::new();
        self.get_leaves(self.root, &mut leaves);

        for node_id in leaves {
            let node = self.get_node_mut(node_id);
            node.shrink(rng);
        }
    }

    /// Adds halls between neighboring rooms.
    fn add_halls<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let mut leaves = Vec::new();
        self.get_leaves(self.root, &mut leaves);

        for node_id in leaves {
            let mut h_halls = Vec::new();
            let mut v_halls = Vec::new();

            let node = self.get_node(node_id);

            // Iterate through all horizontal neighbors to create halls between them.
            for neighbor_id in node.h_neighbors.clone() {
                let neighbor = self.get_node(neighbor_id);

                if node.point_b.y.min(neighbor.point_b.y) - node.point_a.y.max(neighbor.point_a.y)
                    > GRID_SIZE * 3
                // If there is clear overlap in the y-space
                {
                    // Adding double grid size to the room overlap regions, so that hallways don't connect to corners.
                    let y = rng.random_range(
                        cmp::max(node.point_a.y, neighbor.point_a.y) + GRID_SIZE * 2
                            ..cmp::min(node.point_b.y, neighbor.point_b.y)
                                - GRID_SIZE * 2
                                - GRID_SIZE,
                    );

                    let new_hall = MapHall::new(
                        Point::new(node.point_b.x, y),
                        Point::new(neighbor.point_a.x, y + GRID_SIZE),
                    );
                    h_halls.push((node_id, new_hall));
                }
            }

            // Iterate through all vertical neighbors to create halls between them.
            for neighbor_id in node.v_neighbors.clone() {
                let neighbor = self.get_node(neighbor_id);

                if node.point_b.x.min(neighbor.point_b.x) - node.point_a.x.max(neighbor.point_a.x)
                    > GRID_SIZE * 3
                // If there is clear overlap in the x-space
                {
                    // Adding double grid size to the room overlap regions, so that hallways don't connect to corners.
                    let x = rng.random_range(
                        cmp::max(node.point_a.x, neighbor.point_a.x) + GRID_SIZE * 2
                            ..cmp::min(node.point_b.x, neighbor.point_b.x)
                                - GRID_SIZE * 2
                                - GRID_SIZE,
                    );

                    let new_hall = MapHall::new(
                        Point::new(x, node.point_b.y),
                        Point::new(x + GRID_SIZE, neighbor.point_a.y),
                    );
                    v_halls.push((node_id, new_hall));
                }
            }

            // Saving newly created halls to MapBSP::halls
            for (node_id, hall) in h_halls {
                let hall_id = self.halls.len();
                self.halls.push(hall);
                self.get_node_mut(node_id).h_halls.push(hall_id);
            }
            for (node_id, hall) in v_halls {
                let hall_id = self.halls.len();
                self.halls.push(hall);
                self.get_node_mut(node_id).v_halls.push(hall_id);
            }
        }
    }
}

impl From<MapBSP> for World {
    fn from(value: MapBSP) -> Self {
        let mut world = World::new();

        let mut leaves = Vec::new();
        value.get_leaves(value.root, &mut leaves);
        for node_id in leaves.clone().into_iter() {
            let node = value.get_node(node_id).clone();
            world.carve_room(&Room::from(node));
        }

        // for hallway in self.halls.clone().into_iter() {
        //     world.carve_corridor(hallway.point_a, hallway.point_b);
        // }

        world
    }
}

impl From<MapBSP> for WorldData {
    fn from(value: MapBSP) -> Self {
        let mut leaves = Vec::new();
        value.get_leaves(value.root, &mut leaves);

        let room_data = leaves
            .clone()
            .into_iter()
            .map(|leaf_id| RoomData::from(value.get_node(leaf_id).clone()))
            .collect();

        WorldData {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles: Vec::new(),
            rooms: room_data,
            spawns: value.spawns,
        }
    }
}

impl Default for MapBSP {
    fn default() -> Self {
        let mut nodes = Vec::new();
        let root = nodes.len();
        nodes.push(MapNode::default());

        Self { nodes, root, halls: Vec::new(), num_rooms: ROOM_NUMBER, spawns: Vec::new() }
    }
}

pub fn generate_map() -> (World, WorldData) {
    let mut rng = StdRng::seed_from_u64(RNG_SEED);

    let mut map = MapBSP::default();
    map.divide(&mut rng);
    map.shrink_leaves(&mut rng);
    map.find_neighbors();
    map.add_halls(&mut rng);
    map.populate_rooms(&mut rng);

    (World::from(map.clone()), WorldData::from(map))
}
