use bevy::math::Vec2;

use std::collections::{BinaryHeap, HashMap};
use std::f64::consts::PI;

use crate::plugins::grid_system_plugin::models::{Node, HEX_DIRECTIONS};
use crate::plugins::grid_system_plugin::{HEX_GRID_RADIUS, HexDirection, models::AxialCoord};

pub fn calculate_next_point(start_x: f64, start_y: f64, direction: HexDirection) -> Vec2 {
    let distance = HEX_GRID_RADIUS * f64::sqrt(3.0);
    let angle = ((direction as i32) as f64) * PI / 3.0;

    Vec2::new(
        (start_x + distance * angle.cos()) as f32,
        (start_y + distance * angle.sin()) as f32,
    )
}

pub fn calculate_point_distance(a: &AxialCoord, b: &AxialCoord) -> i32 {
    let dq = (a.q - b.q).abs();
    let dr = (a.r - b.r).abs();
    let ds = (a.q + a.r - b.q - b.r).abs();
    (dq + dr + ds) / 2
}

pub fn calculate_point_path(
    start: &AxialCoord,
    goal: &AxialCoord,
    is_blocked: impl Fn(AxialCoord) -> bool,
) -> Option<Vec<AxialCoord>> {
    let mut frontier = BinaryHeap::new();
    frontier.push(Node {
        position: *start,
        cost: 0,
        priority: 0,
    });

    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    
    came_from.insert(*start, None);
    cost_so_far.insert(*start, 0);

    while let Some(Node { position, cost, .. }) = frontier.pop() {
        if position == *goal {
            let mut path = vec![position];
            let mut current = position;
            while let Some(Some(prev)) = came_from.get(&current) {
                path.push(*prev);
                current = *prev;
            }
            path.reverse();
            return Some(path);
        }

        for neighbor in hex_grid_neighbors(&position) {
            if is_blocked(neighbor) {
                continue;
            }

            let new_cost = cost + 1;
            let is_better = cost_so_far
                .get(&neighbor)
                .map(|&c| new_cost < c)
                .unwrap_or(true);

            if is_better {
                cost_so_far.insert(neighbor, new_cost);
                let priority = new_cost + neighbor.distance(goal);
                frontier.push(Node {
                    position: neighbor,
                    cost: new_cost,
                    priority,
                });
                came_from.insert(neighbor, Some(position));
            }
        }
    }

    None
}

pub fn hex_grid_neighbors(hex: &AxialCoord) -> Vec<AxialCoord> {
    HEX_DIRECTIONS
        .iter()
        .map(|(dq, dr)| AxialCoord {
            q: hex.q + dq,
            r: hex.r + dr,
        })
        .collect()
}
