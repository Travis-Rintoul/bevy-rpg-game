use bevy::ecs::{entity::Entity, event::EventReader, system::Query};

use crate::{components::actor::actor::Actor, models::events::actor_events::ActorMissEvent};

pub fn miss_event_listener(
    mut miss_event_reader: EventReader<ActorMissEvent>,
    query: Query<(Entity, &mut Actor)>,
) {
    for event in miss_event_reader.read() {
        if let Ok(
            [
                (_, attacker),
                (_, defender),
            ],
        ) = query.get_many([event.attacker, event.defender]) {
            println!("{} has tried to attack {} but missed", attacker.name, defender.name);
        }
    }
}
