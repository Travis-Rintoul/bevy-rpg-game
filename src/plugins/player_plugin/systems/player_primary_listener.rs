use bevy::{
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::With,
        system::{Query, Res, ResMut},
    },
    input::{ButtonInput, mouse::MouseButton},
    math::Vec3,
    picking::mesh_picking::ray_cast::{MeshRayCast, RayCastSettings},
    render::camera::Camera,
    time::Time,
    transform::components::GlobalTransform,
    window::{PrimaryWindow, Window},
};

use crate::{
    components::{enemy::Enemy, main_camera::MainCamera, player::Player},
    plugins::{
        actor_plugin::events::{ActorAttackEvent, ActorDialogInitiatedEvent, ActorMoveEvent},
        grid_system_plugin::Hex,
        player_plugin::structs::PlayerLastClick,
    },
};

pub fn handle_debounce(time: &Res<Time>, last_click_time: &mut ResMut<PlayerLastClick>) -> bool {
    let current_time = time.elapsed_secs();
    if current_time - last_click_time.0 < 0.2 {
        return true;
    }
    last_click_time.0 = current_time;
    false
}

// triggers player move, dialog, ect interactions
pub fn player_primary_listener(
    mut ray_cast: MeshRayCast,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut last_click_time: ResMut<PlayerLastClick>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
    hex_query: Query<Entity, With<Hex>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut actor_move_event_writer: EventWriter<ActorMoveEvent>,
    mut actor_attack_event_writer: EventWriter<ActorAttackEvent>,
    mut actor_dialog_event_writer: EventWriter<ActorDialogInitiatedEvent>,
) {
    let left_click = mouse.just_pressed(MouseButton::Left);
    let right_click = mouse.just_pressed(MouseButton::Right);

    if left_click || right_click {
        // Handle debounce
        if handle_debounce(&time, &mut last_click_time) {
            return;
        }

        // get necessary data
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
            let is_hex = hex_query.get(*hit_entity).is_ok();

            if is_enemy {
                if right_click {
                    actor_dialog_event_writer.send(ActorDialogInitiatedEvent {
                        initiator: player_entity,
                        recipient: *hit_entity,
                    });
                } else if left_click {
                    actor_attack_event_writer.send(ActorAttackEvent {
                        attacker: player_entity,
                        defender: *hit_entity,
                    });
                }
            } else if is_hex && left_click {
                actor_move_event_writer.send(ActorMoveEvent {
                    actor: player_entity,
                    position: Vec3::new(hit.point.x, 1.0, hit.point.z),
                });
            }
        }
    }
}
