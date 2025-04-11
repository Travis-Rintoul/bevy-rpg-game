use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::primitives::{Extrusion, RegularPolygon},
    pbr::StandardMaterial,
    prelude::Color,
    render::mesh::Mesh,
    utils::default,
};

use crate::plugins::grid_system_plugin::{HEX_GRID_RADIUS, models::HexTileAssets};

pub fn setup_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(HexTileAssets {
        mesh: meshes.add(Extrusion::new(
            RegularPolygon::new(HEX_GRID_RADIUS as f32, 6),
            0.05,
        )),
        material_standard_hex: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 1.0, 0.1),
            ..default()
        }),
        material_focused_hex: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.0, 0.0, 1.0),
            ..default()
        }),
    });
}
