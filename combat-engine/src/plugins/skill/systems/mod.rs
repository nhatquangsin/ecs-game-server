use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::Added,
    system::{Commands, Query},
};
use bevy_transform::components::Transform;

use crate::{
    data::skill::{Skill, SKILL_REGISTRY},
    plugins::skill::components::PlayerCastSkill,
};

pub fn handle_player_cast_skill(
    mut cmds: Commands,
    cast_skill_query: Query<(Entity, &PlayerCastSkill), Added<PlayerCastSkill>>,
    transform_query: Query<(Entity, &Transform)>,
) {
    for (e, event) in cast_skill_query.iter() {
        cmds.entity(e).despawn();
        let Some(skill) = SKILL_REGISTRY.get(&event.skill_id) else {
            continue;
        };

        // Skill cost.
        // if let Ok((mut mp, _)) = system_params.mp_query.get_mut(event.caster_entity) {
        //     if mp.0 < skill.skill_cost {
        //         debug!(
        //             "Not enough mp skill cost, current mp {}, skill cost {}",
        //             mp.0, skill.skill_cost
        //         );
        //         continue;
        //     }

        //     mp.sub_mp(skill.skill_cost);
        // }

        // let Ok((_, parent)) = parent_query.get(event.caster_entity) else {
        //     continue;
        // };

        // let mut skill_cmds = cmds.spawn((
        //     SkillComponent {
        //         skill,
        //         caster_entity: event.caster_entity,
        //         skill_cfg_id: skill.skill_cfg_id,
        //     },
        //     SkillStateComponent(SkillState::Init),
        // ));
        // skill_cmds.set_parent(parent.get());

        // Init interval skill
        // if skill.interval_time > 0. {
        //     skill_cmds.insert(IntervalSkillComp(skill.interval_time));
        // }
    }
}
