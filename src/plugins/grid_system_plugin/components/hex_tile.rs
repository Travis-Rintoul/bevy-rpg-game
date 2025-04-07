use bevy::prelude::Component;

pub struct AxialCoord {
	pub q: i32,
	pub r: i32,
}

#[derive(Component)]
pub struct HexTile {
	coord: AxialCoord,
	ocupied: bool,
}


impl HexTile {
	pub fn new(q: i32, r: i32) -> Self {
		HexTile {
			coord: AxialCoord { q: q, r: r },
			ocupied: false
		}
	}
}