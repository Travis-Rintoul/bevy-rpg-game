use bevy::prelude::Component;

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