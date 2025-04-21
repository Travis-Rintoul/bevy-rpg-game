use bevy::{
    ecs::{
        entity::Entity, event::EventReader, query::With, system::{Commands, Query, Res}
    },
    time::Time,
    transform::components::Transform,
};

use crate::plugins::actor_plugin::{
    components::{Actor, ActorFreeMovementCommand, ActorHexMovementCommand, ActorMoveTarget},
    events::ActorMoveEvent,
};

// Listens for move event to be triggered
pub fn move_event_listener(mut commands: Commands, mut events: EventReader<ActorMoveEvent>) -> () {
    for event in events.read() {
        commands
            .entity(event.actor)
            .insert(ActorMoveTarget(event.point));
    }
}

// Performs the actual move
pub fn perform_move_event(
    mut query: Query<(&mut Transform, &ActorMoveTarget), With<Actor>>,
    time: Res<Time>,
) {
    let speed = 100.0; // Movement speed

    for (mut transform, target) in &mut query {
        let direction = (target.0 - transform.translation).normalize_or_zero();
        let distance = transform.translation.distance(target.0);

        if distance > 0.1 {
            transform.translation += direction * speed * time.delta_secs();
        }
    }
}

pub fn movement_command_handler(
    mut hex_move_query: Query<(Entity, &mut ActorHexMovementCommand), With<Actor>>,
    mut free_move_query: Query<(Entity, &mut ActorHexMovementCommand), With<Actor>>,
) {
    for (entity, mut movement) in &mut hex_move_query {
        movement.current_step = 0;
    }
}
