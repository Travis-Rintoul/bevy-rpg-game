use bevy::{
    asset::Assets,
    color::Color,
    ecs::system::{Commands, ResMut},
    math::{Vec3, primitives::Plane3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::Meshable,
    render::mesh::{Mesh, Mesh3d},
};

use crate::plugins::grid_system_plugin::components::HexGrid;


pub fn spawn_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let width = 20;
    let height = 20;
    let position = Vec3::new(0.0, 0.0, 0.0);

    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(width as f32, height as f32))),
        //MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        HexGrid::new(width, height),
    ));
}
