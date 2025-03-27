use bevy::{app::{App, Plugin, Startup}, ecs::system::Res};

use crate::{models::structs::enemy_definition::EnemyDefinitionCache, systems::enemy::enemy_import::load_enemy_data};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) 
    {
        app
            .insert_resource(EnemyDefinitionCache::new())
            .add_systems(Startup, (load_enemy_data, test_load));
    }
}

fn test_load(res: Res<EnemyDefinitionCache>) {
    match res.find(2) {
        Some(enemy) => println!("{:?}", enemy),
        _ => println!("not found")
    }
}