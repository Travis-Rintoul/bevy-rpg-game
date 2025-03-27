use bevy::{
    ecs::{event::EventReader, query::With, system::Query},
    input::mouse::MouseWheel,
    transform::components::Transform,
};

use crate::components::main_camera::MainCamera;

pub fn camera_zoom_system(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    let mut camera_transform = camera_query.single_mut();

    for event in scroll_events.read() {
        let zoom_amount = event.y * 0.5;
        let forward = camera_transform.forward();
        camera_transform.translation += forward * zoom_amount;
    }
}
