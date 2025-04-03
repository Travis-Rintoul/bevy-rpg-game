mod components;
mod plugins;
mod utils;

use bevy::{DefaultPlugins, app::App};
use plugins::{
    actor_plugin::ActorPlugin, data_cache_plugin::DataCachePlugin,
    grid_system_plugin::plugin::GridSystemPlugin, player_plugin::PlayerPlugin,
    scene_manager_plugin::SceneManagerPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DataCachePlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(GridSystemPlugin)
        .add_plugins(ActorPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
