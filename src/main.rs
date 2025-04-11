mod components;
mod plugins;
mod utils;

use bevy::{DefaultPlugins, app::App, picking::mesh_picking::MeshPickingPlugin};
use plugins::{
    actor_plugin::ActorPlugin, data_cache_plugin::DataCachePlugin,
    grid_system_plugin::plugin::GridSystemPlugin, player_plugin::PlayerPlugin,
    scene_manager_plugin::SceneManagerPlugin, user_input_plugin::UserInputPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UserInputPlugin)
        .add_plugins(MeshPickingPlugin)
        .add_plugins(DataCachePlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(GridSystemPlugin)
        .add_plugins(ActorPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
