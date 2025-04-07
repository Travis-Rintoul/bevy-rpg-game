use bevy::{
    ecs::{
        event::EventReader,
        query::With,
        system::{Commands, Query, Res},
    },
    time::Time,
    transform::components::Transform,
};

use crate::{
    components::actor::{Actor, ActorMoveTarget},
    plugins::actor_plugin::events::ActorMoveEvent,
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

        if distance > 2.0 {
            transform.translation += direction * speed * time.delta_secs();
        }
    }
}
