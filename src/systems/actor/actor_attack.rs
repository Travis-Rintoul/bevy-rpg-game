use bevy::{ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        system::Query,
    }, utils::tracing::dispatcher};

use crate::{
    components::{actor::actor::Actor, health::Health},
    models::events::actor_events::{
        ActorAttackEvent, ActorDeathEvent, ActorHitEvent, ActorMissEvent,
    }, systems::chance::{calculate::calculate_damage_chance, roll::{roll_crit_chance, roll_hit_chance}},
};

pub fn attack_event_listener(
    mut hit_event_writer: EventWriter<ActorHitEvent>,
    mut miss_event_writer: EventWriter<ActorMissEvent>,
    mut death_event_writer: EventWriter<ActorDeathEvent>,
    mut attack_event_reader: EventReader<ActorAttackEvent>,
    query: Query<(Entity, &mut Actor, &Health)>,
) {
    for event in attack_event_reader.read() {
        if let Ok(
            [
                (attacker_entity, attacker, _),
                (defender_enttiy, defender, health),
            ],
        ) = query.get_many([event.attacker, event.defender])
        {
            if roll_hit_chance(attacker, defender) {

                let is_crit = roll_crit_chance(attacker);
                let mut damage = calculate_damage_chance();

                if is_crit {
                    damage = (damage as f32 * 1.5) as i32;
                }

                if health.0 - damage > 0 {
                    hit_event_writer.send(ActorHitEvent {
                        attacker: attacker_entity,
                        defender: defender_enttiy,
                        damage_dealt: damage,
                        is_crit
                    });
                } else {
                    death_event_writer.send(ActorDeathEvent {
                        attacker: attacker_entity,
                        defender: defender_enttiy,
                    });
                }
            } else {
                miss_event_writer.send(ActorMissEvent {
                    attacker: attacker_entity,
                    defender: defender_enttiy,
                });
            }
        }
    }
}