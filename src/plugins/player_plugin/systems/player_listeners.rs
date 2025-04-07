use bevy::{
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        system::{Query, Res, ResMut},
    },
    time::Time,
};

use crate::{
    components::player::Player,
    plugins::{
        actor_plugin::events::{ActorAttackEvent, ActorDialogInitiatedEvent, ActorMoveEvent, PlayerAttackEvent, PlayerDialogInitiatedEvent, PlayerMoveEvent},
        player_plugin::structs::PlayerLastClick,
    },
};

#[allow(dead_code)] // TODO: reimplement
pub fn handle_debounce(time: &Res<Time>, last_click_time: &mut ResMut<PlayerLastClick>) -> bool {
    let current_time = time.elapsed_secs();
    if current_time - last_click_time.0 < 0.2 {
        return true;
    }
    last_click_time.0 = current_time;
    false
}

pub fn player_move_dispatcher(
    player_query: Query<Entity, With<Player>>,
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    mut actor_move_event_writer: EventWriter<ActorMoveEvent>,
) {
    let Some(player_entity) = player_query.get_single().ok() else {
        return;
    };

    for event in player_move_event_reader.read() {
        actor_move_event_writer.send(ActorMoveEvent {
            actor: player_entity,
            point: event.point
        });
    }
}

pub fn player_attack_dispatcher(
    player_query: Query<Entity, With<Player>>,
    mut player_attack_event_reader: EventReader<PlayerAttackEvent>,
    mut actor_attack_event_writer: EventWriter<ActorAttackEvent>,
) {
    let Some(player_entity) = player_query.get_single().ok() else {
        return;
    };

    for event in player_attack_event_reader.read() {
        actor_attack_event_writer.send(ActorAttackEvent {
            attacker: player_entity,
            defender: event.defender
        });
    }
}

pub fn player_dialog_dispatcher(
    player_query: Query<Entity, With<Player>>,
    mut player_dialog_event_reader: EventReader<PlayerDialogInitiatedEvent>,
    mut actor_dialog_event_writer: EventWriter<ActorDialogInitiatedEvent>,
) {
    let Some(player_entity) = player_query.get_single().ok() else {
        return;
    };

    for event in player_dialog_event_reader.read() {
        actor_dialog_event_writer.send(ActorDialogInitiatedEvent {
            initiator: player_entity,
            recipient: event.recipient
        });
    }
}
