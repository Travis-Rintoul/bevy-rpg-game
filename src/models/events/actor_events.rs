use bevy::ecs::entity::Entity;
use bevy::prelude::Vec3;
use bevy::prelude::Event;

#[derive(Event, Debug, Clone)]
pub struct ActorMoveEvent {
    pub(crate) actor: Entity, 
    pub(crate) position: Vec3,
}

#[derive(Event, Debug, Clone)]
pub struct ActorAttackEvent {
    pub(crate) attacker: Entity, 
    pub(crate) defender: Entity
}


#[derive(Event, Debug, Clone)]
pub struct ActorHitEvent {
    pub(crate) prev_event: ActorAttackEvent,
    pub(crate) dealt: f32, 
}

#[derive(Event, Debug, Clone)]
pub struct ActorItemPickupEvent {
    actor: Entity, 
    item: Entity 
}

#[derive(Event, Debug, Clone)]
pub struct ActorDeathEvent {
    actor: Entity,
    killed_by: Option<Entity>
}


#[derive(Event, Debug, Clone)]
pub struct ActorSpawnEvent;