use rand::{Rng, SeedableRng, rngs::StdRng};

/// Binary Space Partitioning to procedurally generate rooms
/// Inspired by: https://www.youtube.com/watch?v=Pj4owFPH1Hw (Java)
use crate::{
    proc_gen::bsp_nodes::{MapBSPNode, NodeId},
    world::coordinate_system::Point,
};

/// Size of the grid with which distances are calculated. Here, the grid is just 1:1.
pub const GRID_SIZE: usize = 1;

/// Minimum dimensions a MapNode should have
///
/// # Note
/// Walls are counted as part of the MapNode! So a MapNode of dimensions 5x5 only has a walkable area of 3x3, surrounded by walls.
pub const MIN_NODE_DIM: usize = 13;

/// Minimum Distance the generated rooms should have to the edge of the map.
pub const PADDING: usize = 2;

/// Number of rooms to be generated.
///
/// # TO DO
/// Randomize within a reasonable range later.
pub const ROOM_NUMBER: usize = 10;

/// In the division stage of the algorithm, a binary search partition is divided in two.
/// From this range, a random value is pulled which represents at which fraction the division is made.
pub const DIVIDER_RANGE: std::ops::Range<f32> = 0.4..0.6;

pub struct MapBSPTree {
    /// All nodes of the tree structure in a linear vector. Ids in the tree structure reference indices of this vector.
    pub nodes: Vec<MapBSPNode>,

    /// NodeId of the root MapNode of the tree structure. (Usually 0)
    pub root: NodeId,

    /// Used to track how many rooms a map has. The BSP alorithm recurses until a certain number of rooms is reached.
    pub num_rooms: usize,
}

impl Default for MapBSPTree {
    fn default() -> Self {
        let mut nodes = Vec::new();
        let root = nodes.len();
        nodes.push(MapBSPNode::default());

        Self { nodes, root, num_rooms: ROOM_NUMBER }
    }
}

impl MapBSPTree {
    pub fn generate_bsp(bsp_seed: u64) -> MapBSPTree {
        let mut rng = StdRng::seed_from_u64(bsp_seed);

        let mut bsp = MapBSPTree::default();
        bsp.divide(&mut rng);
        bsp
    }

    /// Getter for a `MapNode` of a given `id`
    pub fn get_node(&self, id: usize) -> &MapBSPNode {
        &self.nodes[id]
    }

    /// Getter for a mutable `MapNode` of a given `id`
    pub fn get_node_mut(&mut self, id: usize) -> &mut MapBSPNode {
        &mut self.nodes[id]
    }

    /// Initiates the BSp algorithm by subdividing the root [MapNode].
    pub fn divide<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let mut rooms: usize = 1;
        let mut iterations = 0;

        while rooms < self.num_rooms {
            if self.divide_node(self.root, rng) {
                rooms += 1;
            }

            iterations += 1;
            if iterations > 10_000 {
                break;
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
    pub fn divide_node<R: Rng + ?Sized>(&mut self, node_id: NodeId, rng: &mut R) -> bool {
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
                self.nodes.push(MapBSPNode::new(point_a, Point::new(new_midpoint, point_b.y)));

                let right_id = self.nodes.len();
                self.nodes.push(MapBSPNode::new(Point::new(new_midpoint, point_a.y), point_b));

                self.get_node_mut(node_id).left = Some(left_id);
                self.get_node_mut(node_id).right = Some(right_id);
            } else {
                let new_midpoint = (point_a.y as f32
                    + rng.random_range(DIVIDER_RANGE) * room_height as f32)
                    as usize;

                let left_id = self.nodes.len();
                self.nodes.push(MapBSPNode::new(point_a, Point::new(point_b.x, new_midpoint)));

                let right_id = self.nodes.len();
                self.nodes.push(MapBSPNode::new(Point::new(point_a.x, new_midpoint), point_b));

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

    /// Collects all ids of leaves of the tree structure into an accumulator.
    ///
    /// Used by [MapBSPTree::collect_leaves], which is later used to create rooms from the leaves.
    pub fn get_leaf_ids(&self, node_id: NodeId, accumulator: &mut Vec<NodeId>) {
        let node = self.get_node(node_id);

        if node.is_leaf() {
            accumulator.push(node_id);
        }

        if let Some(left) = node.left {
            self.get_leaf_ids(left, accumulator);
        }
        if let Some(right) = node.right {
            self.get_leaf_ids(right, accumulator);
        }
    }

    /// Collects all leaves of the tree structure into an accumulator. The leaves of the tree are what is going to become the rooms on the map.
    ///
    /// From these leaves rooms will be created further down the line.
    pub fn collect_leaves(&self) -> Vec<MapBSPNode> {
        let mut leaf_ids = Vec::with_capacity(self.num_rooms);
        self.get_leaf_ids(self.root, &mut leaf_ids);

        let leaves = leaf_ids.into_iter().map(|leaf_id| self.get_node(leaf_id).clone()).collect();

        leaves
    }
}
