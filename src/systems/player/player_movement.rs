use bevy::{prelude::*, window::PrimaryWindow};

use crate::{components::{enemy::Enemy, main_camera::MainCamera, player::Player}, models::events::actor_events::{ActorAttackEvent, ActorMoveEvent}};

// pub fn player_move_event_listener(
//     mut event_reader: EventReader<ActorEvent>,
//     mut query: Query<&mut Transform, With<Player>>,
// ) {
//     for event in event_reader.read() {
//         for mut transform in &mut query {
//             transform.translation = event.position;
//         }
//     }
// }

// pub fn player_raycast_handler(
//     mut ray_cast: MeshRayCast,
//     mouse: Res<ButtonInput<MouseButton>>,
//     mut event_writer: EventWriter<PlayerMoveEvent>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
//     enemy_query: Query<Entity, With<Enemy>>,
// ) {
// }

// on left click create player move event at cursor
pub fn player_raycast_handler(
    mut ray_cast: MeshRayCast,
    mouse: Res<ButtonInput<MouseButton>>,
    mut actor_move_event_writer: EventWriter<ActorMoveEvent>,
    mut actor_attack_event_writer: EventWriter<ActorAttackEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Some(window) = window_query.get_single().ok() else {
            return;
        };
        let Some((camera, camera_transform)) = camera_query.get_single().ok() else {
            return;
        };
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        let Some(player_entity) = player_query.get_single().ok() else {
            return;
        };
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        if let Some((hit_entity, hit)) = ray_cast.cast_ray(ray, &RayCastSettings::default()).first()
        {
            let is_enemy = enemy_query.get(*hit_entity).is_ok();

            if is_enemy {
                actor_attack_event_writer.send(ActorAttackEvent {
                    attacker: player_entity,
                    defender: *hit_entity,
                });
            } else {
                actor_move_event_writer.send(ActorMoveEvent {
                    actor: player_entity,
                    position: Vec3::new(hit.point.x, 1.0, hit.point.z),
                });
            }
        }
    }
}
