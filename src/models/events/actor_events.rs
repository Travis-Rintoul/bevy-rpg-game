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
    pub(crate) attacker: Entity, 
    pub(crate) defender: Entity,
    pub(crate) damage_dealt: i32, 
    pub(crate) is_crit: bool
}

#[derive(Event, Debug, Clone)]
pub struct ActorMissEvent {
    pub(crate) attacker: Entity, 
    pub(crate) defender: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct ActorItemPickupEvent {
    actor: Entity, 
    item: Entity 
}

#[derive(Event, Debug, Clone)]
pub struct ActorDeathEvent {
    pub(crate) attacker: Entity, 
    pub(crate) defender: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct ActorSpawnEvent;