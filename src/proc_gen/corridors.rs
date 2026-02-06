use std::collections::HashSet;

use rand::{Rng, seq::IndexedRandom};

use crate::{
    ai::pathfinding::a_star,
    proc_gen::{bsp::MapBSP, bsp_nodes::NodeId, mst::mst_kruskal},
    world::coordinate_system::Point,
};

#[derive(Clone)]
pub struct MapEdge {
    pub source: NodeId,
    pub destination: NodeId,
    pub weight: usize,
}

impl MapBSP {
    pub fn all_edges(&self, nodes: &[NodeId]) -> (Vec<MapEdge>, Vec<NodeId>) {
        let mut edges: Vec<MapEdge> = Vec::new();

        for (i, &node_a_id) in nodes.iter().enumerate() {
            let node_a_center = self.get_node(node_a_id).center();

            for (j, &node_b_id) in nodes.iter().enumerate().skip(i + 1) {
                // Skip to avoid repetition (+1 tos kip i=j)
                let node_b_center = self.get_node(node_b_id).center();

                let distance = node_a_center.distance_squared_from(node_b_center);

                edges.push(MapEdge { source: i, destination: j, weight: distance })
            }
        }

        (edges, nodes.to_vec())
    }

    pub fn find_node_connections<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
    ) -> (Vec<MapEdge>, Vec<NodeId>) {
        let (edges, node_ids) = self.all_edges(&self.rooms);

        let mut connections = match mst_kruskal(edges.clone(), self.rooms.len()) {
            // Kruskal should normally return something valid.
            Ok((_, connections)) => connections,

            // If not, just connect pairs, ugly, but better than nothing in an emergeynce.
            Err(_) => self.naive_node_connections(),
        };

        // Extra corridors for Jaquaysing
        for edge in edges {
            if rng.random_bool(0.05) {
                connections.push(edge);
            }
        }

        (connections, node_ids)
    }

    /// Naive way of creating [MapEdge]s between rooms.
    /// Iterates through all the rooms in windows of two and connects them.
    ///
    /// # Note
    /// This is extremely ugly and will only be used in cases where no Minimum Spanning Tree could be built.
    fn naive_node_connections(&self) -> Vec<MapEdge> {
        let mut connections: Vec<MapEdge> = Vec::new();
        for pair in self.rooms.clone().windows(2) {
            connections.push(MapEdge { source: pair[0], destination: pair[1], weight: 1 });
        }
        connections
    }

    pub fn a_star_corridors<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let (connections, node_ids) = self.find_node_connections(rng);

        for connection in connections {
            let node_a_id = node_ids[connection.source];
            let node_b_id = node_ids[connection.destination];

            let node_a = self.get_node(node_a_id);
            let node_b = self.get_node(node_b_id);

            let node_a_point =
                node_a.floor_points().choose(rng).copied().unwrap_or(node_a.center());
            let node_b_point =
                node_b.floor_points().choose(rng).copied().unwrap_or(node_b.center());

            let mut room_corners: HashSet<Point> = HashSet::new();
            let mut room_walls: HashSet<Point> = HashSet::new();
            let mut room_floor: HashSet<Point> = HashSet::new();
            for &node_id in &self.rooms {
                let node = self.get_node(node_id);
                room_corners.extend(node.corner_points());
                room_walls.extend(node.wall_points());
                room_floor.extend(node.floor_points());
            }

            let cost_function = |p| {
                if room_corners.contains(&p) {
                    return None;
                }
                if room_walls.contains(&p) {
                    return Some(20);
                }
                if room_floor.contains(&p) {
                    return Some(10);
                }
                Some(1)
            };

            let path = a_star(node_a_point, node_b_point, cost_function)
                .expect("A* wasn't able to find a path between the two points.");

            self.corridors.extend(path);
        }
    }
}
