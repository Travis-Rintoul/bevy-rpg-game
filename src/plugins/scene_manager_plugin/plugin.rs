use bevy::{
    app::{App, Plugin, Startup},
    state::app::AppExtStates,
};

use crate::plugins::{
    location1_scene_plugin::Location1ScenePlugin, location2_scene_plugin::Location2ScenePlugin,
    sandbox_scene_plugin::SandboxScenePlugin,
};

use super::enums::{GameScene, GameSceneStatus, StartupPhase};
use bevy::prelude::IntoSystemSetConfigs;

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameScene::Sandbox)
            .insert_state(GameSceneStatus::Loading)
            .configure_sets(
                Startup,
                (
                    StartupPhase::SceneLoad,
                    StartupPhase::SpawnHexTiles,
                    StartupPhase::SceneSetup,
                    StartupPhase::PlayerSpawn,
                )
                    .chain(),
            )
            .add_plugins(SandboxScenePlugin)
            .add_plugins(Location1ScenePlugin)
            .add_plugins(Location2ScenePlugin);
    }
}
