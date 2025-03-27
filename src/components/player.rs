use bevy::prelude::Component;

#[derive(Component)]
pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Player {}
    }

    pub fn name(&self) -> String {
        String::from("John Doe")
    }
}
