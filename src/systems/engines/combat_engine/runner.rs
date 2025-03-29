use std::any::TypeId;
use super::{chance::CombatChanceEngine, context::CombatEngineContext, models::{engine_hook::EngineHook, engine_hooks::OnHitEvent}};

pub struct CombatEngineRunner {
    ctx: CombatEngineContext,
}

impl CombatEngineRunner {

    pub fn run(ctx: CombatEngineContext) {
        for (entity, actor) in ctx.defenders.iter() {
            CombatEngineRunner::emit_event(&ctx, &OnHitEvent { 
                recieiver: actor.clone(),
                damage: 20.0
            });
        }
    }

    fn emit_event<T: EngineHook>(ctx: &CombatEngineContext, event: &T) {
        let type_id = TypeId::of::<T>();
        if let Some(listeners) = ctx.listeners.get(&type_id) {
            for listener in listeners {
                listener(event);
            }
        }
    }
}