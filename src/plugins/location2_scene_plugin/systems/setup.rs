use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    pbr::StandardMaterial,
    render::mesh::Mesh,
};

use crate::utils::{camera::setup_lights_and_cameras, ground::spawn_ground};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Location1 setup...");

    setup_lights_and_cameras(&mut commands);
    spawn_ground(&mut commands, &mut meshes, &mut materials);
}
