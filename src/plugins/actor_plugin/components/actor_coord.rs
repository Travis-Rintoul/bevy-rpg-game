use bevy::prelude::Component;
use crate::plugins::grid_system_plugin::AxialCoord;

#[derive(Component)]
pub struct ActorHexCoord {
	pub(crate) coord: AxialCoord,
}

impl ActorHexCoord {
	pub fn new(coord: AxialCoord) -> ActorHexCoord {
		ActorHexCoord {
			coord: coord
		}
	}
}