use bevy::{
    asset::Assets,
    color::Color,
    ecs::system::{Commands, Query, ResMut},
    math::{
        Quat, Vec2,
        primitives::{Extrusion, RegularPolygon},
    },
    pbr::{MeshMaterial3d, StandardMaterial},
    render::mesh::{Mesh, Mesh3d},
    transform::components::Transform,
    utils::default,
};

use crate::plugins::grid_system_plugin::{
    HEX_GRID_RADIUS, HexDirection, TEMP_HEX_GRID_HEIGHT, TEMP_HEX_GRID_WIDTH,
    components::{HexGrid, HexTile},
    models::GridMapPoint,
};

use super::grid::calculate_next_point;

pub fn spawn_hexes(
    query: Query<&HexGrid>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (grid_map) in query.iter() {
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 1.0, 0.1),
            ..default()
        });

        let grid_point = grid_map.point(GridMapPoint::TopLeft);

        let mut prev_vec: Vec2 = grid_point;
        let mut start_point = Vec2::new(grid_point.x, grid_point.y);

        for i in 0..=TEMP_HEX_GRID_WIDTH {
            if i > 0 {
                start_point = calculate_next_point(
                    start_point.x as f64,
                    start_point.y as f64,
                    HexDirection::Bottom,
                );
            }

            for z in 0..=TEMP_HEX_GRID_HEIGHT {
                let position: Vec2;

                if z == 0 {
                    position = Vec2::new(start_point.x, start_point.y);
                } else if z % 2 == 0 {
                    position = calculate_next_point(
                        prev_vec.x as f64,
                        prev_vec.y as f64,
                        HexDirection::TopRight,
                    );
                } else {
                    position = calculate_next_point(
                        prev_vec.x as f64,
                        prev_vec.y as f64,
                        HexDirection::BottomRight,
                    );
                }

                commands.spawn((
                    HexTile::new(z, i),
                    Mesh3d(meshes.add(Extrusion::new(
                        RegularPolygon::new(HEX_GRID_RADIUS as f32, 6),
                        0.05,
                    ))),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(position.x, 0.0, position.y)
                        .with_rotation(Quat::from_rotation_x(std::f64::consts::PI as f32 / 2.0)),
                ));

                prev_vec = position.clone();
            }
        }
    }
}
