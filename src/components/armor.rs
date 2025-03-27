use bevy::prelude::Component;

#[derive(Component)]
pub struct Armor;

impl Armor {
	pub fn new() -> Self {
		Armor { }
	}
}