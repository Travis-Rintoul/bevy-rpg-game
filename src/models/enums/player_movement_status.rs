use bevy::prelude::States;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerMovementStatus {
    Enabled,
    Disabled,
}