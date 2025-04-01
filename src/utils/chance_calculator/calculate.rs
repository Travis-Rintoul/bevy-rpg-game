use rand::Rng;

pub fn calculate_damage_chance() -> i32 {
    return rand::rng().random_range(10..=20);
}
