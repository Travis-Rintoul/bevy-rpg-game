use bevy::{
    asset::Assets,
    ecs::system::{Commands, Res, ResMut},
    math::Vec3,
    pbr::StandardMaterial,
    render::mesh::Mesh,
};

use crate::{
    plugins::location1_scene_plugin::Location1Scene,
    utils::{
        camera::setup_lights_and_cameras, enemy::spawn_enemy, ground::spawn_ground,
        player::spawn_player,
    },
};

pub fn setup(
    scene: Res<Location1Scene>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Location1 setup...");

    setup_lights_and_cameras(&mut commands);
    spawn_ground(&mut commands, &mut meshes, &mut materials);

    spawn_player(
        scene.player_spawn_position,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    let pos = Vec3::new(5.0, 0.0, 5.0);

    spawn_enemy(
        pos,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
