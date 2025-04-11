use bevy::{
    asset::Handle,
    ecs::entity::Entity,
    pbr::StandardMaterial,
    prelude::Resource,
    render::mesh::Mesh,
};
use std::cmp::Ordering;

use super::systems::grid::calculate_point_distance;

pub enum HexDirection {
    Bottom = 0,
    BottomLeft = 1,
    TopLeft = 2,
    Top = 3,
    TopRight = 4,
    BottomRight = 5,
}
pub enum GridMapPoint {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AxialCoord {
    pub q: i32,
    pub r: i32,
}

impl AxialCoord {
    pub fn distance(&self, other: &AxialCoord) -> i32 {
        calculate_point_distance(self, other)
    }
}

#[derive(Resource)]
pub struct FirstAxialCoord(pub Option<Entity>);

#[derive(Resource)]
pub struct LastAxialCoord(pub Option<Entity>);

#[derive(Resource)]
pub struct HexTileAssets {
    pub mesh: Handle<Mesh>,
    pub material_standard_hex: Handle<StandardMaterial>,
    pub material_focused_hex: Handle<StandardMaterial>,
}

pub const HEX_DIRECTIONS: [(i32, i32); 6] = [
    (1, 0),  // east
    (1, -1), // northeast
    (0, -1), // northwest
    (-1, 0), // west
    (-1, 1), // southwest
    (0, 1),  // southeast
];

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Node {
    pub position: AxialCoord,
    pub cost: i32,
    pub priority: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // lower priority == higher priority in A*
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.cost.cmp(&other.cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
