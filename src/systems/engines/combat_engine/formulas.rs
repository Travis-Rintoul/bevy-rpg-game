pub struct CombatEngineFormulas;

impl CombatEngineFormulas {
    pub fn hit_chance(
        attacker_weapon_proficiency: i32,
        attacker_luck: i32,
        target_evasion: i32,
        target_luck: i32,
    ) -> i32 {
        (attacker_weapon_proficiency + (attacker_luck * 2)) - (target_evasion + target_luck).max(0)
    }

    pub fn crit_chance(luck: i32) -> i32 {
        (luck as f32 * 1.4).ceil() as i32 + 6
    }
}
