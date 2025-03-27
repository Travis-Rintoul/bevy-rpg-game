
use bevy::prelude::*;
use crate::models::enums::game_scene::GameScene;

use super::scenes::{
    location1_plugin::Location1ScenePlugin,
    location2_plugin::Location2ScenePlugin
};

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) 
    {
        app
            .insert_state(GameScene::Location1) // Set initial scene
            .add_plugins(Location1ScenePlugin)
            .add_plugins(Location2ScenePlugin);
    }
}