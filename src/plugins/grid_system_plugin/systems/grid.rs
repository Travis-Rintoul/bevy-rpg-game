use bevy::math::Vec2;
use std::f64::consts::PI;

use crate::plugins::grid_system_plugin::{HexDirection, HEX_GRID_RADIUS};

pub fn calculate_next_point(start_x: f64, start_y: f64, direction: HexDirection) -> Vec2 {
    let distance = HEX_GRID_RADIUS * f64::sqrt(3.0);
    let angle = ((direction as i32) as f64) * PI / 3.0;

    Vec2::new(
        (start_x + distance * angle.cos()) as f32,
        (start_y + distance * angle.sin()) as f32,
    )
}