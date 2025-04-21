use bevy::prelude::Component;

use crate::plugins::grid_system_plugin::AxialCoord;

#[derive(Component)]
pub struct ActorHexMovementCommand {
    pub path: Vec<AxialCoord>,
    pub current_step: u32,
}

#[derive(Component)]
pub struct ActorFreeMovementCommand {
    pub path: Vec<AxialCoord>,
    pub current_step: u32,
}