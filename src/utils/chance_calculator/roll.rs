use rand::Rng;

use crate::plugins::actor_plugin::components::Actor;

use super::formula;

pub fn roll() -> i32 {
    rand::rng().random_range(0..=100)
}

pub fn roll_hit_chance(attacker: &Actor, target: &Actor) -> bool {
    let roll = roll();
    let calc = formula::hit_chance(
        attacker.weapon_proficiency(),
        attacker.stats.fortune,
        target.skills.dodge_prof,
        target.stats.fortune,
    );
    calc > roll
}

pub fn roll_crit_chance(actor: &Actor) -> bool {
    formula::crit_chance(actor.stats.fortune) > roll()
}
