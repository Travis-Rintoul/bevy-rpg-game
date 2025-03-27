use crate::systems::{
    camera::camera_spawn::setup_lights_and_cameras, other::spawn_ground::spawn_ground,
    player::player_spawn::spawn_player,
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Location1Scene {
    pub player_spawn_position: Vec3,
}

impl Location1Scene {
    const DEFAULT_SPAWN_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    pub fn new() -> Self {
        Location1Scene {
            player_spawn_position: Self::DEFAULT_SPAWN_POSITION,
        }
    }
}

pub fn setup(
    scene: Res<Location1Scene>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //println!("Location1 setup...");

    setup_lights_and_cameras(&mut commands);
    spawn_ground(&mut commands, &mut meshes, &mut materials);

    spawn_player(
        scene.player_spawn_position,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    // spawn_enemy(&mut commands, &mut meshes, &mut materials);
}

pub fn cleanup() {
    println!("Location1 cleanup...");
}
