use std::time::Duration;

use bevy_ecs::{entity::Entity, prelude::Component};
use bevy_internal::{
    math::Vec2,
    time::{Timer, TimerMode},
};

#[derive(Component, Clone, Debug)]
pub struct AgentMovementComponent {
    pub direction: Vec2,
    pub current_destination: Option<Vec2>,
    pub final_position: Option<Vec2>,
    pub final_target_entity: Option<Entity>,
    pub last_position: Vec2,
    pub stuck_count: u8,
    pub no_move_count: u8,
    pub flow_field: Option<(usize, usize, Vec<Vec<Vec2>>)>,
    pub timer: Timer,
}

const AGENT_TIMER_DURATION: Duration = Duration::from_millis(250);

impl AgentMovementComponent {
    pub fn new(position: Vec2) -> Self {
        Self {
            direction: Vec2::ZERO,
            current_destination: None,
            final_position: None,
            final_target_entity: None,
            last_position: position,
            stuck_count: 0,
            no_move_count: 0,
            flow_field: None,
            timer: Timer::new(AGENT_TIMER_DURATION, TimerMode::Repeating),
        }
    }
}

impl AgentMovementComponent {
    pub fn on_cancel_movement(&mut self) {
        self.final_position = None;
        self.final_target_entity = None;
    }

    pub fn update_last_position(&mut self, position: Vec2) {
        if self.timer.just_finished() {
            self.last_position = position;
        }
    }

    pub fn check_stuck(&mut self, position: Vec2) -> bool {
        if self.final_target_entity.is_none() && self.final_position.is_none() {
            return false;
        }

        if !self.timer.just_finished() {
            return false;
        }

        if position.distance(self.last_position) > 0.1 {
            return false;
        }

        self.stuck_count = (self.stuck_count + 1).min(10);
        self.stuck_count > 2
    }

    pub fn check_no_move(&mut self, position: Vec2) -> bool {
        if !self.timer.finished() {
            return false;
        }

        if position.distance(self.last_position) > 0.5 {
            self.no_move_count = 0;
        }

        self.no_move_count = (self.no_move_count + 1).min(10);
        self.no_move_count > 1
    }
}
