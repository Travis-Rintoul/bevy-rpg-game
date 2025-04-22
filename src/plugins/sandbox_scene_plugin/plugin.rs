use crate::{
    plugins::scene_manager_plugin::enums::StartupPhase, utils::ground::spawn_ground_system,
};
use bevy::{
    app::{App, Plugin, Startup},
    ecs::schedule::IntoSystemConfigs,
};

use super::systems::setup;

pub struct SandboxScenePlugin;

impl Plugin for SandboxScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground_system.in_set(StartupPhase::SceneLoad))
            .add_systems(Startup, setup.in_set(StartupPhase::SceneSetup));
        //.add_systems(OnExit(GameScene::Sandbox), cleanup)
        //.add_systems(OnEnter(GameSceneStatus::Ready), reader_method);
    }
}
