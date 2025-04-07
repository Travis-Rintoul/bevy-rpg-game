use bevy::{ecs::entity::Entity, math::Vec3, prelude::Event};

#[derive(Event)]
pub struct RayCastHitEvent {
    pub hit_entity: Entity,
	pub left_click: bool,
	pub right_click: bool,
	pub point: Vec3
}
