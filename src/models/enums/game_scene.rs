use std::fmt;
use bevy::prelude::States;

use crate::scenes::test_scene::TestScene;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameScene {
    TestScene,
    Location1,
    Location2
}

impl fmt::Display for GameScene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameScene::TestScene => write!(f, "Test"),
            GameScene::Location1 => write!(f, "Location 1"),
            GameScene::Location2 => write!(f, "Location 2"),
        }
    }
}
