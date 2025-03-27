use bevy::app::{App, Plugin, Update};

use crate::systems::actor::actor_event_handler::{attack_event_listener, move_event_listener};

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_event_listener)
            .add_systems(Update, attack_event_listener);
    }
}
