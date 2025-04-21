use bevy::ecs::{
    entity::Entity,
    event::EventReader,
    system::{Commands, Query},
};

use crate::plugins::actor_plugin::components::Actor;

use super::events::ActorDeathEvent;

pub fn death_event_listener(
    mut hit_event_reader: EventReader<ActorDeathEvent>,
    mut commands: Commands,
    query: Query<(Entity, &Actor)>,
) {
    for event in hit_event_reader.read() {
        if let Ok([(_, attacker), (_, defender)]) = query.get_many([event.attacker, event.defender])
        {
            for _ in query.iter() {
                commands.entity(event.defender).despawn();
                println!("{} attacked {} killing them", attacker.name, defender.name);
            }
        }
    }
}
