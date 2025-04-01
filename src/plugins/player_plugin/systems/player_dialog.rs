use bevy::{
    ecs::{
        entity::Entity,
        event::EventReader,
        system::{Query, ResMut},
    },
    state::state::NextState,
};

use crate::{
    components::actor::Actor,
    plugins::{actor_plugin::events::ActorDialogInitiatedEvent, player_plugin::PlayerState},
};

pub fn player_dialog_event_listener(
    mut dialog_event_reader: EventReader<ActorDialogInitiatedEvent>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    query: Query<(Entity, &Actor)>,
) {
    for event in dialog_event_reader.read() {
        if let Ok([(_, initiator), (_, recipient)]) =
            query.get_many([event.initiator, event.recipient])
        {
            println!(
                "{} has initiated dialog with {}",
                initiator.name, recipient.name
            );
            next_player_state.set(PlayerState::InDialog);
        }
    }
}
