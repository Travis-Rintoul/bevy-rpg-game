use bevy::app::{App, Plugin, Update};

use crate::{
    models::events::actor_events::{
        ActorAttackEvent, ActorDeathEvent, ActorDialogInitiatedEvent, ActorHitEvent,
        ActorMissEvent, ActorMoveEvent,
    },
    systems::actor::{
        actor_attack::attack_event_listener,
        actor_death::death_event_listener,
        actor_hit::hit_event_listener,
        actor_miss::miss_event_listener,
        actor_move::{move_event_listener, perform_move_event},
    },
};

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActorMoveEvent>()
            .add_event::<ActorAttackEvent>()
            .add_event::<ActorMissEvent>()
            .add_event::<ActorHitEvent>()
            .add_event::<ActorDeathEvent>()
            .add_event::<ActorDialogInitiatedEvent>()
            .add_systems(Update, move_event_listener)
            .add_systems(Update, perform_move_event)
            .add_systems(Update, attack_event_listener)
            .add_systems(Update, hit_event_listener)
            .add_systems(Update, miss_event_listener)
            .add_systems(Update, death_event_listener);
    }
}
