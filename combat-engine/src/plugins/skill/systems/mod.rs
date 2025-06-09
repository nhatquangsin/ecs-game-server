use bevy_ecs::{component::Component, entity::Entity};

use crate::data::skill::Skill;

#[derive(Debug, Component)]
pub struct SkillComponent {
    pub skill: &'static Skill,
    pub caster_entity: Entity,
}
