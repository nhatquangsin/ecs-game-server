use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query},
};
use bevy_internal::hierarchy::DespawnRecursiveExt;

#[derive(Component)]
pub struct EntityDespawnLastFrame;

pub fn despawn_entity(
    mut cmds: Commands,
    query_despawn: Query<Entity, With<EntityDespawnLastFrame>>,
) {
    for entity in query_despawn.iter() {
        cmds.entity(entity).despawn_recursive();
    }
}
