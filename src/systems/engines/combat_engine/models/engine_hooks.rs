use crate::components::actor::actor::Actor;

#[derive(Clone)]
pub struct OnHitEvent {
	pub recieiver: Actor,
    pub damage: f32,
}
