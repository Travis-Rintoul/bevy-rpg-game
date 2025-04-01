use serde::Deserialize;

#[allow(dead_code)] // TODO remove dead_code
#[derive(Deserialize, Debug, Clone)]
pub struct WeaponDefinition {
    pub name: String,
}