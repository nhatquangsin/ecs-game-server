use bevy_ecs::{
    change_detection::ResMut,
    component::Component,
    entity::Entity,
    prelude::{Query, Schedules},
    query::With,
    schedule::{LogLevel, ScheduleBuildSettings},
    system::Commands,
};
use bevy_internal::{
    hierarchy::DespawnRecursiveExt,
    log::{info, warn},
    prelude::{default, Transform},
    tasks::ComputeTaskPool,
};
// use combat_engine::{
//     component::aoi::Game,
//     ecs_skill::component::skill::{SkillComponent, SkillStateComponent},
// };

pub fn ready(mut schedules: ResMut<Schedules>) {
    info!("Server Started");

    let pool = ComputeTaskPool::get();
    let thread_count = pool.thread_num();
    info!("ComputeTaskPool threads: {thread_count:?}");

    for (_, schedule) in schedules.iter_mut() {
        // FOR DEBUGGING CONFLICT/AMBIGIOUS SYSTEMS pairs.
        // TURN ON THIS (with ResMut)

        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Warn,
            ..default()
        });
        for system in schedule.graph().systems() {
            if system.1.is_exclusive() {
                warn!("{:?}", system.1);
            }
        }

        let sys_count = schedule.systems_len();
        if sys_count > 1 {
            info!(
                "ECS {:?} Schedule runs with {sys_count:?} systems",
                schedule.label()
            );
        }
    }
}

// #[allow(dead_code)]
// pub fn entity_stats(
//     games: Query<&Game>,
//     transforms: Query<&Transform>,
//     skills: Query<&SkillComponent>,
//     skill_states: Query<&SkillStateComponent>,
// ) {
//     // let entities_count = world.entities().total_count();
//     // info!("entity_count: {entities_count:?}");

//     let games_entities_count = games.iter().count();
//     info!("room_entities_count: {games_entities_count:?}");

//     let transform_count = transforms.iter().count();
//     info!("transform_count: {transform_count:?}");

//     let skills_count = skills.iter().count();
//     info!("skills_count: {skills_count:?}");

//     let skill_states_count = skill_states.iter().count();
//     info!("skill_states_count: {skill_states_count:?}");
// }
