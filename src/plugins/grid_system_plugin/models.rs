use bevy::prelude::Component;
use std::f64::consts::PI;

pub enum HexDirection {
    Bottom = 0,
    BottomLeft = 1,
    TopLeft = 2,
    Top = 3,
    TopRight = 4,
    BottomRight = 5,
}

#[derive(Component, Default, Debug)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl Hex {
    pub fn new(q: i32, r: i32) -> Self {
        Hex { q, r }
    }
}

#[derive(Default, Debug)]
pub struct HexGrid {
    hexes: Vec<Hex>,
}

impl HexGrid {
    pub fn new(width: u32, height: u32) -> Self {
        HexGrid {
            hexes: (0..(width * height))
                .map(|i| Hex {
                    q: (i % width) as i32,
                    r: (i / height) as i32,
                })
                .collect(),
        }
    }

    pub fn get_hex_neighbor(hex: &Hex, direction: HexDirection) -> Option<&Hex> {
        None
    }
}

pub fn hex_grid_angle(direction: HexDirection) -> f64 {
    ((direction as i32) as f64) * PI / 3.0
}
