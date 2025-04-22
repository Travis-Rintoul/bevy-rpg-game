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
        actor_plugin::{
            components::ActorHexCoord,
            events::{
                ActorAttackEvent, ActorDialogInitiatedEvent, ActorGridMoveEvent, ActorMoveEvent,
                PlayerAttackEvent, PlayerDialogInitiatedEvent, PlayerGridMoveEvent,
                PlayerMoveEvent,
            },
        },
        grid_system_plugin::components::HexGrid,
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
    player_query: Query<(Entity, &ActorHexCoord), With<Player>>,
    grid_query: Query<&HexGrid>,
    mut player_move_event_reader: EventReader<PlayerGridMoveEvent>,
    mut actor_move_event_writer: EventWriter<ActorGridMoveEvent>,
) {
    let Some((player_entity, start_coord)) = player_query.get_single().ok() else {
        return;
    };

    let Some(grid) = grid_query.get_single().ok() else {
        return;
    };

    for event in player_move_event_reader.read() {
        let Some(start_tile_entity) = grid.hex_map.get(&start_coord.coord) else {
            continue;
        };

        actor_move_event_writer.send(ActorGridMoveEvent {
            actor: player_entity,
            from_tile: *start_tile_entity,
            to_tile: event.to_tile,
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
            defender: event.defender,
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
            recipient: event.recipient,
        });
    }
}
