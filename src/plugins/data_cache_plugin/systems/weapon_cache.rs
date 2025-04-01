use bevy::{ecs::system::ResMut, prelude::Resource, utils::HashMap};

use crate::plugins::data_cache_plugin::structs::WeaponDefinition;

use super::load_cache;

#[derive(Resource)]
pub struct WeaponDefinitionCache(HashMap<i32, WeaponDefinition>);

impl WeaponDefinitionCache {
    pub fn new() -> Self {
        WeaponDefinitionCache(HashMap::default())
    }

    pub fn fetch(&self) -> &HashMap<i32, WeaponDefinition> {
        &self.0
    }
}

pub fn load_weapon_cache(mut cache: ResMut<WeaponDefinitionCache>) {
    let path = "assets/data/weapons.json";
    if let Err(err) = load_cache(path, &mut cache.0) {
        eprintln!("Error loading weapon data: {}", err);
    }
}
