use bevy::prelude::Component;

use crate::components::{armor::Armor, weapon::Weapon};

use super::{
    actor_abilities::ActorAbilities, actor_perks::ActorPerks, actor_skills::ActorSkills,
    actor_stats::ActorStats,
};

#[derive(Component, Clone, Debug)]
pub struct Actor {
    pub name: String,
    pub base_health: i32,
    pub stats: ActorStats,
    pub skills: ActorSkills,
    pub perks: ActorPerks,
    pub abilities: ActorAbilities,
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,
}

impl Default for Actor {
    fn default() -> Self {
        Actor {
            name: String::from(""),
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

impl Actor {
    pub fn new(name: String) -> Self {
        Actor {
            name: String::from(name),
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
