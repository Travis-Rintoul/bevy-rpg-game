use serde::Deserialize;

#[allow(dead_code)] // TODO remove dead_code
#[derive(Deserialize, Debug, Clone)]
pub struct EnemyDefinition {
    pub name: String,
    pub base_health: i32,
    pub armor: Option<i32>,
    pub weapon: Option<i32>,
}

#[allow(dead_code)] // TODO remove dead_code
#[derive(Deserialize, Debug, Clone)]
pub struct ArmorDefinition {
    name: String,
    defense: f32,
    durability: f32,
    weight: f32,
}

#[allow(dead_code)] // TODO remove dead_code
#[derive(Deserialize, Debug, Clone)]
pub struct WeaponDefinition {
    pub name: String,
}
