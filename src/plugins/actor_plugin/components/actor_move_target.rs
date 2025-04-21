use bevy::{math::Vec3, prelude::Component};

#[derive(Component, Default, Clone, Debug)]
pub struct ActorMoveTarget(pub Vec3);
