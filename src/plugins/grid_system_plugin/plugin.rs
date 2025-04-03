use bevy::app::{App, Plugin, Startup};


use super::systems::spawn_hexes;
pub struct GridSystemPlugin;

impl Plugin for GridSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hexes);
    }
}