use bevy::{
    core_pipeline::core_3d::Camera3d, ecs::system::Commands, math::Vec3, prelude::PointLight,
    render::camera::Camera, transform::components::Transform, utils::default,
};

use crate::components::main_camera::MainCamera;

pub fn spawn_camera(commands: &mut Commands) {
    let transform = Transform::from_xyz(0.0, 12.0, 0.0) // Position
        .looking_at(Vec3::ZERO, Vec3::Y); // Look at the center of the scene

    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        MainCamera::new(),
        transform,
    ));
}

pub fn setup_lights_and_cameras(commands: &mut Commands) {
    //println!("setup_lights_and_cameras");
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 8.0, 0.0),
    ));

    spawn_camera(commands);
}
