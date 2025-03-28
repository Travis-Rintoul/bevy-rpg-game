use bevy::prelude::Component;

#[derive(Component, Clone, Debug)]
pub struct Weapon {
	name: String,
}

impl Default for Weapon {
	fn default() -> Self {
		Weapon { name: String::from("Fists") }
	}
}