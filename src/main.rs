mod components;
mod models;
mod plugins;
mod scenes;
mod systems;

use bevy::prelude::*;
use plugins::{
    actor_plugin::ActorPlugin, enemy_plugin::EnemyPlugin, events_plugins::EventsPlugin,
    game_master_plugin::GameMasterPlugin, player_plugin::PlayerPlugin,
    scene_manager_plugin::SceneManagerPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EventsPlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(ActorPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(GameMasterPlugin)
        .run();
}
