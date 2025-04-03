use bevy::{
    asset::Assets,
    color::Color,
    ecs::system::{Commands, ResMut},
    math::{Vec3, primitives::Plane3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::Meshable,
    render::mesh::{Mesh, Mesh3d},
};

use crate::components::grid_mappable::GridMappable;

pub fn spawn_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let width = 20;
    let length = 20;
    let position = Vec3::new(0.0, 0.0, 0.0);

    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(width as f32, length as f32))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        GridMappable::new(width, length, position),
    ));
}
