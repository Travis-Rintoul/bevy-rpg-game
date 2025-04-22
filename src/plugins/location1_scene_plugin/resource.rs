use bevy::prelude::Resource;

use crate::plugins::grid_system_plugin::AxialCoord;

#[derive(Resource)]
pub struct Location1Scene {
    pub player_spawn_position: AxialCoord,
}

impl Location1Scene {
    pub fn new() -> Self {
        Location1Scene {
            player_spawn_position: AxialCoord { q: 0, r: 0 },
        }
    }
}
