use bevy::prelude::Component;


#[derive(Debug, Clone)]
pub enum WeaponClass {
    Gun,
    Bow,
    Melee,
    Unarmed,
    Thrown,
}

#[derive(Component, Clone, Debug)]
pub struct Weapon {
    pub name: String,
    pub class: WeaponClass,
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon {
            name: String::from("Fists"),
            class: WeaponClass::Unarmed,
        }
    }
}