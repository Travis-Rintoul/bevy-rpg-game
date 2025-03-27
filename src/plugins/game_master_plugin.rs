use bevy::app::{App, Plugin, Startup};

use crate::systems::game_master::start::start_game_master;


pub struct GameMasterPlugin;

impl Plugin for GameMasterPlugin {
    fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, start_game_master);
	}
}
