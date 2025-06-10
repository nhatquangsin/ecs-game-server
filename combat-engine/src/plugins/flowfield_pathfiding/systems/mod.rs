pub mod agent;

use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use bevy_ecs::system::ResMut;
use bevy_internal::math::Vec2;

use crate::plugins::flowfield_pathfiding::{
    components::CostFieldComponent, resources::FlowfieldRes,
};

pub fn init_cost_field(mut res: ResMut<FlowfieldRes>) {
    let file_path = format!("./cost_fields/map.txt");
    let contents = fs::read_to_string(file_path).unwrap();

    let data = serde_json::from_str::<Vec<Vec<u16>>>(&contents).unwrap();

    res.cost_field = CostFieldComponent(data);
}

pub fn init_integration_field(mut res: ResMut<FlowfieldRes>) {
    let file_path = format!("./integration_fields/map.txt");
    let contents = fs::read_to_string(file_path).unwrap();

    let data = serde_json::from_str::<HashMap<String, Vec<Vec<u16>>>>(&contents).unwrap();

    let mut integration_field_map = HashMap::new();
    for (destination, integration_field) in data.iter() {
        let splits = destination.split("_").into_iter().collect::<Vec<&str>>();
        let dest_x = splits[0].parse::<usize>().unwrap();
        let dest_y = splits[1].parse::<usize>().unwrap();

        integration_field_map.insert((dest_x, dest_y), integration_field.clone());
    }

    res.integration_field.integration_field_map = integration_field_map;
}

pub fn calculate_integration_field(
    length: usize,
    width: usize,
    mut cost_map: Vec<Vec<u16>>,
    destination: (usize, usize),
) -> Vec<Vec<u16>> {
    let mut open_set = VecDeque::new();
    open_set.push_front(destination);

    cost_map[destination.0][destination.1] = 0;
    let mut best_cost_map = vec![vec![u16::MAX; width]; length];
    best_cost_map[destination.0][destination.1] = 0;

    while open_set.len() > 0 {
        if let Some((cur_x, cur_y)) = open_set.pop_back() {
            let neighbours = get_neighbours(length, width, cur_x, cur_y);

            'a: for (neighbour_x, neighbour_y) in neighbours {
                if cost_map[neighbour_x][neighbour_y] == u16::MAX {
                    continue 'a;
                }

                let new_cost = cost_map[neighbour_x][neighbour_y] + best_cost_map[cur_x][cur_y];

                if new_cost < best_cost_map[neighbour_x][neighbour_y] {
                    best_cost_map[neighbour_x][neighbour_y] = new_cost;
                    open_set.push_front((neighbour_x, neighbour_y));
                }
            }
        }
    }

    best_cost_map
}

pub fn get_neighbours(
    length: usize,
    width: usize,
    cur_x: usize,
    cur_y: usize,
) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (x, y) in vec![
        (-1, 0),
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ] {
        if (cur_x == 0 && x == -1)
            || (cur_y == 0 && y == -1)
            || (cur_x == length - 1 && x == 1)
            || (cur_y == width - 1 && y == 1)
        {
            continue;
        }

        result.push(((cur_x as i32 + x) as usize, (cur_y as i32 + y) as usize));
    }

    result
}

pub fn init_flow_field(mut res: ResMut<FlowfieldRes>) {
    let file_path = format!("./flow_fields/map.txt",);
    let contents = fs::read_to_string(file_path).unwrap();

    let data = serde_json::from_str::<HashMap<String, Vec<Vec<Vec2>>>>(&contents).unwrap();

    let mut flow_field_map = HashMap::new();
    for (destination, flow_field) in data.iter() {
        let splits = destination.split("_").into_iter().collect::<Vec<&str>>();
        let dest_x = splits[0].parse::<usize>().unwrap();
        let dest_y = splits[1].parse::<usize>().unwrap();

        flow_field_map.insert((dest_x, dest_y), flow_field.clone());
    }

    res.flow_field.flow_field_map = flow_field_map;
}

pub fn calculate_flow_field(
    length: usize,
    width: usize,
    integration_field: &Vec<Vec<u16>>,
) -> Vec<Vec<Vec2>> {
    let mut flow_field = vec![vec![Vec2::ZERO; width]; length];
    for (cur_x, arr) in integration_field.iter().enumerate() {
        'a: for (cur_y, best_cost) in arr.iter().enumerate() {
            if *best_cost == u16::MAX {
                continue 'a;
            }

            let mut best_cost = *best_cost;
            let mut best_neighbour = None;

            let neighbours = get_neighbours(length, width, cur_x, cur_y);
            for (neighbour_x, neighbour_y) in neighbours {
                let neighbour_best_cost = integration_field[neighbour_x][neighbour_y];
                if neighbour_best_cost < best_cost {
                    best_cost = neighbour_best_cost;
                    best_neighbour = Some((neighbour_x, neighbour_y));
                }
            }

            if let Some((x, y)) = best_neighbour {
                flow_field[cur_x][cur_y] = (Vec2::new(x as f32, y as f32)
                    - Vec2::new(cur_x as f32, cur_y as f32))
                .normalize_or_zero();
            }
        }
    }

    flow_field
}

#[cfg(test)]
mod tests {
    use crate::plugins::flowfield_pathfiding::systems::{
        calculate_flow_field, calculate_integration_field,
    };

    #[test]
    fn test_calculate_integration_field() {
        let a = calculate_integration_field(
            4,
            4,
            vec![
                vec![1, 1, 1, 1],
                vec![1, 3, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
            ],
            (3, 3),
        );
        assert_eq!(
            a,
            vec![
                vec![4, 3, 3, 3],
                vec![3, 4, 2, 2],
                vec![3, 2, 1, 1],
                vec![3, 2, 1, 0]
            ]
        );
    }

    #[test]
    fn test_calculate_integration_field_with_obstacles() {
        let a = calculate_integration_field(
            4,
            4,
            vec![
                vec![1, 1, 1, 1],
                vec![1, 3, 1, 1],
                vec![1, u16::MAX, 1, 1],
                vec![1, 1, 1, 1],
            ],
            (3, 3),
        );
        assert_eq!(
            a,
            vec![
                vec![4, 3, 3, 3],
                vec![4, 4, 2, 2],
                vec![3, u16::MAX, 1, 1],
                vec![3, 2, 1, 0]
            ]
        );
    }

    #[test]
    fn test_calculate_flow_field() {
        let a = calculate_flow_field(
            4,
            4,
            &vec![
                vec![4, 3, 3, 3],
                vec![3, 4, 2, 2],
                vec![3, 2, 1, 1],
                vec![3, 2, 1, 0],
            ],
        );
        println!("{:?}", a);
    }

    #[test]
    fn test_calculate_flow_field_with_obstacles() {
        let a = calculate_flow_field(
            4,
            4,
            &vec![
                vec![4, 3, 3, 3],
                vec![4, 4, 2, 2],
                vec![3, u16::MAX, 1, 1],
                vec![3, 2, 1, 0],
            ],
        );
        println!("{:#?}", a);
    }
}
