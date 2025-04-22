use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    pbr::StandardMaterial,
    render::mesh::Mesh,
};

use crate::utils::{camera::setup_lights_and_cameras, enemy::spawn_enemy, ground::spawn_ground};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Location1 setup...");

    setup_lights_and_cameras(&mut commands);
    spawn_ground(&mut commands, &mut meshes, &mut materials);

    let pos = Vec3::new(5.0, 0.0, 5.0);

    spawn_enemy(pos, &mut commands, &mut meshes, &mut materials);
}
