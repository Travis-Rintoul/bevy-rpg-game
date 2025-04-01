use bevy::prelude::States;

#[allow(dead_code)] // TODO remove dead_code
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    InGame,
    InMap,
    InMenu,
}
