use bevy::prelude::*;

use crate::plugins::actor_plugin::events::{
    PlayerAttackEvent, PlayerDialogInitiatedEvent, PlayerGridMoveEvent,
};

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDialogInitiatedEvent>()
            .add_event::<PlayerAttackEvent>()
            .add_event::<PlayerGridMoveEvent>();
    }
}
