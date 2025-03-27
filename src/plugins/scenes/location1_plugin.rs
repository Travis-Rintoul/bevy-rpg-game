use crate::{
    models::enums::game_scene::GameScene,
    scenes::location1_scene::{Location1Scene, cleanup, setup},
};
use bevy::prelude::*;

pub struct Location1ScenePlugin;

impl Plugin for Location1ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Location1Scene::new())
            .add_systems(OnEnter(GameScene::Location1), setup)
            .add_systems(OnExit(GameScene::Location1), cleanup);
    }
}
