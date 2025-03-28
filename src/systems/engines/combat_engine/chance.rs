use rand::Rng;

pub struct CombatChanceEngine;

impl CombatChanceEngine {

    pub fn rand() -> i32 {
        rand::rng().random_range(0..=100)
    }

    pub fn roll() -> () {
        
    }
}