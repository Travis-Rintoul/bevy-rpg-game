use bevy::{ecs::system::ResMut, prelude::Resource, utils::HashMap};

use crate::models::structs::armor_definition::ArmorDefinition;

use super::load_cache::load_cache;

#[derive(Resource)]
pub struct ArmorDefinitionCache(HashMap<i32, ArmorDefinition>);

impl ArmorDefinitionCache {
    pub fn new() -> Self {
        ArmorDefinitionCache(HashMap::default())
    }

    pub fn fetch(&self) -> &HashMap<i32, ArmorDefinition> {
        &self.0
    }
}

pub fn load_armor_cache(mut cache: ResMut<ArmorDefinitionCache>) {
    let path = "assets/data/armor.json";
    if let Err(err) = load_cache(path, &mut cache.0) {
        eprintln!("Error loading armor data: {}", err);
    }
}
