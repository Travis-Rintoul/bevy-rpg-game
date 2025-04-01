use bevy::prelude::States;

#[allow(dead_code)] // TODO remove dead_code
#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerMovementStatus {
    Enabled,
    Disabled,
}