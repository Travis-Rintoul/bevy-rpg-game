use bevy::{math::Vec3, prelude::Resource};

use crate::plugins::grid_system_plugin::AxialCoord;

#[derive(Resource)]
pub struct Location1Scene {
    pub player_spawn_position: AxialCoord,
}

impl Location1Scene {
    const DEFAULT_SPAWN_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    pub fn new() -> Self {
        Location1Scene {
            player_spawn_position: AxialCoord { q: 0, r: 0 },
        }
    }
}
