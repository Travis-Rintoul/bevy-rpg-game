use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    None,
    Exploring,
    InCombat,
    InDialog,
}
