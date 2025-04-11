use bevy::{
    ecs::{event::EventWriter, observer::Trigger, system::{Commands, Query, Res, ResMut}}, input::{keyboard::KeyCode, ButtonInput}, math::{Quat, Vec2}, pbr::MeshMaterial3d, picking::events::{Click, Out, Over, Pointer}, render::mesh::Mesh3d, transform::components::Transform
};

use crate::{
    plugins::{actor_plugin::events::PlayerGridMoveEvent, grid_system_plugin::{
        components::{HexGrid, HexTile}, models::{GridMapPoint, HexTileAssets}, FirstAxialCoord, HexDirection, LastAxialCoord, TEMP_HEX_GRID_HEIGHT, TEMP_HEX_GRID_WIDTH
    }},
    utils::material::update_material_on,
};

use super::grid::calculate_next_point;

pub fn spawn_hexes(
    query: Query<&HexGrid>,
    mut commands: Commands,
    hex_tile_assets: Res<HexTileAssets>,
) {
    for (grid_map) in query.iter() {
        let grid_point = grid_map.point(GridMapPoint::TopLeft);

        let mut prev_vec: Vec2 = grid_point;
        let mut start_point = Vec2::new(grid_point.x, grid_point.y);

        for i in 0..=TEMP_HEX_GRID_WIDTH {
            if i > 0 {
                start_point = calculate_next_point(
                    start_point.x as f64,
                    start_point.y as f64,
                    HexDirection::Bottom,
                );
            }

            for z in 0..=TEMP_HEX_GRID_HEIGHT {
                let position: Vec2;

                if z == 0 {
                    position = Vec2::new(start_point.x, start_point.y);
                } else if z % 2 == 0 {
                    position = calculate_next_point(
                        prev_vec.x as f64,
                        prev_vec.y as f64,
                        HexDirection::TopRight,
                    );
                } else {
                    position = calculate_next_point(
                        prev_vec.x as f64,
                        prev_vec.y as f64,
                        HexDirection::BottomRight,
                    );
                }

                commands
                    .spawn((
                        HexTile::new(z, i),
                        Mesh3d(hex_tile_assets.mesh.clone()),
                        MeshMaterial3d(hex_tile_assets.material_standard_hex.clone()),
                        Transform::from_xyz(position.x, 0.0, position.y).with_rotation(
                            Quat::from_rotation_x(std::f64::consts::PI as f32 / 2.0),
                        ),
                    ))
                    .observe(update_material_on::<Pointer<Over>>(
                        hex_tile_assets.material_focused_hex.clone(),
                    ))
                    .observe(update_material_on::<Pointer<Out>>(
                        hex_tile_assets.material_standard_hex.clone(),
                    ))
                    .observe(on_hex_click);

                prev_vec = position.clone();
            }
        }
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
    if keyboard.pressed(KeyCode::ControlLeft) {
        println!("First set");
        first.0 = Some(click.entity());
    } else if  keyboard.pressed(KeyCode::ShiftLeft) {
        println!("second set");
        last.0 = Some(click.entity());
    } else if keyboard.pressed(KeyCode::AltLeft) {
        if let Ok(hex) = query.get(click.entity()) {
            println!("{:?}", hex.coord);
        }
    } else {
        player_move_event_writer.send(PlayerGridMoveEvent {
            selected_grid: click.entity()
        });
    }

}