use super::{
    context::CombatEngineContext, event_handler::CombatEngineEventHandler,
    models::engine_hooks::OnHitEvent,
};

pub struct CombatEngineRunner;

impl CombatEngineRunner {
    pub fn run(ctx: &CombatEngineContext, event_handler: &mut CombatEngineEventHandler) {
        for (entity, actor) in ctx.defenders.iter() {
            event_handler.emit(OnHitEvent {
                recieiver: actor.clone(),
                damage: 20.0,
            });
        }
    }
}
