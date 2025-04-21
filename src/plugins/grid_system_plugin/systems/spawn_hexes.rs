use bevy::{
    ecs::{
        entity::Entity, event::EventWriter, observer::Trigger, system::{Commands, Query, Res, ResMut, Single}
    },
    input::{keyboard::KeyCode, ButtonInput},
    log::tracing_subscriber::util::SubscriberInitExt,
    math::{Quat, Vec2},
    pbr::MeshMaterial3d,
    picking::events::{Click, Out, Over, Pointer},
    render::mesh::Mesh3d,
    transform::components::Transform,
    utils::default,
};

use crate::{
    plugins::{
        actor_plugin::events::PlayerGridMoveEvent,
        grid_system_plugin::{
            AxialCoord, FirstAxialCoord, HexDirection, LastAxialCoord, TEMP_HEX_GRID_HEIGHT,
            TEMP_HEX_GRID_WIDTH,
            components::{HexGrid, HexTile},
            models::{GridMapPoint, HexTileAssets},
        },
    },
    utils::material::update_material_on,
};

use super::grid::{calculate_next_point, hex_grid_neighbors};

pub fn register_hexes() -> Vec<AxialCoord> {
    let mut hexes: Vec<AxialCoord> = default();
    for q in 0..=TEMP_HEX_GRID_WIDTH {
        for r in 0..=TEMP_HEX_GRID_HEIGHT {
            hexes.push(AxialCoord { q, r })
        }
    }
    hexes
}

pub fn spawn_hexes(
    mut query: Query<&mut HexGrid>,
    mut commands: Commands,
    hex_tile_assets: Res<HexTileAssets>,
) {
    for (mut grid_map) in query.iter_mut() {
        assert!(grid_map.hexes.len() > 0, "Hexes must be pre initialized");
        let grid_point = grid_map.point(GridMapPoint::TopLeft);
        let hexes = &mut grid_map.hexes;
        let mut hex_map = &mut grid_map.hex_map;

        let mut next_point = Vec2::new(grid_point.x, grid_point.y);
        let mut prev_coord = hexes[0].clone();
        let mut prev_row_start_vec2 = Vec2::new(grid_point.x, grid_point.y);

        for coord in hexes {
            if *coord == prev_coord {
                next_point = grid_point;
            } else {
                if coord.r == 0 {
                    next_point = calculate_next_point(
                        prev_row_start_vec2.x as f64,
                        prev_row_start_vec2.y as f64,
                        HexDirection::Bottom,
                    );
                    prev_row_start_vec2 = next_point;
                } else if coord.q == prev_coord.q || coord.q != prev_coord.q {
                    if coord.r % 2 != 0 {
                        next_point = calculate_next_point(
                            next_point.x as f64,
                            next_point.y as f64,
                            HexDirection::BottomRight,
                        );
                    } else {
                        next_point = calculate_next_point(
                            next_point.x as f64,
                            next_point.y as f64,
                            HexDirection::TopRight,
                        );
                    }
                }
            }

            let entity = commands
                .spawn((
                    HexTile::new(coord.q, coord.r),
                    Mesh3d(hex_tile_assets.mesh.clone()),
                    MeshMaterial3d(hex_tile_assets.material_standard_hex.clone()),
                    Transform::from_xyz(next_point.x, 0.0, next_point.y)
                        .with_rotation(Quat::from_rotation_x(std::f64::consts::PI as f32 / 2.0)),
                ))
                .observe(update_material_on::<Pointer<Over>>(
                    hex_tile_assets.material_focused_hex.clone(),
                ))
                .observe(update_material_on::<Pointer<Out>>(
                    hex_tile_assets.material_standard_hex.clone(),
                ))
                .observe(on_hex_click)
                .id();

            hex_map.insert(*coord, entity);

            prev_coord = coord.clone();
        }
    }
}

pub fn map_hexes(
    mut hex_grid: Single<&mut HexGrid>,
    query: Query<(&HexTile, Entity)>,
) {
    for (hex_tile, hex_entity) in &query {
        hex_grid.hex_map.insert(hex_tile.coord, hex_entity);
    }
}

fn on_hex_click(
    click: Trigger<Pointer<Click>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<&HexTile>,
    mut first: ResMut<FirstAxialCoord>,
    mut last: ResMut<LastAxialCoord>,
    mut player_move_event_writer: EventWriter<PlayerGridMoveEvent>,
) {
    let Ok(hex_tile) = query.get(click.entity()) else {
        panic!("Unable to find HexTile that was clicked!");
    };

    if !hex_tile.occupied {
        player_move_event_writer.send(PlayerGridMoveEvent {
            selected_grid: click.entity(),
        });
    }
}
