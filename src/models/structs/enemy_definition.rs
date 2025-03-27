use bevy::{prelude::Resource, utils::HashMap};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct EnemyDefinition {
    pub name: String,
    pub base_health: i32,
    pub armor: Option<i32>,
    pub weapon: Option<i32>,
}

#[derive(Resource)]
pub struct EnemyDefinitionCache {
    data: HashMap<i32, EnemyDefinition>,
}

impl EnemyDefinitionCache {
    pub fn new() -> Self {
        EnemyDefinitionCache { data: HashMap::new() }
    }

    pub fn find(&self, id: i32) -> Option<&EnemyDefinition> {
        self.data.get(&id)
    }

    pub fn add(&mut self, id: i32, enemy: EnemyDefinition) {
        self.data.insert(id, enemy);
    }

    pub fn print(&self) {
        let values: Vec<&EnemyDefinition> = self.data.values().collect();
        println!("{:?}", values);
    }
}