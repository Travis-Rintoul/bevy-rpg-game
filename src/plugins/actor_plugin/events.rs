use bevy::ecs::entity::Entity;
use bevy::prelude::Event;

#[derive(Event, Debug, Clone)]
pub struct ActorGridMoveEvent {
    pub(crate) actor: Entity,
    pub(crate) from_tile: Entity,
    pub(crate) to_tile: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct PlayerGridMoveEvent {
    pub(crate) to_tile: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct ActorAttackEvent {
    pub(crate) attacker: Entity,
    pub(crate) defender: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct PlayerAttackEvent {
    pub(crate) defender: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct ActorHitEvent {
    pub(crate) attacker: Entity,
    pub(crate) defender: Entity,
    pub(crate) damage_dealt: i32,
    pub(crate) is_crit: bool,
}

#[derive(Event, Debug, Clone)]
pub struct ActorMissEvent {
    pub(crate) attacker: Entity,
    pub(crate) defender: Entity,
}

#[allow(dead_code)] // TODO: remove dead_code
#[derive(Event, Debug, Clone)]
pub struct ActorItemPickupEvent {
    actor: Entity,
    item: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct ActorDeathEvent {
    pub(crate) attacker: Entity,
    pub(crate) defender: Entity,
}

#[allow(dead_code)] // TODO: remove dead_code
#[derive(Event, Debug, Clone)]
pub struct ActorSpawnEvent;

#[derive(Event, Debug, Clone)]
pub struct ActorDialogInitiatedEvent {
    pub(crate) initiator: Entity,
    pub(crate) recipient: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct PlayerDialogInitiatedEvent {
    pub(crate) recipient: Entity,
}
