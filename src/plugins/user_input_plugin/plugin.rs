use bevy::prelude::*;

use crate::plugins::actor_plugin::events::{
    PlayerAttackEvent, PlayerDialogInitiatedEvent, PlayerGridMoveEvent, PlayerMoveEvent,
};

use super::{
    models::RayCastHitEvent,
    systems::{
        mouse_raycast_emitter, player_grid_move_event_dispatcher, player_move_event_dispatcher,
        raycast_event_dispatcher,
    },
};

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RayCastHitEvent>()
            .add_event::<PlayerDialogInitiatedEvent>()
            .add_event::<PlayerAttackEvent>()
            .add_event::<PlayerMoveEvent>()
            .add_event::<PlayerGridMoveEvent>()
            .add_systems(
                Update,
                (
                    raycast_event_dispatcher,
                    mouse_raycast_emitter,
                    player_grid_move_event_dispatcher,
                    player_move_event_dispatcher,
                ),
            );
    }
}
