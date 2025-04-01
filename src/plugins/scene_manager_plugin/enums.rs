use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameScene {
    Location1,
    Location2,
}

#[allow(dead_code)] // TODO remove dead_code
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    InGame,
    InMap,
    InMenu,
}
