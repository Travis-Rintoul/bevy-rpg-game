use bevy::{math::Vec3, prelude::Resource};

#[allow(dead_code)]
#[derive(Resource)]
pub struct Location2Scene {
    pub player_spawn_position: Vec3,
}

impl Location2Scene {
    const DEFAULT_SPAWN_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    pub fn new() -> Self {
        Location2Scene {
            player_spawn_position: Self::DEFAULT_SPAWN_POSITION,
        }
    }
}
