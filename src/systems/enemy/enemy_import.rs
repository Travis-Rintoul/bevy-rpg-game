use bevy::{ecs::system::ResMut, utils::HashMap};
use serde_json;
use std::fs;

use crate::models::structs::enemy_definition::{EnemyDefinition, EnemyDefinitionCache};

pub fn load_enemy_data(mut cache: ResMut<EnemyDefinitionCache>) {

    let file_content =
        fs::read_to_string("assets/data/enemies.json").expect("Failed to read enemy data");

    let enemy_definitions: HashMap<i32, EnemyDefinition> =
        serde_json::from_str(&file_content).expect("Invalid JSON format");

    for (id, enemy) in enemy_definitions {
        cache.add(id, enemy);
    }
}
