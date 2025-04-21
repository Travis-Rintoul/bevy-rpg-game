use crate::{
    plugins::scene_manager_plugin::enums::{GameScene, GameSceneStatus},
    utils::ground::spawn_ground_system,
};
use bevy::{
    app::{App, Plugin},
    ecs::schedule::IntoSystemConfigs,
    state::state::{OnEnter, OnExit},
};

use super::systems::{cleanup, reader_method, setup };

pub struct SandboxScenePlugin;

impl Plugin for SandboxScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameScene::Sandbox),
            (spawn_ground_system, setup.after(spawn_ground_system)),
        )
        .add_systems(OnExit(GameScene::Sandbox), cleanup)
        .add_systems(OnEnter(GameSceneStatus::Ready), reader_method);
    }
}
