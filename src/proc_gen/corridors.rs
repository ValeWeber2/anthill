use std::collections::HashSet;

use rand::{Rng, seq::IndexedRandom};

use crate::{
    ai::pathfinding::a_star,
    proc_gen::{bsp_nodes::NodeId, mst::mst_kruskal, proc_gen_world::ProcGenWorld},
    world::coordinate_system::Point,
};

#[derive(Clone)]
pub struct MapEdge {
    pub source: NodeId,
    pub destination: NodeId,
    pub weight: usize,
}

impl ProcGenWorld {
    pub fn all_edges(&self) -> Vec<MapEdge> {
        let mut edges: Vec<MapEdge> = Vec::new();

        for (i, room_a) in self.rooms.iter().enumerate() {
            let room_a_center = room_a.center();

            for (j, room_b) in self.rooms.iter().enumerate().skip(i + 1) {
                // Skip to avoid repetition (+1 tos kip i=j)
                let room_b_center = room_b.center();

                let distance = room_a_center.distance_squared_from(room_b_center);

                edges.push(MapEdge { source: i, destination: j, weight: distance })
            }
        }

        edges
    }

    pub fn find_room_connections<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec<MapEdge> {
        let edges = self.all_edges();

        let mut connections = match mst_kruskal(edges.clone(), self.rooms.len()) {
            // Kruskal should normally return something valid.
            Ok((_, connections)) => connections,

            // If not, just connect pairs, ugly, but better than nothing in an emergeynce.
            Err(_) => self.naive_room_connections(),
        };

        // Extra corridors for Jaquaysing
        for edge in edges {
            if rng.random_bool(0.05) {
                connections.push(edge);
            }
        }

        connections
    }

    /// Naive way of creating [MapEdge]s between rooms.
    /// Iterates through all the rooms in windows of two and connects them.
    ///
    /// # Note
    /// This is extremely ugly and will only be used in cases where no Minimum Spanning Tree could be built.
    fn naive_room_connections(&self) -> Vec<MapEdge> {
        let mut connections: Vec<MapEdge> = Vec::new();
        for i in 0..self.rooms.len().saturating_sub(1) {
            connections.push(MapEdge { source: i, destination: i + 1, weight: 1 })
        }
        connections
    }

    pub fn a_star_corridors<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let connections = self.find_room_connections(rng);

        let mut room_corners: HashSet<Point> = HashSet::new();
        let mut room_walls: HashSet<Point> = HashSet::new();
        let mut room_floor: HashSet<Point> = HashSet::new();
        for node in &self.rooms {
            room_corners.extend(node.corner_points());
            room_walls.extend(node.wall_points());
            room_floor.extend(node.floor_points());
        }

        for connection in connections {
            let room_a = &self.rooms[connection.source];
            let room_b = &self.rooms[connection.destination];

            let room_a_point =
                room_a.floor_points().choose(rng).copied().unwrap_or(room_a.center());
            let room_b_point =
                room_b.floor_points().choose(rng).copied().unwrap_or(room_b.center());

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

            let path = a_star(room_a_point, room_b_point, cost_function)
                .expect("A* wasn't able to find a path between the two points.");

            self.corridors.extend(path);
        }
    }
}
