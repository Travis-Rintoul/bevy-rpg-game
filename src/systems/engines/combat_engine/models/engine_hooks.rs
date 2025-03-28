use crate::components::actor::actor::Actor;

pub struct OnHitEvent {
	pub recieiver: Actor,
    pub damage: f32,
}
