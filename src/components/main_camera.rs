use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

impl MainCamera {
	pub fn new() -> Self {
		MainCamera {}
	}
}