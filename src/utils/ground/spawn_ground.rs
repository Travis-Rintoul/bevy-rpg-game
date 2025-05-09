use bevy::{
    asset::Assets,
    color::Color,
    ecs::system::{Commands, ResMut},
    math::primitives::Plane3d,
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::Meshable,
    render::mesh::{Mesh, Mesh3d},
};

use crate::plugins::grid_system_plugin::components::HexGrid;

pub fn spawn_ground_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_ground(&mut commands, &mut meshes, &mut materials);
}

pub fn spawn_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let width = 20;
    let height = 20;

    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(width as f32, height as f32))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        HexGrid::new(width, height),
    ));
}
