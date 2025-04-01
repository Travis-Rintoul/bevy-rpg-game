use std::fmt;
use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameScene {
    Location1,
    Location2
}

impl fmt::Display for GameScene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameScene::Location1 => write!(f, "Location 1"),
            GameScene::Location2 => write!(f, "Location 2"),
        }
    }
}
