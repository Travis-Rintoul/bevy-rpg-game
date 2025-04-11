use bevy::{
    ecs::system::{Query, Res},
    input::{ButtonInput, keyboard::KeyCode},
};

use crate::plugins::grid_system_plugin::{models::AxialCoord, LastAxialCoord};
use crate::plugins::grid_system_plugin::{FirstAxialCoord, components::HexTile};
use crate::plugins::grid_system_plugin::systems::grid::calculate_point_distance;
use crate::plugins::grid_system_plugin::systems::grid::calculate_point_path;

pub fn test_emitter(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut first_coord: Res<FirstAxialCoord>,
    mut last_coord: Res<LastAxialCoord>,
    query: Query<&HexTile>
) {
    if keyboard.just_pressed(KeyCode::Space) {

        let Some(first_entity) = first_coord.0 else {
            return;
        };

        let Some(last_entity) = last_coord.0 else {
            return;
        };

        if let Ok(
            [
                first_hex_tile,
                second_hex_tile,
            ],
        ) = query.get_many([first_entity, last_entity])
        {
            println!("First: q:{} | r:{}", first_hex_tile.coord.q, first_hex_tile.coord.r);
            println!("Last: q:{} | r:{}", second_hex_tile.coord.q, first_hex_tile.coord.r);
            println!("DISTANCE: {}", calculate_point_distance(&first_hex_tile.coord, &second_hex_tile.coord));

            match calculate_point_path(&first_hex_tile.coord, &second_hex_tile.coord, |coord: AxialCoord| false) {
                Some(path) => {
                    println!("Path found:");
                    for step in path {
                        println!("q: {}, r: {}", step.q, step.r);
                    }
                }
                None => println!("No path found."),
            }
            
        }
    }

    //     // let first = AxialCoord { q: 2, r: 2 };
    //     // let second  = AxialCoord { q: 0, r: 0 };
    //     // println!("DISTANCE: {}", calculate_point_distance(&first, &second));

    //     let Some(first_entity) = first_coord.0 else {
    //         return;
    //     };

    //     let Some(last_entity) = last_coord.0 else {
    //         return;
    //     };

    //     if let Ok(
    //         [
    //             first_hex_tile,
    //             second_hex_tile,
    //         ],
    //     ) = query.get_many([first_entity, last_entity])
    //     {
    //         println!("First: q:{} | r:{}", first_hex_tile.coord.q, first_hex_tile.coord.r);
    //         println!("Last: q:{} | r:{}", second_hex_tile.coord.q, first_hex_tile.coord.r);
    //         println!("DISTANCE: {}", calculate_point_distance(&first_hex_tile.coord, &second_hex_tile.coord));
    //     }
    // }

    // let Some(window) = window_query.get_single().ok() else {
    //     return;
    // };

    // let Some((camera, camera_transform)) = camera_query.get_single().ok() else {
    //     return;
    // };

    // let Some(cursor_position) = window.cursor_position() else {
    //     return;
    // };

    // let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
    //     return;
    // };

    // if let Some((hit_entity, hit)) = ray_cast.cast_ray(ray, &RayCastSettings::default()).first() {
    //     if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
    //         let is_hex: bool = hex_query.get(*hit_entity).is_ok();

    //         if is_hex {
    //             if let Ok(hex_tile) = hex_query.get(*hit_entity) {
    //                 if mouse.just_pressed(MouseButton::Left) {
    //                     first_coord.0 = Some(*hit_entity);
    //                 } else if mouse.just_pressed(MouseButton::Right) {
    //                     last_coord.0 = Some(*hit_entity);
    //                 }
    //             }
    //         }
    //     }
    // }
}