use bevy::prelude::*;

use super::{
    PlayerMovementStatus, PlayerState,
    structs::PlayerLastClick,
    systems::{camera_zoom_system, player_attack_dispatcher, player_dialog_event_listener, player_move_dispatcher, player_dialog_dispatcher},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(PlayerMovementStatus::Enabled)
            .insert_state(PlayerState::Exploring)
            .insert_resource(PlayerLastClick(0.0))
            .add_systems(Update, player_attack_dispatcher)
            .add_systems(Update, player_move_dispatcher)
            .add_systems(Update, player_dialog_dispatcher)
            .add_systems(
                Update,
                camera_zoom_system
                    .run_if(in_state(PlayerMovementStatus::Enabled))
                    .run_if(|state: Res<State<PlayerState>>| {
                        state.get() == &PlayerState::Exploring
                            || state.get() == &PlayerState::InCombat
                    }),
            )
            .add_systems(Update, player_dialog_event_listener);
    }
}
