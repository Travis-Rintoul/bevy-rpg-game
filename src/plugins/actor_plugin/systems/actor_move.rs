use bevy::{
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{With, Without},
        system::{Commands, Query, Res, Single},
    },
    time::Time,
    transform::components::Transform,
};

use crate::plugins::{actor_plugin::components::{Actor, ActorHexCoord, ActorHexMovementCommand, ActorMoveTarget}, grid_system_plugin::{calculate_point_path, components::{HexGrid, HexTile}}};

use super::events::ActorGridMoveEvent;

// Listens for move event to be triggered
pub fn move_event_listener(
    mut commands: Commands, 
    mut events: EventReader<ActorGridMoveEvent>,
    query: Query<&HexTile>,

) -> () {
    for event in events.read() {

        if let Ok(
            [
                from_tile,
                to_tile,
            ],
        ) = query.get_many([event.from_tile, event.to_tile]) {

            let Some(path) = calculate_point_path(&from_tile.coord, &to_tile.coord, |_| false) else {
                panic!("unable to calc path");
            };

            commands
                .entity(event.actor)
                .insert(ActorHexMovementCommand {
                    path: path,
                    current_step: 0,
                });
        }
    }
}

// Performs the actual move
pub fn perform_move_event(
    mut grid: Single<&HexGrid>,
    mut tile_query: Query<(&HexTile, &Transform), Without<Actor>>,
    mut actor_query: Query<(&mut Transform, &mut ActorHexMovementCommand, &mut ActorHexCoord), With<Actor>>,
    time: Res<Time>,
) {
    let speed = 10.0; // Units per second

    let Some((mut actor_transform, mut actor_command, mut actor_coord)) = actor_query.get_single_mut().ok() else {
        return;
    };

    if actor_command.current_step >= actor_command.path.len() {
        return;
    }

    let Some(tile_entity) = grid.hex_map.get(&actor_command.path[actor_command.current_step]) else {
        return;
    };

    let Some((tile, tile_transform)) = tile_query.get(*tile_entity).ok() else {
        return;
    };

    let current_pos = actor_transform.translation;
    let target_pos = tile_transform.translation;
    actor_coord.coord = tile.coord;

    if current_pos == target_pos {
        actor_command.current_step += 1;
        return;
    }

    let direction = target_pos - current_pos;
    let movement = direction.normalize() * speed * time.delta_secs();

    let next_pos = if movement.length() >= direction.length() {
        target_pos
    } else {
        current_pos + movement
    };

    actor_transform.translation = next_pos;
}

pub fn movement_command_handler(
    mut hex_move_query: Query<(Entity, &mut ActorHexMovementCommand), With<Actor>>,
    mut free_move_query: Query<(Entity, &mut ActorHexMovementCommand), With<Actor>>,
) {
    for (entity, mut movement) in &mut hex_move_query {
        movement.current_step = 0;
    }
}
