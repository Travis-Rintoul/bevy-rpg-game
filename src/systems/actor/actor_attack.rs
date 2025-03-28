use bevy::ecs::event::EventReader;

use crate::models::events::actor_events::ActorAttackEvent;

pub fn attack_event_listener(mut events: EventReader<ActorAttackEvent>) -> () {
    for event in events.read() {
        println!("{:?}", event);
    }
}