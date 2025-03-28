use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WeaponDefinition {
    pub name: String,
}