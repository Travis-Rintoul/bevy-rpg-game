use bevy::{ecs::entity::Entity, utils::hashbrown::HashMap};

use crate::components::actor::actor::Actor;

use std::any::TypeId;

use super::{
    context::CombatEngineContext, event_handler::CombatEngineEventHandler,
    models::engine_hook::EngineHook, runner::CombatEngineRunner,
};

pub struct CombatEngine {
    pub ctx: CombatEngineContext,
    pub event_handler: CombatEngineEventHandler,
}

impl CombatEngine {
    pub fn new() -> Self {
        CombatEngine {
            event_handler: CombatEngineEventHandler::new(),
            ctx: CombatEngineContext {
                attacker: None,
                defenders: HashMap::new(),
            },
        }
    }

    pub fn set_attacker(&mut self, entity: Entity, attacker: Actor) -> &mut Self {
        self.ctx.attacker = Some((entity, attacker));
        self
    }

    pub fn add_defender(&mut self, entity: Entity, defender: Actor) -> &mut Self {
        if !self.ctx.defenders.contains_key(&entity) {
            self.ctx.defenders.insert(entity, defender);
        }
        self
    }

    pub fn add_listener<T: EngineHook>(&mut self, listener: impl Fn(&T) + 'static) -> &mut Self {
        self.event_handler.add_listener::<T>(listener);
        self
    }

    pub fn execute(&mut self) -> &mut Self {
        CombatEngineRunner::run(&self.ctx, &mut self.event_handler);
        self
    }

    pub fn get_events(&mut self) -> &HashMap<TypeId, Vec<Box<dyn EngineHook>>> {
        &self.event_handler.get_events()
    }

    pub fn debug(&self) {
        println!("{:#?}", self.ctx.attacker);
        println!("{:#?}", self.ctx.defenders);
    }
}
