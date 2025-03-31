use bevy::ecs::{entity::Entity, event::{self, EventReader}, system::Query};

use crate::{components::{actor::actor::Actor, health::{self, Health}}, models::events::actor_events::ActorHitEvent};

pub fn hit_event_listener(
    mut attack_event_reader: EventReader<ActorHitEvent>,
    mut query: Query<(Entity, &mut Actor, &mut Health)>,
) {
    for event in attack_event_reader.read() {
        println!("Hit Event: {:?}", event); // Debug print event data

        if let Ok(
            [
                (_, attacker, _),
                (_, defender, mut health),
            ],
        ) = query.get_many_mut([event.attacker, event.defender])
        {
            println!(
                "{} has attacked {} dealing {} damage",
                attacker.name, defender.name, event.damage_dealt
            );
            health.0 -= event.damage_dealt; // Corrected line
        } else {
            println!("Error: Failed to find entities in query.");
        }
    }
}