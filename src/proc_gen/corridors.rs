use std::collections::HashSet;

use rand::{
    Rng, SeedableRng,
    distr::{Distribution, StandardUniform},
    rngs::StdRng,
    seq::IndexedRandom,
};

use crate::{
    ai::pathfinding::{a_star, pathfinding_naive},
    proc_gen::{bsp_nodes::NodeId, mst::mst_kruskal, proc_gen_world::ProcGenWorld},
    world::{
        coordinate_system::{Direction, Point},
        level_data::DoorTypeData,
    },
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
            Err(_) => self.find_room_connections_naive(),
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
    /// This is extremely ugly and will only be used as a fallback in cases where no Minimum Spanning Tree could be built.
    fn find_room_connections_naive(&self) -> Vec<MapEdge> {
        let mut connections: Vec<MapEdge> = Vec::new();
        for i in 0..self.rooms.len().saturating_sub(1) {
            connections.push(MapEdge { source: i, destination: i + 1, weight: 1 })
        }
        connections
    }

    pub fn a_star_corridors(&mut self, corridor_seed: u64) -> ProcGenCorridorMap {
        let mut rng = StdRng::seed_from_u64(corridor_seed);

        let connections = self.find_room_connections(&mut rng);

        let mut room_corners: HashSet<Point> = HashSet::new();
        let mut room_walls: HashSet<Point> = HashSet::new();
        let mut room_floor: HashSet<Point> = HashSet::new();
        for node in &self.rooms {
            room_corners.extend(node.corner_points());
            room_walls.extend(node.wall_points());
            room_floor.extend(node.floor_points());
        }

        let mut path_points: HashSet<Point> = HashSet::new();
        for connection in connections {
            let room_a = &self.rooms[connection.source];
            let room_b = &self.rooms[connection.destination];

            let room_a_point =
                room_a.floor_points().choose(&mut rng).copied().unwrap_or(room_a.center());
            let room_b_point =
                room_b.floor_points().choose(&mut rng).copied().unwrap_or(room_b.center());

            let cost_function = |p| {
                // Avoids wall openings being next to each other
                let neighbors_already_door = path_points.contains(&(p + Direction::Up))
                    || path_points.contains(&(p + Direction::Right))
                    || path_points.contains(&(p + Direction::Down))
                    || path_points.contains(&(p + Direction::Left));
                if room_walls.contains(&p) && neighbors_already_door {
                    return None;
                }

                // Corners are forbidden.
                if room_corners.contains(&p) {
                    return None;
                }
                // Prefer already existing corridors slightly
                if path_points.contains(&p) {
                    return Some(1);
                }
                // Only break through walls if you reach your target directly.
                if room_walls.contains(&p) {
                    return Some(8);
                }
                // Avoid floor of rooms, so a corridor doesn't needlessly carve through a room.
                if room_floor.contains(&p) {
                    return Some(10);
                }

                Some(4)
            };

            // Uses a naive pathfinding algorithm in the worst case scenario where A* can't find a path
            let path = match a_star(room_a_point, room_b_point, cost_function) {
                Some(a_star_path) => a_star_path,
                None => pathfinding_naive(room_a_point, room_b_point),
            };

            path_points.extend(path);
        }

        let corridor_points: Vec<Point> = path_points
            .iter()
            .filter(|point| !room_floor.contains(point) && !room_walls.contains(point))
            .copied()
            .collect();
        let mut door_points: Vec<Point> = path_points.intersection(&room_walls).copied().collect();
        door_points.sort();

        let door_data = door_points
            .iter()
            .map(|&point| {
                let door_type: DoorTypeData = rng.random();
                (point, door_type)
            })
            .collect();

        ProcGenCorridorMap { corridors: corridor_points, doors: door_data }
    }
}

#[derive(Default)]
pub struct ProcGenCorridorMap {
    pub corridors: Vec<Point>,
    pub doors: Vec<(Point, DoorTypeData)>,
}

impl Distribution<DoorTypeData> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DoorTypeData {
        match rng.random_range(0..100) {
            0..=19 => DoorTypeData::Closed,
            20..=24 => DoorTypeData::Open,
            _ => DoorTypeData::Archway,
        }
    }
}
