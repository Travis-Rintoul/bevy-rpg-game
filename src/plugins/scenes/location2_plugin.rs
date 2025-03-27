use crate::{
    models::enums::game_scene::GameScene,
    scenes::location2_scene::{Location2Scene, cleanup, setup},
};
use bevy::prelude::*;

pub struct Location2ScenePlugin;

impl Plugin for Location2ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Location2Scene::new())
            .add_systems(OnEnter(GameScene::Location2), setup)
            .add_systems(OnExit(GameScene::Location2), cleanup);
    }
}
