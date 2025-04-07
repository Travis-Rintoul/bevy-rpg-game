use bevy::ecs::entity::Entity;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::query::With;
use bevy::ecs::system::Query;

use crate::components::enemy::Enemy;
use crate::plugins::actor_plugin::events::{PlayerAttackEvent, PlayerDialogInitiatedEvent, PlayerMoveEvent};
use crate::plugins::{
    grid_system_plugin::components::HexTile, user_input_plugin::models::RayCastHitEvent,
};

pub fn raycast_event_dispatcher(
    mut raycast_hit_event_writer: EventReader<RayCastHitEvent>,
    mut player_dialog_event_writer: EventWriter<PlayerDialogInitiatedEvent>,
    mut player_attack_event_writer: EventWriter<PlayerAttackEvent>,
    mut player_move_event_writer: EventWriter<PlayerMoveEvent>,
    enemy_query: Query<Entity, With<Enemy>>,
    hex_query: Query<Entity, With<HexTile>>,
) {
    for event in raycast_hit_event_writer.read() {

        let is_enemy = enemy_query.get(event.hit_entity).is_ok();
        let is_hex = hex_query.get(event.hit_entity).is_ok();

        if is_enemy {
            if event.right_click {
                player_dialog_event_writer.send(PlayerDialogInitiatedEvent {
                    recipient: event.hit_entity,
                });
            } else if event.left_click {
                player_attack_event_writer.send(PlayerAttackEvent {
                    defender: event.hit_entity,
                });
            }
        } else if is_hex && event.left_click {
            player_move_event_writer.send(PlayerMoveEvent {
                point: event.point
            });
        }
    }
}
