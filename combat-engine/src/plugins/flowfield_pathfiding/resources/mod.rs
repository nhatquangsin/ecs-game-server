use bevy_ecs::system::Resource;

use crate::plugins::flowfield_pathfiding::components::{
    CostFieldComponent, FlowFieldComponent, IntegrationFieldComponent,
};

#[derive(Resource)]
pub struct FlowfieldRes {
    pub cost_field: CostFieldComponent,
    pub integration_field: IntegrationFieldComponent,
    pub flow_field: FlowFieldComponent,
}
