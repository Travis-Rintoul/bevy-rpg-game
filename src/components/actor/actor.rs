use bevy::prelude::Component;

use crate::components::{armor::Armor, weapon::Weapon};

use super::{
    actor_abilities::ActorAbilities, actor_perks::ActorPerks, actor_skills::ActorSkills,
    actor_stats::ActorStats,
};

#[derive(Component)]
pub struct Actor {
    name: String,
    base_health: i32,
    stats: ActorStats,
    skills: ActorSkills,
    perks: ActorPerks,
    abilities: ActorAbilities,
    weapon: Option<Weapon>,
    armor: Option<Armor>,
}

impl Actor {
    pub fn new(name: String) -> Self {
        Actor {
            name: name,
            base_health: 0,
            stats: ActorStats {},
            skills: ActorSkills {},
            perks: ActorPerks {},
            abilities: ActorAbilities {},
            weapon: None,
            armor: None,
        }
    }
}
