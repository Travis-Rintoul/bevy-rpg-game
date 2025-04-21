use bevy::prelude::Component;

#[allow(dead_code)] // TODO: remove dead_code
#[derive(Component, Default, Clone, Debug)]
pub struct ActorStats {
    pub brawn: i32,
    pub fortune: i32,
    pub awareness: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub charisma: i32,
    pub toughness: i32,
}
