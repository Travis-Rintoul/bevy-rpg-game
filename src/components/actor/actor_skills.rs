use bevy::prelude::Component;

#[derive(Component, Clone, Debug, Default)]
pub struct ActorSkills
{
    pub gun_prof: i32,
    pub bow_prof: i32,
    pub melee_prof: i32,
    pub unarmed_prof: i32,
    pub thrown_prof: i32,
    pub survival_prof: i32,
    pub crafting_prof: i32,
    pub engineering_prof: i32,
    pub stealth_prof: i32,
    pub athletics_prof: i32,
    pub charisma_prof: i32,
    pub intimidation_prof: i32,
    pub leadership_prof: i32,
    pub perception_prof: i32,
    pub intelligence_prof: i32,
    pub investigation_prof: i32,
    pub medicine_prof: i32,
    pub dodge_prof: i32,
}