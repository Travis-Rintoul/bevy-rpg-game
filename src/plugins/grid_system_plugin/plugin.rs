use bevy::{app::{App, Plugin, Startup, Update}, ecs::schedule::IntoSystemConfigs};

use super::{
    LastAxialCoord,
    models::FirstAxialCoord,
    systems::{setup_assets, spawn_hexes, test_emitter},
};

pub struct GridSystemPlugin;

impl Plugin for GridSystemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FirstAxialCoord(None))
            .insert_resource(LastAxialCoord(None))
            .add_systems(Startup, setup_assets)
            .add_systems(Startup, spawn_hexes.after(setup_assets))
            .add_systems(Update, test_emitter);
    }
}
