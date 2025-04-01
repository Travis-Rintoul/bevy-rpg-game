use bevy::{
    app::{App, Plugin},
    state::state::{OnEnter, OnExit},
};

use crate::plugins::scene_manager_plugin::enums::GameScene;

use super::{
    Location1Scene,
    systems::{cleanup, setup},
};

pub struct Location1ScenePlugin;

impl Plugin for Location1ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Location1Scene::new())
            .add_systems(OnEnter(GameScene::Location1), setup)
            .add_systems(OnExit(GameScene::Location1), cleanup);
    }
}
