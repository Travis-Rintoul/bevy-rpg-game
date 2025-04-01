use bevy::{
    app::{App, Plugin},
    state::app::AppExtStates,
};

use crate::plugins::{
    location1_scene_plugin::Location1ScenePlugin, location2_scene_plugin::Location2ScenePlugin,
};

use super::enums::GameScene;

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameScene::Location1)
            .add_plugins(Location1ScenePlugin)
            .add_plugins(Location2ScenePlugin);
    }
}
