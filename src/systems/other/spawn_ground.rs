use bevy::{
    asset::Assets,
    color::Color,
    ecs::system::{Commands, ResMut},
    math::primitives::Plane3d,
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::Meshable,
    render::mesh::{Mesh, Mesh3d},
};

pub fn spawn_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
}
