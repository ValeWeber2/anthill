use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::world::tiles::Collision;
use crate::world::{
    coordinate_system::{Direction, Point},
    worldspace::World,
};

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
        let a_star_path: Vec<Point> = a_star(start, goal, |p| {
            if self.get_tile(p).tile_type.is_walkable() { Some(1) } else { None }
        })?;
        let next = a_star_path.get(1)?;

        let delta = *next - start;

        Direction::try_from(delta).ok()
    }
}

/// A* Algorithm to find the shortest path between two Points on the Map.
///
/// Taken from [idiomatic-rust-snippets.org](https://idiomatic-rust-snippets.org/algorithms/graph/a-star.html) and adapted to our world space.
pub fn a_star<F>(start: Point, goal: Point, mut cost: F) -> Option<Vec<Point>>
where
    F: FnMut(Point) -> Option<usize>,
{
    let mut open_list = BinaryHeap::new();

    // Best-known cost to reach given tile
    let mut g_score = HashMap::new();

    // Previously covered nodes.
    let mut closed_list = HashMap::new();

    // Reconstructible path
    let mut came_from = HashMap::new();

    g_score.insert(start, 0);

    open_list.push(Node { point: start, g: 0, h: heuristic(start, goal) });

    while let Some(current) = open_list.pop() {
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

        let neighbors = [
            Point { x: current.point.x.saturating_sub(1), y: current.point.y },
            Point { x: current.point.x + 1, y: current.point.y },
            Point { x: current.point.x, y: current.point.y.saturating_sub(1) },
            Point { x: current.point.x, y: current.point.y + 1 },
        ];

        for neighbor in neighbors {
            if closed_list.contains_key(&neighbor) {
                continue;
            }

            let tile_cost = match cost(neighbor) {
                Some(c) => c,
                None => continue,
            };

            let tentative_g = current.g + tile_cost;

            let previous_best_known = g_score.get(&neighbor).copied().unwrap_or(usize::MAX);

            let is_better_than_best = tentative_g < previous_best_known;

            if !is_better_than_best {
                continue;
            }

            // if let Some(&existing_g) = closed_list.get(&neighbor) {
            //     if tentative_g >= existing_g {
            //         continue;
            //     }
            // }

            open_list.push(Node { point: neighbor, g: tentative_g, h: heuristic(neighbor, goal) });

            g_score.insert(neighbor, tentative_g);

            came_from.insert(neighbor, current.point);
        }
    }
    None
}
