use bevy::ecs::{entity::Entity, event::EventReader, system::Query};

use crate::plugins::actor_plugin::components::{Actor, ActorHealth};

use super::events::ActorHitEvent;

pub fn hit_event_listener(
    mut attack_event_reader: EventReader<ActorHitEvent>,
    mut query: Query<(Entity, &mut Actor, &mut ActorHealth)>,
) {
    for event in attack_event_reader.read() {
        println!("Hit Event: {:?}", event); // Debug print event data

        if let Ok([(_, attacker, _), (_, defender, mut health)]) =
            query.get_many_mut([event.attacker, event.defender])
        {
            if event.is_crit {
                println!(
                    "{} has landed a critical hit on {} dealing {} damage",
                    attacker.name, defender.name, event.damage_dealt
                );
            } else {
                println!(
                    "{} has landed a hit on {} dealing {} damage",
                    attacker.name, defender.name, event.damage_dealt
                );
            }

            health.0 -= event.damage_dealt; // Corrected line
        } else {
            println!("Error: Failed to find entities in query.");
        }
    }
}
