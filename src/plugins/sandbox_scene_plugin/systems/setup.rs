use bevy::{
    app::Plugin,
    asset::Assets,
    ecs::system::{Commands, Query, Res, ResMut},
    pbr::StandardMaterial,
    render::mesh::Mesh,
    state::state::NextState,
    transform::components::Transform,
};

use crate::{
    plugins::{
        grid_system_plugin::components::{HexGrid, HexTile},
        location1_scene_plugin::Location1Scene,
        scene_manager_plugin::enums::GameSceneStatus,
    },
    utils::{camera::setup_lights_and_cameras, player::spawn_player},
};

pub fn setup(
    scene: Res<Location1Scene>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hex_grid_query: Query<(&HexGrid)>,
    mut hex_tile_query: Query<(&mut HexTile, &mut Transform)>,
    mut scene_status: ResMut<NextState<GameSceneStatus>>,
) {
    println!("Sandbox setup...");

    setup_lights_and_cameras(&mut commands);

    scene_status.set(GameSceneStatus::Ready);

    for (hex_grid) in &hex_grid_query {
        let Some(entity) = hex_grid.hex_map.get(&scene.player_spawn_position) else {
            return;
        };

        let Ok((mut tile, transform)) = hex_tile_query.get_mut(*entity) else {
            return;
        };

        spawn_player(
            transform.translation,
            tile.coord,
            &mut commands,
            &mut meshes,
            &mut materials,
        );

        tile.occupied = true;
    }
}

pub fn reader_method() {}
