use bevy::{
    app::{App, Plugin},
    state::state::{OnEnter, OnExit},
};

use crate::plugins::scene_manager_plugin::enums::GameScene;

use super::systems::{cleanup, setup};

pub struct SandboxScenePlugin;

impl Plugin for SandboxScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::Sandbox), setup)
            .add_systems(OnExit(GameScene::Sandbox), cleanup);
    }
}
