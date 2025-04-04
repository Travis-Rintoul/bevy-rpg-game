use bevy::prelude::*;

use crate::components::{
    actor::{Actor, ActorHealth},
    player::Player,
};

pub fn spawn_player(
    spawn_point: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let player = Player::new();
    let actor = Actor::new(player.name());
    let health = ActorHealth(actor.base_health);

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
            Transform::from_xyz(spawn_point.x, spawn_point.y, spawn_point.z),
            actor,
            health,
            player,
        ))
        .id()
}
