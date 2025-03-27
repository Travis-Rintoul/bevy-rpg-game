use bevy::{app::Plugin, prelude::Component};

use crate::models::structs::enemy_definition::EnemyDefinition;

#[derive(Component)]
pub struct Enemy;

impl Enemy {

    pub fn new() -> Self {
        Enemy {}
    }

    pub fn name(&self) -> String {
        String::from("Enemy")
    }
}