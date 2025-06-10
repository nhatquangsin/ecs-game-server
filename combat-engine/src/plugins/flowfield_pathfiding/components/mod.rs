use std::collections::HashMap;

use bevy_ecs::prelude::Component;
use bevy_internal::math::Vec2;

#[derive(Component, Default, Clone, Debug)]
pub struct CostFieldComponent(pub Vec<Vec<u16>>);

#[derive(Component, Default, Debug)]
pub struct IntegrationFieldComponent {
    pub integration_field_map: HashMap<(usize, usize), Vec<Vec<u16>>>,
}

#[derive(Component, Default, Debug)]
pub struct FlowFieldComponent {
    pub flow_field_map: HashMap<(usize, usize), Vec<Vec<Vec2>>>,
}
