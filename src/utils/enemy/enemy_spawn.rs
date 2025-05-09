use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        entity::Entity,
        system::{Commands, ResMut},
    },
    math::{Vec3, primitives::Sphere},
    pbr::{MeshMaterial3d, StandardMaterial},
    render::mesh::{Mesh, Mesh3d},
    transform::components::Transform,
};

use crate::{
    components::enemy::Enemy,
    plugins::actor_plugin::components::{Actor, ActorHealth},
};

#[allow(dead_code)] // TODO: remove dead_code
pub fn spawn_enemy(
    position: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let enemy = Enemy::new();
    let actor = Actor::new(enemy.name());
    let health = ActorHealth(actor.base_health);

    commands
        .spawn((
            Mesh3d(meshes.add(Sphere::default())),
            MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
            Transform::from_xyz(position.x, position.y, position.z),
            enemy,
            actor,
            health,
        ))
        .id()
}
