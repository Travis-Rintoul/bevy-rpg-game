use bevy::{ecs::entity::Entity, utils::hashbrown::HashMap};

use crate::components::actor::actor::Actor;

use std::any::TypeId;

use super::{context::CombatEngineContext, models::engine_hook::EngineHook, runner::CombatEngineRunner};

pub struct CombatEngine {
    pub ctx: CombatEngineContext,
}

impl CombatEngine {
    pub fn new() -> Self {
        CombatEngine {
            ctx: CombatEngineContext {
                listeners: HashMap::new(),
                attacker: None,
                defenders: HashMap::new(),
            },
        }
    }

    pub fn set_attacker(mut self, entity: Entity, attacker: Actor) -> Self {
        self.ctx.attacker = Some((entity, attacker));
        self
    }

    pub fn add_defender(mut self, entity: Entity, defender: Actor) -> Self {
        if !self.ctx.defenders.contains_key(&entity) {
            self.ctx.defenders.insert(entity, defender);
        }
        self
    }

    pub fn add_listener<T: EngineHook>(&mut self, listener: impl Fn(&T) + 'static) {
        let type_id = TypeId::of::<T>();
        let wrapped = Box::new(move |hook: &dyn EngineHook| {
            if let Some(event) = hook.as_any().downcast_ref::<T>() {
                listener(event);
            }
        }) as Box<dyn Fn(&dyn EngineHook)>;

        self.ctx.listeners.entry(type_id).or_default().push(wrapped);
    }

    pub fn run(mut self) {
        CombatEngineRunner::run(self.ctx);
    }

    pub fn debug(&self) {
        println!("{:#?}", self.ctx.attacker);
        println!("{:#?}", self.ctx.defenders);
    }
}
