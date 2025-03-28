use bevy::{ecs::system::ResMut, prelude::Resource, utils::HashMap};

use crate::models::structs::enemy_definition::EnemyDefinition;

use super::load_cache::load_cache;

#[derive(Resource)]
pub struct EnemyDefinitionCache(HashMap<i32, EnemyDefinition>);

impl EnemyDefinitionCache {
    pub fn new() -> Self {
        EnemyDefinitionCache(HashMap::default())
    }

    pub fn fetch(&self) -> &HashMap<i32, EnemyDefinition> {
        &self.0
    }
}

pub fn load_enemy_cache(mut cache: ResMut<EnemyDefinitionCache>) {
    let path = "assets/data/enemies.json";
    if let Err(err) = load_cache(path, &mut cache.0) {
        eprintln!("Error loading enemy data: {}", err);
    }
}
