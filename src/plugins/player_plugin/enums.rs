use bevy::prelude::States;

#[allow(dead_code)] // TODO remove dead_code
#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerMovementStatus {
    Enabled,
    Disabled,
}

#[allow(dead_code)] // TODO remove dead_code
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    None,
    Exploring,
    InCombat,
    InDialog,
}
