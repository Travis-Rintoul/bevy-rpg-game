use bevy::prelude::Component;

use crate::components::{armor::Armor, weapon::{Weapon, WeaponClass}};

use super::{
    actor_abilities::ActorAbilities, actor_perks::ActorPerks, actor_skills::ActorSkills,
    actor_stats::ActorStats,
};

#[allow(dead_code)] // TODO: remove dead_code
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
            name: String::default(),
            base_health: 100,
            stats: ActorStats::default(),
            skills: ActorSkills::default(),
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
            base_health: 100,
            stats: ActorStats::default(),
            skills: ActorSkills::default(),
            perks: ActorPerks {},
            abilities: ActorAbilities {},
            weapon: None,
            armor: None,
        }
    }

    pub fn weapon_proficiency(&self) -> i32 {
        if let Some(weapon) = &self.weapon {
            match weapon.class {
                WeaponClass::Gun => self.skills.gun_prof,
                WeaponClass::Bow => self.skills.bow_prof,
                WeaponClass::Melee => self.skills.melee_prof,
                WeaponClass::Thrown => self.skills.thrown_prof,
                WeaponClass::Unarmed => self.skills.unarmed_prof,
            }
        } else {
            // if no weapon is selected the actor is considered unarmed, using their fists
            self.skills.unarmed_prof
        }
    }
}
