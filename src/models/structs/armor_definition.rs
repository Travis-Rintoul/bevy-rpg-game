use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ArmorDefinition {
	name: String,
    defense: f32,
    durability: f32,
    weight: f32,
}