mod data;
mod plugins;
mod shared_systems;

use bevy_app::{Last, Plugin};

use crate::shared_systems::despawn_entity;

#[macro_use]
extern crate strum_macros;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(Last, despawn_entity);
    }
}
