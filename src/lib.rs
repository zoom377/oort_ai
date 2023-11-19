pub mod f64_extensions;
pub mod graphing;
pub mod kinematics;
pub mod vec_extensions;

use std::{collections::VecDeque, num::ParseIntError};

use f64_extensions::F64Ex;
use graphing::graphing::Graph;
use kinematics::*;
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s

#[derive(Default)]
pub struct Ship {
    target_last_velocity: Vec2,
    target_last_accel: Vec2,
    target_last_jerk: Vec2,
    target_last_heading: f64,
    target_last_angular_vel: f64,
    target_last_angular_accel: f64,
    target_vel_history: VecDeque<Vec2>,
    graph1: Graph,
    graph2: Graph,
    graph3: Graph,
    graph4: Graph,
}

impl Ship {
    pub fn new() -> Ship {
        return Ship {
            graph1: Graph {
                title: String::from("delta"),
                position: vec2(-750.0, 500.0),
                size: vec2(1500.0, 400.0),
                timespan: 3.0,
                color: 0xff0000,
                ..Default::default()
            },
            graph2: Graph {
                title: String::from("vel delta"),
                position: vec2(-750.0, 0.0),
                size: vec2(1500.0, 400.0),
                timespan: 3.0,
                color: 0x00ffff,
                ..Default::default()
            },
            graph3: Graph {
                title: String::from("accel"),
                position: vec2(-750.0, -500.0),
                size: vec2(1500.0, 400.0),
                timespan: 3.0,
                color: 0x00ff00,
                ..Default::default()
            },
            graph4: Graph {
                title: String::from(""),
                position: vec2(-750.0, -1000.0),
                size: vec2(1500.0, 400.0),
                timespan: 3.0,
                color: 0xffff00,
                ..Default::default()
            },
            ..Default::default()
        };
    }

    pub fn tick(&mut self) {
        let target_delta = target() - position();
        let target_velocity_delta = target_velocity() - velocity();
        let target_accel = (target_velocity() - self.target_last_velocity) / TICK_LENGTH;
        let target_jerk = (target_accel - self.target_last_accel) / TICK_LENGTH;
        let bullet_intercept = predict_intercept(
            target_delta,
            target_velocity_delta,
            target_accel,
            target_jerk,
            BULLET_SPEED * 0.97,
        );
        let bullet_intercept_world = position() + bullet_intercept;
        let bullet_intercept_angle = bullet_intercept.angle();
        let delta_angle = angle_diff(heading(), bullet_intercept_angle);
        accelerate(bullet_intercept);
        self.track(bullet_intercept_angle);

        if delta_angle.abs() <= TAU / 4.0 && current_tick() > 1 {
            activate_ability(Ability::Boost);
            debug!("boost");
        }
        if delta_angle.abs() <= 0.05 {
            fire(0);
        }
        draw_diamond(bullet_intercept_world, 50.0, 0xffff00);
        self.target_last_accel = target_accel;
        self.target_last_velocity = target_velocity();
    }

    fn get_target_average_vel(&self) -> Vec2 {
        let mut res = vec2(0.0, 0.0);
        for acc in &self.target_vel_history {
            res += acc;
        }
        res /= self.target_vel_history.len() as f64;
        return res;
    }

    fn get_target_average_accel(&self) -> Vec2 {
        let last_index = self.target_vel_history.len() - 1;
        return (self.target_vel_history[last_index] - self.target_vel_history[0]) / TICK_LENGTH;
    }

    fn get_max_acceleration(&self, direction: Vec2) {}

    //Self frame of reference
    fn intercept(&self, position: Vec2, velocity: Vec2, accel: Vec2, jerk: Vec2) {
        // let ttt = get_ttt(position.length(), velocity.x, accel);
    }

    //Turns ship to track a moving target. Automatically calculates target velocity.
    //Self frame of reference
    fn track(&mut self, target_heading: f64) {
        let angle_delta = angle_diff(heading(), target_heading);
        let target_angular_velocity = angle_diff(self.target_last_heading, target_heading);

        let desired_velocity = get_optimal_arrive_velocity(
            angle_delta,
            max_angular_acceleration() * 1.0,
            target_angular_velocity,
        );
        // let ttt = get_ttt(
        //     angle_delta,
        //     target_angular_velocity,
        //     max_angular_acceleration(),
        //     0.0,
        // );
        let mut impulse = desired_velocity - angular_velocity();
        impulse = impulse.clamp(-max_angular_acceleration(), max_angular_acceleration());
        torque(impulse / TICK_LENGTH);

        self.graph3.add(angle_delta);
        self.graph3.tick();
        // self.graph4.add(ttt);
        // self.graph4.tick();

        self.target_last_heading = target_heading;
    }
}
