use bevy::prelude::*;

use crate::components::{actor::actor::Actor, player::Player};

pub fn spawn_player(
    spawn_point: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {

    let player = Player::new();

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
            Transform::from_xyz(spawn_point.x, spawn_point.y, spawn_point.z),
            Actor::new(player.name()),
            player,
        ))
        .id()
}
