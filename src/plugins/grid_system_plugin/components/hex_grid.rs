use bevy::{ecs::entity::Entity, math::Vec2, prelude::Component, utils::hashbrown::HashMap};

use crate::plugins::grid_system_plugin::{
    models::{AxialCoord, GridMapPoint}, systems::register_hexes
};

#[derive(Component)]
pub struct HexGrid {
    width: u32,
    height: u32,
    pub hexes: Vec<AxialCoord>,
    pub hex_map: HashMap<AxialCoord, Entity>,
}

impl HexGrid {
    pub fn new(width: u32, height: u32) -> Self {
        HexGrid {
            width: width,
            height: height,
            hexes: register_hexes(),
            hex_map: HashMap::default(),
        }
    }

    // pub fn get_hex_neighbor(hex: &AxialCoord, direction: HexDirection) -> Option<&AxialCoord> {
    //     None
    // }

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

// pub fn hex_grid_angle(direction: HexDirection) -> f64 {
//     ((direction as i32) as f64) * PI / 3.0
// }
