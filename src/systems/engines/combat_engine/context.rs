use bevy::{ecs::entity::Entity, utils::HashMap};

use crate::components::actor::actor::Actor;
use std::any::TypeId;

use super::models::engine_hook::EngineHook;

// pub type CombatEngineListener<T> = Box<dyn Fn(&T)>;

pub struct CombateEngineContext {
    pub listeners: HashMap<TypeId, Vec<Box<dyn Fn(&dyn EngineHook)>>>,
    pub attacker: Option<(Entity, Actor)>,
    pub defenders: HashMap<Entity, Actor>,
}

impl CombateEngineContext {
    pub fn new(&self) -> Self {
        CombateEngineContext {
            listeners: HashMap::default(),
            attacker: None,
            defenders: HashMap::default(),
        }
    }
}
