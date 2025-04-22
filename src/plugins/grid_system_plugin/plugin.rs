use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::plugins::scene_manager_plugin::enums::StartupPhase;

use super::systems::{hex_tile_click_listener, setup_assets, spawn_hexes};

pub struct GridSystemPlugin;

impl Plugin for GridSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_assets.in_set(StartupPhase::SceneLoad))
            .add_systems(Startup, spawn_hexes.in_set(StartupPhase::SpawnHexTiles))
            .add_systems(Update, hex_tile_click_listener);
    }
}
