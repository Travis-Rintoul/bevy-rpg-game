use bevy::{
    asset::Handle,
    ecs::{observer::Trigger, system::Query},
    pbr::{MeshMaterial3d, StandardMaterial},
};

pub fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.entity()) {
            material.0 = new_material.clone();
        }
    }
}
