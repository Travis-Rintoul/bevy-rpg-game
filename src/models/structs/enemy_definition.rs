use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct EnemyDefinition {
    pub name: String,
    pub base_health: i32,
    pub armor: Option<i32>,
    pub weapon: Option<i32>,
}
