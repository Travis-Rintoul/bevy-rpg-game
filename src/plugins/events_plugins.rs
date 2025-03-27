use bevy::app::{App, Plugin};

use crate::models::events::actor_events::{ActorAttackEvent, ActorMoveEvent};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) 
    {
        app
            .add_event::<ActorMoveEvent>()
            .add_event::<ActorAttackEvent>();

    }
}