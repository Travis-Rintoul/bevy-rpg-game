use bevy::{math::{Vec2, Vec3}, prelude::Component};

use crate::plugins::grid_system_plugin::HexGrid;

pub enum GridMapPoint {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

#[derive(Component, Default)]
pub struct GridMappable {
    pub width: u32,
    pub height: u32,
    pub position: Vec3,
    pub grid: HexGrid,
}

impl GridMappable {
    pub fn new(width: u32, height: u32, position: Vec3) -> Self {
        GridMappable {
            width,
            height,
            position: position,
            grid: HexGrid::new(width, height),
        }
    }

    pub fn point(&self, point_type: GridMapPoint) -> Vec2 {

        let x = (self.width / 2) as f32;
        let z = (self.height / 2) as f32;

        match point_type {
            GridMapPoint::TopRight => Vec2::new(x, z),
            GridMapPoint::TopLeft => Vec2::new(-x, z),
            GridMapPoint::BottomRight => Vec2::new(-x, z),
            GridMapPoint::BottomLeft => Vec2::new(-x, -z),
        }
    }
}
