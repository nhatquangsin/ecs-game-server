use bevy_ecs::{component::Component, entity::Entity};

use crate::data::skill::Skill;

#[derive(Debug, Component)]
pub struct SkillComponent {
    pub skill: &'static Skill,
    pub caster_entity: Entity,
}

#[derive(Component, Debug)]
#[component(storage = "SparseSet")]
pub struct PlayerCastSkill {
    pub caster_entity: Entity,
    pub skill_id: String,
}
