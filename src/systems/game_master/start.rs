use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    pbr::StandardMaterial,
    render::mesh::Mesh,
};

use crate::systems::enemy::enemy_spawn::spawn_enemy;

pub fn start_game_master(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let position = Vec3::new(5.0, 1.0, 5.0);
    spawn_enemy(position, &mut commands, &mut meshes, &mut materials);
}
