use bevy_app::{Plugin, PreStartup, PreUpdate};
use bevy_ecs::schedule::IntoSystemConfigs;

use crate::plugins::flowfield_pathfiding::systems::{
    init_cost_field, init_flow_field, init_integration_field,
};

pub mod components;
pub mod resources;
pub mod systems;

pub struct FlowfieldPlugin;

impl Plugin for FlowfieldPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            PreStartup,
            (init_cost_field, init_integration_field, init_flow_field).chain(),
        );

        // app.add_systems(
        //     PreUpdate,
        //     (
        //         on_handle_start_find_path_event,
        //         on_handle_cancel_movement,
        //         on_player_movement,
        //     ),
        // );
        // app.add_systems(Update, init_agent_movement);
    }
}
