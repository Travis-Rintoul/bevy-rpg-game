// use bevy::utils::hashbrown::HashMap;

// use super::{context::CombatEngineContext, models::engine_hook::EngineHook};
use std::any::{Any, TypeId};

use bevy::utils::HashMap;

use super::models::engine_hook::EngineHook;

pub struct CombatEngineEventHandler {
    event_buffer: HashMap<TypeId, Vec<Box<dyn EngineHook>>>,
    pub listeners: HashMap<TypeId, Vec<Box<dyn Fn(&dyn EngineHook)>>>,
}

impl CombatEngineEventHandler {
    pub fn new() -> Self {
        CombatEngineEventHandler {
            event_buffer: HashMap::default(),
            listeners: HashMap::default(),
        }
    }

    pub fn store<T: EngineHook>(&mut self, event: T) {
        self.event_buffer
            .entry(TypeId::of::<T>())
            .or_default()
            .push(Box::new(event));
    }

    pub fn emit<T: EngineHook + Clone>(&mut self, event: T) {
        // Store the event in the event buffer (with cloning)
        self.store(event.clone());

        // Now borrow the event as a reference
        let event_ref: &dyn EngineHook = &event;

        // Retrieve and call the listeners
        if let Some(listeners) = self.listeners.get(&TypeId::of::<T>()) {
            for listener in listeners {
                // Call the listener with a reference to the event
                listener(event_ref);
            }
        }
    }

    pub fn add_listener<T: EngineHook>(&mut self, listener: impl Fn(&T) + 'static) -> &mut Self {
        let wrapped = Box::new(move |hook: &dyn EngineHook| {
            if let Some(event) = hook.as_any().downcast_ref::<T>() {
                listener(event);
            }
        }) as Box<dyn Fn(&dyn EngineHook)>;

        self.listeners
            .entry(TypeId::of::<T>())
            .or_default()
            .push(wrapped);
        self
    }

    pub fn get_events(&self) -> &HashMap<TypeId, Vec<Box<dyn EngineHook>>> {
        &self.event_buffer
    }
}
