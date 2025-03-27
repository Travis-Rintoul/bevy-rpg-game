use bevy::prelude::*;

use crate::systems::{
    camera::camera_spawn::setup_lights_and_cameras, other::spawn_ground::spawn_ground,
    player::player_spawn::spawn_player,
};

#[derive(Resource)]
pub struct Location2Scene {
    pub player_spawn_position: Vec3,
}

impl Location2Scene {
    const DEFAULT_SPAWN_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    pub fn new() -> Self {
        Location2Scene {
            player_spawn_position: Self::DEFAULT_SPAWN_POSITION, // use constant inline
        }
    }
}

pub fn setup(
    scene: Res<Location2Scene>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Location2 setup...");

    spawn_player(
        scene.player_spawn_position,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    setup_lights_and_cameras(&mut commands);
    spawn_ground(&mut commands, &mut meshes, &mut materials);
}

pub fn cleanup() {
    println!("Location2 cleanup...");
}
