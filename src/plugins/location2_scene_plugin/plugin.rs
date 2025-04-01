use bevy::{
    app::{App, Plugin},
    state::state::{OnEnter, OnExit},
};

use crate::plugins::scene_manager_plugin::enums::GameScene;

use super::{
    resource::Location2Scene,
    systems::{cleanup, setup},
};

pub struct Location2ScenePlugin;

impl Plugin for Location2ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Location2Scene::new())
            .add_systems(OnEnter(GameScene::Location2), setup)
            .add_systems(OnExit(GameScene::Location2), cleanup);
    }
}
