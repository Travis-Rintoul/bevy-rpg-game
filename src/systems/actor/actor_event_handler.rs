use bevy::{
    ecs::{entity::Entity, event::EventReader, query::With, system::Query},
    transform::components::Transform,
};

use crate::models::events::actor_events::{ActorAttackEvent, ActorMoveEvent};

pub fn move_event_listener(
    mut events: EventReader<ActorMoveEvent>,
    mut query: Query<&mut Transform>,
) -> () {
    for event in events.read() {
        if let Ok(mut transform) = query.get_mut(event.actor) {
            transform.translation = event.position;
        }
    }
}

pub fn attack_event_listener(mut events: EventReader<ActorAttackEvent>) -> () {
    for event in events.read() {
        println!("{:?}", event);
    }
}
