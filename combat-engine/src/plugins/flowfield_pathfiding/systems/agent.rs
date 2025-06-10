use std::collections::{HashMap, HashSet};

use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    system::{Commands, Query, Res},
};
use bevy_internal::{math::Vec2, time::Time};
use bevy_transform::components::Transform;

use crate::plugins::flowfield_pathfiding::resources::FlowfieldRes;

// pub fn init_agent_movement(
//     mut cmds: Commands,
//     query_player: Query<(Entity, &Transform, &PlayerComponent), Added<PlayerComponent>>,
// ) {
//     for (entity, transform, player_comp) in query_player.iter() {
//         if player_comp.auto_move {
//             cmds.entity(entity)
//                 .insert(AgentMovementComponent::new(transform.position));
//         }
//     }
// }

// pub fn on_handle_start_find_path_event(
//     mut event_reader: EventReader<AgentStartPathFindingEvent>,
//     mut player_movement_query: Query<(Entity, &mut AgentMovementComponent)>,
// ) {
//     for event in event_reader.read() {
//         let Ok((_, mut agent_movement_comp)) = player_movement_query.get_mut(event.entity) else {
//             continue;
//         };

//         if agent_movement_comp.final_target_entity != Some(event.target_entity) {
//             // Remove agent flow field when agent update destination.
//             agent_movement_comp.flow_field = None;
//             agent_movement_comp.final_target_entity = Some(event.target_entity);
//         }
//     }
// }

// pub fn on_handle_cancel_movement(
//     mut commands: Commands,
//     mut player_query: Query<(Entity, &mut AgentMovementComponent), With<AgentCancelPathFinding>>,
// ) {
//     for (entity, mut agent_movement_comp) in player_query.iter_mut() {
//         agent_movement_comp.on_cancel_movement();
//         commands.entity(entity).remove::<AgentCancelPathFinding>();
//     }
// }

// pub fn on_player_movement(
//     res: Res<FlowfieldRes>,
//     time: Res<Time>,
//     mut agent_query: Query<(Entity, &Transform, &Collider)>,
//     mut agent_movement_query: Query<&mut AgentMovementComponent>,
//     room_query: Query<(Entity, &RoomComponent)>,
//     mut agent_movement_event_writer: EventWriter<AgentMovementEvent>,
// ) {
//     let dt_time = time.delta().as_secs_f32();
//     let mut position_in_rooms: HashMap<Entity, HashMap<Entity, (Vec2, Collider)>> = HashMap::new();
//     let mut occupied_positions: HashSet<(usize, usize)> = HashSet::new();
//     for (entity, parent, transform, collider) in agent_query.iter_mut() {
//         if let Some(positions) = position_in_rooms.get_mut(&parent.get()) {
//             positions.insert(entity, (transform.position, *collider));
//         } else {
//             let mut positions = HashMap::new();
//             positions.insert(entity, (transform.position, *collider));
//             position_in_rooms.insert(parent.get(), positions);
//         }

//         if let Ok(mut agent_movement_comp) = agent_movement_query.get_mut(entity) {
//             agent_movement_comp.timer.tick(time.delta());
//             if agent_movement_comp.check_no_move(transform.position) {
//                 update_occupied_position(&mut occupied_positions, transform.position);
//             }
//         }
//     }

//     for (entity, parent, transform, _) in agent_query.iter_mut() {
//         let Ok((_, room_comp)) = room_query.get(parent.get()) else {
//             continue;
//         };
//         let Some(map_res) = game_res.maps.get(&room_comp.map_code) else {
//             continue;
//         };
//         let Ok(mut agent_movement_comp) = agent_movement_query.get_mut(entity) else {
//             continue;
//         };

//         // Get final destination.
//         let (final_dest_x, final_dest_y) =
//             if let Some(final_position) = agent_movement_comp.final_position {
//                 (
//                     (final_position.x - room_comp.position.x) as usize,
//                     (final_position.y - room_comp.position.y) as usize,
//                 )
//             } else if let Some(final_target_entity) = agent_movement_comp.final_target_entity {
//                 let Some(positions) = position_in_rooms.get(&parent.get()) else {
//                     continue;
//                 };
//                 let Some((target_position, _)) = positions.get(&final_target_entity) else {
//                     continue;
//                 };

//                 (
//                     (target_position.x - room_comp.position.x) as usize,
//                     (target_position.y - room_comp.position.y) as usize,
//                 )
//             } else {
//                 continue;
//             };

//         let (left_bound, right_bound, flow_field) =
//             if let Some((left_bound, right_bound, flow_field)) = &agent_movement_comp.flow_field {
//                 (*left_bound as f32, *right_bound as f32, flow_field)
//             } else if let Some(flow_field) = map_res
//                 .flow_field
//                 .flow_field_map
//                 .get(&(final_dest_x, final_dest_y))
//             {
//                 (room_comp.position.x, room_comp.position.y, flow_field)
//             } else {
//                 continue;
//             };

//         // Get direction.
//         let direction = if agent_movement_comp.current_destination.is_some() {
//             agent_movement_comp.direction
//         } else {
//             agent_movement_comp.direction = flow_field
//                 [(transform.position.x - left_bound) as usize]
//                 [(transform.position.y - right_bound) as usize];
//             agent_movement_comp.current_destination = Some(Vec2::new(
//                 (transform.position.x + agent_movement_comp.direction.x) as usize as f32 + 0.5,
//                 (transform.position.y + agent_movement_comp.direction.y) as usize as f32 + 0.5,
//             ));

//             agent_movement_comp.direction
//         };

//         // Calculate avoidance vector and update new position.
//         let Some(positions) = position_in_rooms.get(&parent.get()) else {
//             continue;
//         };

//         agent_movement_event_writer.send(AgentMovementEvent {
//             agent_entity: entity,
//             dt_time,
//             direction,
//         });

//         // Re-calculate new direction
//         let Some(target) = agent_movement_comp.current_destination else {
//             continue;
//         };
//         let dist_x = (transform.position.x - target.x).abs();
//         let dist_y = (transform.position.y - target.y).abs();
//         if dist_x < 0.1 && dist_y < 0.1 || transform.position.distance(target) > 1. {
//             agent_movement_comp.direction = Vec2::ZERO;
//             agent_movement_comp.current_destination = None;
//         }

//         // Handle movement stuck
//         handle_movement_stuck(
//             &transform,
//             &mut agent_movement_comp,
//             positions,
//             room_comp,
//             &occupied_positions,
//             &map_res,
//         );

//         // TODO: Check if agent move to final destination, reset agent movement.
//         // if let Some(final_position) = agent_movement_comp.final_position {
//         //     if transform.position.distance(final_position) < 0.1 {
//         //         agent_movement_comp.reset();
//         //     }
//         // }
//         //
//         // if let Some(final_target_entity) = agent_movement_comp.final_target_entity {
//         //     // TODO: optimize this
//         //     let Some(target_position) = positions.get(&final_target_entity) else {
//         //         continue;
//         //     };
//         //     if transform.position.distance(*target_position) < 1. {
//         //         agent_movement_comp.reset();
//         //     }
//         // }
//     }

//     for (entity, _, transform, _) in agent_query.iter_mut() {
//         if let Ok(mut agent_movement_comp) = agent_movement_query.get_mut(entity) {
//             // Update agent last position.
//             agent_movement_comp.update_last_position(transform.position);
//         }
//     }
// }

fn update_occupied_position(occupied_positions: &mut HashSet<(usize, usize)>, position: Vec2) {
    occupied_positions.insert((position.x as usize, position.y as usize));
    let diff_x = position.x - ((position.x as usize) as f32);
    let diff_y = position.y - ((position.y as usize) as f32);

    if diff_x < 0.3 {
        occupied_positions.insert((position.x as usize - 1, position.y as usize));
    }

    if diff_x > 0.7 {
        occupied_positions.insert((position.x as usize + 1, position.y as usize));
    }

    if diff_y < 0.3 {
        occupied_positions.insert((position.x as usize, position.y as usize - 1));
    }

    if diff_y > 0.7 {
        occupied_positions.insert((position.x as usize, position.y as usize + 1));
    }
}
