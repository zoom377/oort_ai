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
    bullets_fired: i32,
    graph1: Graph,
    graph2: Graph,
    graph3: Graph,
    graph4: Graph,
}

impl Ship {
    pub fn new() -> Ship {
        return Ship {
            graph1: Graph {
                title: String::from("ang delta"),
                position: vec2(-750.0, 500.0),
                size: vec2(1500.0, 400.0),
                color: 0xff0000,
                ..Default::default()
            },
            graph2: Graph {
                title: String::from("ang vel delta"),
                position: vec2(-750.0, 0.0),
                size: vec2(1500.0, 400.0),
                timespan: 2.0,
                color: 0x00ffff,
                ..Default::default()
            },
            graph3: Graph {
                title: String::from("ang accel"),
                position: vec2(-750.0, -500.0),
                size: vec2(1500.0, 400.0),
                timespan: 2.0,
                color: 0x00ff00,
                ..Default::default()
            },
            graph4: Graph {
                title: String::from(""),
                position: vec2(-750.0, -1000.0),
                size: vec2(1500.0, 400.0),
                timespan: 2.0,
                color: 0xffff00,
                ..Default::default()
            },
            ..Default::default()
        };
    }

    pub fn tick(&mut self) {
        debug!("tick: {}", current_tick());
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

        let bullet_intercept_angle = bullet_intercept.angle();
        let delta_angle = angle_diff(heading(), bullet_intercept_angle);

        let ship_intercept = predict_intercept(
            target_delta,
            target_velocity_delta,
            target_accel,
            target_jerk,
            350.0,
        );

        accelerate(ship_intercept.normalize() * max_forward_acceleration());
        self.track(bullet_intercept_angle);

        if angle_diff(heading(), ship_intercept.angle()).abs() <= TAU / 5.0 && current_tick() > 2 {
            activate_ability(Ability::Boost);
        } else {
            deactivate_ability(Ability::Boost);
        }

        let fire_angle_threshold = TAU * 1.75 / bullet_intercept.length();
        if delta_angle.abs() <= fire_angle_threshold {
            fire(0);
            self.bullets_fired += 1;
        }
        debug!("fired: {}", self.bullets_fired);

        draw_line(
            position(),
            position() + vec2(1.0, 0.0).rotate(heading()) * bullet_intercept.length(),
            0x00ff00,
        );
        draw_line(
            position(),
            position() + bullet_intercept.rotate(-fire_angle_threshold),
            0xff0000,
        );
        draw_line(
            position(),
            position() + bullet_intercept.rotate(fire_angle_threshold),
            0xff0000,
        );
        draw_diamond(position() + bullet_intercept, 50.0, 0xff0000);
        draw_diamond(position() + ship_intercept, 50.0, 0x00ff00);
        self.target_last_accel = target_accel;
        self.target_last_velocity = target_velocity();
    }

    // fn intercept(&self, pos: Vec2, vel: Vec2, )

    //Turns ship to track a moving target. Automatically calculates target velocity.
    //Self frame of reference
    fn track(&mut self, target_heading: f64) {
        let angle_delta = angle_diff(heading(), target_heading);
        let target_angular_velocity = angle_diff(self.target_last_heading, target_heading);

        let desired_velocity = get_optimal_arrive_velocity(
            angle_delta,
            max_angular_acceleration(),// * 0.95,
            target_angular_velocity,
        ) * 0.9055; 
        let mut accel = (desired_velocity - angular_velocity()) / TICK_LENGTH;
        accel = accel.clamp(-max_angular_acceleration(), max_angular_acceleration());
        torque(accel);

        self.graph1.add(angle_delta);
        self.graph1.tick();

        self.graph2.add(desired_velocity - angular_velocity());
        self.graph2.tick();

        self.graph3.add(accel);
        self.graph3.tick();

        self.target_last_heading = target_heading;
    }
}
