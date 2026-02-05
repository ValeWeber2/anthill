use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::world::tiles::Collision;
use crate::world::{
    coordinate_system::{Direction, Point},
    worldspace::World,
};

// Max iterations the A* algorithm is allowed to run with.
const MAX_ITERS: usize = 200;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    point: Point,
    g: usize,
    h: usize,
}

impl Node {
    fn f(&self) -> usize {
        self.g + self.h
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f().cmp(&self.f())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: Point, b: Point) -> usize {
    a.distance_squared_from(b)
}

impl World {
    /// Uses the A* algorithm to find the next direction to move in.
    pub fn next_step_toward(&self, start: Point, goal: Point) -> Option<Direction> {
        let a_star_path: Vec<Point> = self.a_star(start, goal)?;
        let next = a_star_path.get(1)?;

        let delta = *next - start;

        Direction::try_from(delta).ok()
    }

    /// A* Algorithm to find the shortest path between two Points on the Map.
    ///
    /// Taken from [idiomatic-rust-snippets.org](https://idiomatic-rust-snippets.org/algorithms/graph/a-star.html) and adapted to our world space.
    fn a_star(&self, start: Point, goal: Point) -> Option<Vec<Point>> {
        let mut iterations = 0;
        let mut open_list = BinaryHeap::new();
        let mut closed_list = HashMap::new();
        let mut came_from = HashMap::new();

        open_list.push(Node { point: start, g: 0, h: heuristic(start, goal) });

        while let Some(current) = open_list.pop() {
            iterations += 1;
            // If max iterations overstepped, cancel A*
            if iterations > MAX_ITERS {
                return None;
            }

            if current.point == goal {
                let mut path = vec![current.point];
                let mut current_position = current.point;
                while let Some(&prev_position) = came_from.get(&current_position) {
                    path.push(prev_position);
                    current_position = prev_position;
                }
                path.reverse();
                return Some(path);
            }

            closed_list.insert(current.point, current.g);

            for &neighbor in &[
                Point { x: current.point.x.saturating_sub(1), y: current.point.y },
                Point { x: current.point.x + 1, y: current.point.y },
                Point { x: current.point.x, y: current.point.y.saturating_sub(1) },
                Point { x: current.point.x, y: current.point.y + 1 },
            ] {
                if !self.get_tile(neighbor).tile_type.is_walkable() {
                    continue;
                }
                if closed_list.contains_key(&neighbor) {
                    continue;
                }

                let tentative_g = current.g + 1;

                if let Some(&existing_g) = closed_list.get(&neighbor) {
                    if tentative_g >= existing_g {
                        continue;
                    }
                }

                open_list.push(Node {
                    point: neighbor,
                    g: tentative_g,
                    h: heuristic(neighbor, goal),
                });

                came_from.insert(neighbor, current.point);
            }
        }
        None
    }
}
