use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::core::game::GameState;
use crate::world::coordinate_system::{Direction, Point};
use crate::world::tiles::Collision;

// Max iterations the A* algorithm is allowed to run with.
const MAX_ITERS: usize = 200;

// Node representing one step in the A* algorithm.
#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    // Coordinates of the step.
    point: Point,

    // G-statistic of A*. Cost of the path from the starting node to the current node.
    g: usize,

    // H-statistic of A*. Heustistically calculated distance to the goal node.
    h: usize,
}

impl Node {
    // F-statistic of A*. Estimated cost of the cheapest path from the start to the goal through the current node.
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

/// Heuristic for A*. Uses the squared euclidean distance (a^2 + b^2) to gauge distance.
///
/// # Note
/// Special case, when the distance is = 1. If the distance is = 1, but diagonal, returns 2.
fn heuristic(a: Point, b: Point) -> usize {
    let distance = a.distance_squared_from(b);

    if distance > 1 {
        distance
    } else {
        let distance_exact = (distance as f64).sqrt();
        if distance_exact > 1.0 { 2 } else { 1 }
    }
}

impl GameState {
    /// Uses the A* algorithm to find the next direction to move in.
    ///
    /// # Returns
    /// * [None] if no path could be found
    /// * Some([Direction]) for the next required step
    pub fn next_step_toward(&self, start: Point, goal: Point) -> Option<Direction> {
        let a_star_path: Vec<Point> = a_star(start, goal, |point| {
            if !self.current_world().get_tile(point).tile_type.is_walkable() {
                return None;
            }
            if self.current_level().get_npc_at(point).is_some() {
                return None;
            }

            Some(1)
        })?;
        let next = a_star_path.get(1)?;

        let delta = *next - start;

        Direction::try_from(delta).ok()
    }
}

/// A* Algorithm to find the shortest path between two Points on the Map.
///
/// Taken from [idiomatic-rust-snippets.org](https://idiomatic-rust-snippets.org/algorithms/graph/a-star.html) and adapted to our world space.
/// Modified to use a cost closure instead of a closed list.
///
/// # Arguments
/// * start - Start point of A*.
/// * goal - Goal point of A*.
/// * cost - Cost Function that takes in a Point and returns its cost. The cost can either be [usize] (representing cost) or [None] (representing a forbidden Point).
pub fn a_star<F>(start: Point, goal: Point, mut cost: F) -> Option<Vec<Point>>
where
    F: FnMut(Point) -> Option<usize>,
{
    let mut iterations: usize = 0;

    let mut open_list = BinaryHeap::new();

    // Best-known cost to reach given tile
    let mut g_score = HashMap::new();

    // Reconstructible path
    let mut came_from = HashMap::new();

    g_score.insert(start, 0);

    open_list.push(Node { point: start, g: 0, h: heuristic(start, goal) });

    while let Some(current) = open_list.pop() {
        iterations += 1;
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

        let neighbors = [
            Point { x: current.point.x.saturating_sub(1), y: current.point.y },
            Point { x: current.point.x + 1, y: current.point.y },
            Point { x: current.point.x, y: current.point.y.saturating_sub(1) },
            Point { x: current.point.x, y: current.point.y + 1 },
        ];

        for neighbor in neighbors {
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

            open_list.push(Node { point: neighbor, g: tentative_g, h: heuristic(neighbor, goal) });

            g_score.insert(neighbor, tentative_g);

            came_from.insert(neighbor, current.point);
        }
    }
    None
}
