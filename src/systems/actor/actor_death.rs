use bevy::ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        system::{Commands, ParamSet, Query},
    };

use crate::{
    components::{actor::actor::Actor, health::Health},
    models::events::actor_events::{
        ActorAttackEvent, ActorDeathEvent, ActorHitEvent, ActorMissEvent,
    },
    systems::chance::{calculate::calculate_damage_chance, roll::roll_hit_chance},
};

pub fn death_event_listener(
    mut hit_event_reader: EventReader<ActorDeathEvent>,
    mut commands: Commands,
    query: Query<(Entity, &Actor)>,
) {
    for event in hit_event_reader.read() {
        if let Ok(
            [
                (_, attacker),
                (_, defender),
            ],
        ) = query.get_many([event.attacker, event.defender])
        {
            for entity in query.iter() {
                commands.entity(event.defender).despawn();
                println!("{} attacked {} killing them", attacker.name, defender.name);
            }
        }
    }
}