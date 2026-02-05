#![allow(dead_code)]
use rand::{Rng, SeedableRng, rngs::StdRng, seq::IndexedRandom};

/// Binary Space Partitioning to procedurally generate rooms
/// Inspired by: https://www.youtube.com/watch?v=Pj4owFPH1Hw (Java)
use crate::{
    proc_gen::bsp_nodes::{MapNode, NodeId},
    world::{
        coordinate_system::Point,
        world_data::{RoomData, SpawnData, TileData, TileTypeData, WorldData},
        worldspace::{Room, WORLD_HEIGHT, WORLD_WIDTH, World},
    },
};

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

/// Central Data Structure that contains the vector of nodes for the binary search partition tree structure.
#[derive(Clone)]
pub struct MapBSP {
    /// All nodes of the tree structure in a linear vector. Ids in the tree structure reference indices of this vector.
    pub nodes: Vec<MapNode>,

    /// NodeId of the root MapNode of the tree structure. (Usually 0)
    pub root: NodeId,

    /// All final rooms of the map (these are the leaves of the tree).
    pub rooms: Vec<NodeId>,

    /// Vector of all the tiles that will become hallways on the map.
    pub corridors: Vec<Point>,

    /// Used to track how many rooms a map has. The BSP alorithm recurses until a certain number of rooms is reached.
    pub num_rooms: usize,

    /// Contains the entry point, where the player appears upon reaching the level.
    pub entry: Point,

    /// Contains the exit point, where stairs that lead to the next level will be placed.
    pub exit: Point,

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

    /// Collects all leaves of the tree structure into `MapBSP`. This is done once, since the leaves are what are needed to proceed witht he generation.
    pub fn leaves_to_rooms(&mut self, node_id: NodeId) {
        let (is_leaf, left, right) = {
            let node = self.get_node(node_id);
            (node.is_leaf(), node.left, node.right)
        };

        if is_leaf {
            self.rooms.push(node_id);
            return;
        }

        if let Some(left) = left {
            self.leaves_to_rooms(left);
        }
        if let Some(right) = right {
            self.leaves_to_rooms(right);
        }
    }

    /// Takes all leaves of the BSP tree structure and shrinks their dimensions. This is done to make the map appear more natural and to create space between nodes.
    fn shrink_leaves<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        for node_id in self.rooms.clone() {
            self.get_node_mut(node_id).shrink(rng);
        }
    }

    /// Adds entry points and exit points for the Map (which will be turned into stairs)
    fn add_entry_exit<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        // Define rooms that need to exist on every level.
        let mandatory_rooms: Vec<usize> = self.rooms.choose_multiple(rng, 2).cloned().collect();
        let entry_node_id = mandatory_rooms[0];
        let exit_node_id = mandatory_rooms[1];

        // Determine entry
        let entry_node = self.get_node_mut(entry_node_id);
        let entry_node_floor = entry_node.get_floor_points();
        let entry_point =
            entry_node_floor.choose(rng).expect("Rooms are by definition bigger than 0");
        self.entry = *entry_point;

        // Determine exit
        let exit_node = self.get_node_mut(exit_node_id);
        let exit_node_floor = exit_node.get_floor_points();
        let exit_point =
            exit_node_floor.choose(rng).expect("Rooms are by definition bigger than 0");
        self.exit = *exit_point;
    }
}

impl From<MapBSP> for World {
    fn from(value: MapBSP) -> Self {
        let mut world = World::new();

        for node_id in value.rooms.clone().into_iter() {
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
        let room_data: Vec<RoomData> = value
            .rooms
            .clone()
            .into_iter()
            .map(|leaf_id| RoomData::from(value.get_node(leaf_id).clone()))
            .collect();

        let tiles: Vec<TileData> = vec![
            // Entry
            TileData { x: value.entry.x, y: value.entry.y, tile_type: TileTypeData::StairsUp },
            // Exit
            TileData { x: value.exit.x, y: value.exit.y, tile_type: TileTypeData::StairsDown },
        ];

        WorldData {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles,
            rooms: room_data,
            corridors: value.corridors,
            entry: value.entry,
            spawns: value.spawns,
        }
    }
}

impl Default for MapBSP {
    fn default() -> Self {
        let mut nodes = Vec::new();
        let root = nodes.len();
        nodes.push(MapNode::default());

        Self {
            nodes,
            root,
            rooms: Vec::new(),
            corridors: Vec::new(),
            num_rooms: ROOM_NUMBER,
            entry: Point::new(5, 5),
            exit: Point::new(6, 6),
            spawns: Vec::new(),
        }
    }
}

pub fn generate_map(map_seed: u64) -> (World, WorldData) {
    let mut rng = StdRng::seed_from_u64(map_seed);

    let mut map = MapBSP::default();
    map.divide(&mut rng);
    map.leaves_to_rooms(map.root);
    map.shrink_leaves(&mut rng);
    map.find_node_connections(&mut rng);
    map.a_star_corridors(&mut rng);
    map.populate_rooms(&mut rng);
    map.add_entry_exit(&mut rng);

    (World::from(map.clone()), WorldData::from(map))
}
