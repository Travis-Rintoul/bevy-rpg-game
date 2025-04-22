use bevy::prelude::*;

use crate::{
    components::player::Player,
    plugins::{
        actor_plugin::components::{Actor, ActorHealth, ActorHexCoord},
        grid_system_plugin::AxialCoord,
    },
};

pub fn spawn_player(
    spawn_point: Vec3,
    coord: AxialCoord,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let player = Player::new();
    let actor = Actor::new(player.name());
    let health = ActorHealth(actor.base_health);

    println!("{:?}", spawn_point);

    return commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
            Transform::from_xyz(spawn_point.x, spawn_point.y, spawn_point.z),
            ActorHexCoord::new(coord),
            actor,
            health,
            player,
        ))
        .id();
}
