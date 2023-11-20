use std::{any::type_name, collections::VecDeque};

// use crate::{
//     ais::ai::*,
//     graphing::graphing::*,
//     kinematics::*,
//     constants::*
// };

// use crate::ais::ai::*;
use super::ai::AI;
use crate::constants::*;
use crate::graphing::graphing::*;
use crate::kinematics::*;
use oort_api::prelude::*;

#[derive(Default)]
pub struct Deflection {
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
    graph5: Graph,
}

impl Deflection {
    pub fn new() -> Deflection {
        return Deflection {
            graph1: Graph {
                title: String::from("ang delta"),
                position: vec2(-750.0, 500.0),
                size: vec2(1500.0, 400.0),
                max: f64::from(1.0).to_radians(),
                min: f64::from(-1.0).to_radians(),
                auto_grow: false,
                auto_shrink: false,
                timespan: 1.0,
                color: 0xff0000,
                ..Default::default()
            },
            graph2: Graph {
                title: String::from("ang vel delta"),
                position: vec2(-750.0, 0.0),
                size: vec2(1500.0, 400.0),
                timespan: 1.0,
                color: 0x00ffff,
                ..Default::default()
            },
            graph3: Graph {
                title: String::from("ang accel"),
                position: vec2(-750.0, -500.0),
                size: vec2(1500.0, 400.0),
                timespan: 1.0,
                color: 0x00ff00,
                ..Default::default()
            },
            graph4: Graph {
                title: String::from("opt vel"),
                position: vec2(-750.0, -1000.0),
                size: vec2(1500.0, 400.0),
                timespan: 1.0,
                color: 0xff8800,
                ..Default::default()
            },
            graph5: Graph {
                title: String::from(""),
                position: vec2(-750.0, -1000.0),
                size: vec2(1500.0, 400.0),
                timespan: 1.0,
                color: 0xff00ff,
                ..Default::default()
            },
            ..Default::default()
        };
    }

    //Turns ship to track a moving target. Automatically calculates target velocity.
    //Self frame of reference
    fn track(&mut self, target_heading: f64) {
        let angle_delta = angle_diff(heading(), target_heading);
        let target_angular_velocity =
            angle_diff(self.target_last_heading, target_heading) / TICK_LENGTH;

        // let desired_velocity = get_optimal_arrive_velocity(
        //     angle_delta,
        //     max_angular_acceleration(),
        //     target_angular_velocity,
        // ) * 0.9055;
        let desired_velocity = get_optimal_arrive_velocity(
            angle_delta,
            max_angular_acceleration(),
            target_angular_velocity,
        );

        let max_accel = max_angular_acceleration() * -angle_delta.signum();
        // let max_accel = max_angular_acceleration();

        let mut accel = (desired_velocity - angular_velocity()) / TICK_LENGTH;
        accel = accel.clamp(-max_angular_acceleration(), max_angular_acceleration());
        torque(accel);

        let ttt = get_ttt(
            angle_delta,
            target_angular_velocity - angular_velocity(),
            max_accel,
        );

        let ttt2 = get_ttt_2(
            angle_delta,
            target_angular_velocity - angular_velocity(),
            max_accel,
        );

        let opt_vel = get_optimal_arrival_velocity_v3(angle_delta, ttt, max_angular_acceleration());

        debug!("accel: {}", accel);
        debug!("max accel: {}", max_accel);

        self.graph1.add(f64::NAN);
        self.graph1.tick();

        self.graph2.add(accel);
        self.graph2.tick();

        self.graph3.add(accel);
        self.graph3.tick();

        self.graph4.add(opt_vel);
        self.graph4.tick();

        // self.graph5.add();
        // self.graph5.tick();

        self.target_last_heading = target_heading;
    }
}

// #[derive(New)]
impl AI for Deflection {
    fn tick(&mut self) {
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
            BULLET_SPEED,
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

        // if angle_diff(heading(), ship_intercept.angle()).abs() <= TAU / 5.0 && current_tick() > 2 {
        if angle_diff(heading(), ship_intercept.angle()).abs() <= TAU / 5.0 && current_tick() > 2 {
            activate_ability(Ability::Boost);
        } else {
            deactivate_ability(Ability::Boost);
        }

        let fire_angle_threshold = TAU * 1.65 / bullet_intercept.length();
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
        draw_diamond(position() + ship_intercept, 50.0, 0x0000ff);
        self.target_last_accel = target_accel;
        self.target_last_velocity = target_velocity();
    }

    fn name(&self) -> String {
        return type_name::<Deflection>().into();
    }
}
