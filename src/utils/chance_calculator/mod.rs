mod calculate;
mod formula;
mod roll;

pub use calculate::calculate_damage_chance;
pub use formula::{crit_chance, hit_chance};
pub use roll::{roll, roll_crit_chance, roll_hit_chance};
