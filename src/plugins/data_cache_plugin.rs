use bevy::{
    app::{App, Plugin, Startup},
    ecs::system::ResMut,
};

use crate::systems::cache::{
    armor_cache::{ArmorDefinitionCache, load_armor_cache},
    enemy_cache::{EnemyDefinitionCache, load_enemy_cache},
    weapon_cache::{WeaponDefinitionCache, load_weapon_cache},
};

pub struct DataCachePlugin;

impl Plugin for DataCachePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyDefinitionCache::new())
            .insert_resource(WeaponDefinitionCache::new())
            .insert_resource(ArmorDefinitionCache::new())
            .add_systems(
                Startup,
                (load_enemy_cache, load_weapon_cache, load_armor_cache),
            );
            //.add_systems(Startup, debug);
    }
}

#[allow(dead_code)] // TODO remove dead_code
pub fn debug(
    cache1: ResMut<EnemyDefinitionCache>,
    cache2: ResMut<WeaponDefinitionCache>,
    cache3: ResMut<ArmorDefinitionCache>,
) {
    println!("--- Enemy Definitions ---");
    for (id, enemy) in cache1.fetch() {
        println!("ID: {}, Enemy: {:?}", id, enemy);
    }

    println!("--- Weapon Definitions ---");
    for (id, weapon) in cache2.fetch() {
        println!("ID: {}, Weapon: {:?}", id, weapon);
    }

    println!("--- Armor Definitions ---");
    for (id, armor) in cache3.fetch() {
        println!("ID: {}, Armor: {:?}", id, armor);
    }
}
