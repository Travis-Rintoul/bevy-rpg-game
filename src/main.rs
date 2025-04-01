mod components;
mod models;
mod plugins;
mod scenes;
mod systems;

use bevy::prelude::*;
use plugins::{
    actor_plugin::ActorPlugin, data_cache_plugin::DataCachePlugin, game_master_plugin::GameMasterPlugin, player_plugin::PlayerPlugin, scene_manager_plugin::SceneManagerPlugin
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DataCachePlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(ActorPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GameMasterPlugin)
        .run();
}
