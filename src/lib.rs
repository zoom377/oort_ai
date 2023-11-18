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

// const CUSTOM_SHAPE: Vec<Vec2> = vec![vec2(0.0, 0.0)];

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
                title: String::from("tar avg vel"),
                position: vec2(-750.0, 500.0),
                size: vec2(1500.0, 400.0),
                timespan: 3.0,
                color: 0xff0000,
                ..Default::default()
            },
            graph2: Graph {
                title: String::from("tar avg accel"),
                position: vec2(-750.0, 0.0),
                size: vec2(1500.0, 400.0),
                timespan: 3.0,
                color: 0x00ffff,
                ..Default::default()
            },
            graph3: Graph {
                title: String::from("ang del"),
                position: vec2(-750.0, -500.0),
                size: vec2(1500.0, 400.0),
                timespan: 3.0,
                color: 0x00ff00,
                ..Default::default()
            },
            graph4: Graph {
                title: String::from("ang accel"),
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

        self.target_vel_history.push_back(target_velocity_delta);
        if self.target_vel_history.len() > 2 {
            self.target_vel_history.pop_front();
        }
        debug!("len: {}", self.target_vel_history.len());

        let target_accel = self.get_target_average_accel();
        // let new_accel = (target_velocity() - self.target_last_velocity) / TICK_LENGTH;
        // let target_accel = vec2(
        //     F64Ex::lerp(0.5, self.target_last_accel.x, new_accel.x),
        //     F64Ex::lerp(0.5, self.target_last_accel.y, new_accel.y),
        // );

        let target_jerk = (target_accel - self.target_last_accel) / TICK_LENGTH;
        // let new_jerk = (target_accel - self.target_last_accel) / TICK_LENGTH;
        // let target_jerk = vec2(
        //     F64Ex::lerp(0.5, self.target_last_jerk.x, new_jerk.x),
        //     F64Ex::lerp(0.5, self.target_last_jerk.y, new_jerk.y),
        // );

        let bullet_intercept = predict_intercept(
            target_delta,
            self.get_target_average_vel(),
            target_accel,
            // vec2(0.0, 0.0),
            target_jerk,
            BULLET_SPEED,
        );

        // let target_heading = (target() - position()).angle();

        //velocity = change in position over change in time
        let target_angular_vel =
            angle_diff(bullet_intercept.angle(), self.target_last_heading) / TICK_LENGTH;

        let target_angular_accel =
            (target_angular_vel - self.target_last_angular_vel) / TICK_LENGTH;

        let target_angular_jerk =
            (target_angular_accel - self.target_last_angular_accel) / TICK_LENGTH;

        self.track(bullet_intercept.angle(), target_angular_vel);
        accelerate(bullet_intercept.normalize() * max_forward_acceleration());

        if angle_diff(heading(), (target() - position()).angle()) <= TAU / 4.0 {
            activate_ability(Ability::Boost);
        }

        let fire_angle_threshold = TAU * 2.0 / bullet_intercept.length();
        let target_delta_angle = angle_diff(heading(), bullet_intercept.angle());
        if target_delta_angle.abs() <= fire_angle_threshold {
            fire(0);
        }

        // self.intercept(
        //     position() + bullet_intercept,
        //     target_velocity_delta,
        //     target_accel,
        //     target_jerk,
        // );
        //DEBUG
        {
            self.graph1.add(self.get_target_average_vel().length());
            self.graph1.tick();

            self.graph2.add(self.get_target_average_accel().length());
            self.graph2.tick();

            self.graph3
                .add(angle_diff(heading(), bullet_intercept.angle()));
            self.graph3.tick();

            draw_line(
                position(),
                position() + target_delta.rotate(fire_angle_threshold),
                0xff0000,
            );
            draw_line(
                position(),
                position() + target_delta.rotate(-fire_angle_threshold),
                0xff0000,
            );
            draw_diamond(position() + bullet_intercept, 50.0, 0xffff00);
            draw_line(vec2(0.0, 0.0), vec2(0.0, 0.0), 0xff0000);
        }

        //Record state for next frame
        // self.target_last_velocity = target_velocity();
        self.target_last_accel = target_accel;
        self.target_last_jerk = target_jerk;

        self.target_last_heading = bullet_intercept.angle();
        self.target_last_angular_vel = target_angular_vel;
        self.target_last_angular_accel = target_angular_accel;
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
        return self.target_vel_history[last_index] - self.target_vel_history[0];
    }

    fn get_max_acceleration(&self, direction: Vec2) {}

    //Self frame of reference
    fn intercept(&mut self, position: Vec2, velocity: Vec2, accel: Vec2, jerk: Vec2) {
        
    }

    //Turns ship to track a moving target. Automatically calculates target velocity.
    //Self frame of reference
    fn track(&mut self, target_heading: f64, target_angular_velocity: f64) {
        let angle_delta = angle_diff(heading(), target_heading);

        let optimal_angular_velocity = get_optimal_arrive_velocity_2(
            angle_delta,
            target_angular_velocity,
            max_angular_acceleration(),
            0.0,
        );

        let accel = (optimal_angular_velocity - angular_velocity())
            .clamp(-max_angular_acceleration(), max_angular_acceleration());

        torque(accel / TICK_LENGTH);

        self.graph4.add(accel);
        self.graph4.tick();
    }
}
