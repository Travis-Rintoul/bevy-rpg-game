use bevy::ecs::entity::Entity;
use bevy::ecs::event::{self, EventReader, EventWriter};
use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Single};
use bevy::transform::components::Transform;

use crate::components::enemy::Enemy;
use crate::components::player::Player;
use crate::plugins::actor_plugin::events::{
    ActorMoveEvent, PlayerAttackEvent, PlayerDialogInitiatedEvent, PlayerGridMoveEvent,
    PlayerMoveEvent,
};
use crate::plugins::{
    grid_system_plugin::components::HexTile, user_input_plugin::models::RayCastHitEvent,
};

pub fn raycast_event_dispatcher(
    mut raycast_hit_event_writer: EventReader<RayCastHitEvent>,
    mut player_dialog_event_writer: EventWriter<PlayerDialogInitiatedEvent>,
    mut player_attack_event_writer: EventWriter<PlayerAttackEvent>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for event in raycast_hit_event_writer.read() {
        let is_enemy = enemy_query.get(event.hit_entity).is_ok();

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
        }
    }
}

pub fn player_move_event_dispatcher(
    player_query: Query<Entity, With<Player>>,
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    mut actor_move_event_writer: EventWriter<ActorMoveEvent>,
) {
    for event in player_move_event_reader.read() {
        for (player) in &player_query {
            actor_move_event_writer.send(ActorMoveEvent {
                actor: player,
                point: event.point,
            });
        }
    }
}

pub fn player_grid_move_event_dispatcher(
    player_query: Query<Entity, With<Player>>,
    hex_query: Query<&Transform, With<HexTile>>,
    mut player_move_event_reader: EventReader<PlayerGridMoveEvent>,
    mut actor_move_event_writer: EventWriter<ActorMoveEvent>,
) {
    for event in player_move_event_reader.read() {
        if let Ok(transform) = hex_query.get(event.selected_grid) {
            for (player) in &player_query {
                actor_move_event_writer.send(ActorMoveEvent {
                    actor: player,
                    point: transform.translation,
                });
            }
        }
    }
}
