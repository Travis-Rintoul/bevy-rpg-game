use bevy::prelude::Component;

#[derive(Component, Clone, Debug)]
pub struct Armor;

impl Armor {
	pub fn new() -> Self {
		Armor { }
	}
}