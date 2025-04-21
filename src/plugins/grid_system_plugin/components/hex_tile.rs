use bevy::prelude::Component;

use crate::plugins::grid_system_plugin::models::AxialCoord;



#[derive(Component)]
pub struct HexTile {
	pub coord: AxialCoord,
	pub occupied: bool,
}

impl HexTile {
	pub fn new(q: i32, r: i32) -> Self {
		HexTile {
			coord: AxialCoord { q: q, r: r },
			occupied: false
		}
	}
}