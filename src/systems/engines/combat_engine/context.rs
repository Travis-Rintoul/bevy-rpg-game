use bevy::{ecs::entity::Entity, utils::HashMap};

use crate::components::actor::actor::Actor;
use std::any::TypeId;

use super::models::engine_hook::EngineHook;

// pub type CombatEngineListener<T> = Box<dyn Fn(&T)>;

pub struct CombatEngineContext {
    pub attacker: Option<(Entity, Actor)>,
    pub defenders: HashMap<Entity, Actor>,
}

impl CombatEngineContext {
    pub fn new(&self) -> Self {
        CombatEngineContext {
            attacker: None,
            defenders: HashMap::default(),
        }
    }
}
