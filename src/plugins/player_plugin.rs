use bevy::prelude::*;

use crate::{
    models::enums::player_movement_status::PlayerMovementStatus,
    systems::{
        camera::camera_movement::camera_zoom_system,
        player::player_movement::player_raycast_handler,
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(PlayerMovementStatus::Enabled)
            .add_systems(
                Update,
                player_raycast_handler.run_if(in_state(PlayerMovementStatus::Enabled)),
            )
            // .add_systems(
            //     Update,
            //     player_event_listener.run_if(in_state(PlayerMovementStatus::Enabled)),
            // )
            .add_systems(
                Update,
                camera_zoom_system.run_if(in_state(PlayerMovementStatus::Enabled)),
            );
    }
}
