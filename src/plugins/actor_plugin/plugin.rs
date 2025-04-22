use bevy::app::{App, Plugin, Update};

use super::{
    events::{
        ActorAttackEvent, ActorDeathEvent, ActorDialogInitiatedEvent, ActorGridMoveEvent,
        ActorHitEvent, ActorMissEvent,
    },
    systems::{
        attack_event_listener, death_event_listener, hit_event_listener, miss_event_listener,
        move_event_listener, perform_move_event,
    },
};

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActorAttackEvent>()
            .add_event::<ActorMissEvent>()
            .add_event::<ActorHitEvent>()
            .add_event::<ActorDeathEvent>()
            .add_event::<ActorDialogInitiatedEvent>()
            .add_event::<ActorGridMoveEvent>()
            .add_systems(Update, move_event_listener)
            .add_systems(Update, perform_move_event)
            .add_systems(Update, attack_event_listener)
            .add_systems(Update, hit_event_listener)
            .add_systems(Update, miss_event_listener)
            .add_systems(Update, death_event_listener);
    }
}
