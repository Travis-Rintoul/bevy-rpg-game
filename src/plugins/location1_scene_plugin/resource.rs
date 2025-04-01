use bevy::{math::Vec3, prelude::Resource};

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
