use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::plugins::scene_manager_plugin::enums::StartupPhase;

use super::{
    LastAxialCoord,
    models::FirstAxialCoord,
    systems::{map_hexes, setup_assets, spawn_hexes, test_emitter},
};

pub struct GridSystemPlugin;

impl Plugin for GridSystemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FirstAxialCoord(None))
            .insert_resource(LastAxialCoord(None))
            .add_systems(Startup, setup_assets.in_set(StartupPhase::SceneLoad))
            .add_systems(Startup, spawn_hexes.in_set(StartupPhase::SpawnHexTiles))            
            .add_systems(Update, test_emitter.in_set(StartupPhase::SceneSetup));
    }
}
