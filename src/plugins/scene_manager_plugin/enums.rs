use bevy::prelude::States;
use bevy::prelude::SystemSet;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameScene {
    Sandbox,
    Location1,
    Location2,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSceneStatus {
    Loading,
    Ready,
}

#[allow(dead_code)] // TODO remove dead_code
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    InGame,
    InMap,
    InMenu,
}

#[allow(dead_code)] // TODO: remove
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StartupPhase {
    SceneLoad,
    ComputeHexGrid,
    SpawnHexTiles,
    SceneSetup,
    PlayerSpawn,
}
