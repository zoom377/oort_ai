pub mod f64_extensions;
pub mod graphing;
pub mod kinematics;
pub mod vec_extensions;

use std::num::ParseIntError;

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
    last_target_heading: f64,
    graph1: Graph,
    graph2: Graph,
    graph3: Graph,
    graph4: Graph,
}

impl Ship {
    pub fn new() -> Ship {
        return Ship {
            graph1: Graph {
                title: String::from("tar acc"),
                position: vec2(-750.0, 500.0),
                size: vec2(1500.0, 400.0),
                timespan: 3.0,
                color: 0xff0000,
                ..Default::default()
            },
            graph2: Graph {
                title: String::from(""),
                position: vec2(-750.0, 0.0),
                size: vec2(1500.0, 400.0),
                timespan: 3.0,
                color: 0xff00ff,
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
                title: String::from("impulse"),
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
        // let mut x = 0.0;
        // for i in  0..1_000_000{
        //     x += i as f64;
        //     x /= 2.0;
        // }

        // turn(x);

        // return;

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

        // let ship_intercept = predict_intercept(
        //     target_delta,
        //     target_velocity_delta,
        //     target_accel,
        //     target_jerk,
        //     BULLET_SPEED,
        // );

        let bullet_intercept_world = position() + bullet_intercept;
        let bullet_intercept_angle = bullet_intercept.angle();
        let delta_angle = angle_diff(heading(), bullet_intercept_angle);

        // let accel_x = get_optimal_arrive_velocity(bullet_intercept.x, max_forward_acceleration(), velocity().x);
        // let accel_y = get_optimal_arrive_velocity(bullet_intercept.y, max_forward_acceleration(), velocity().y);
        // let accel = vec2(accel_x, accel_y);
        // let accel = bullet_intercept.normalize() * max_forward_acceleration();
        accelerate(bullet_intercept);

        self.track(bullet_intercept_angle);

        if delta_angle.abs() <= TAU / 4.0  && current_tick() > 1{
            activate_ability(Ability::Boost);
            debug!("boost");
        }

        if delta_angle.abs() <= 0.05 {
            fire(0);
        }

        // self.graph1.add(target_accel.length());
        // self.graph1.tick();

        // self.graph2.add();
        // self.graph2.tick();
        draw_diamond(bullet_intercept_world, 50.0, 0xffff00);

        self.target_last_accel = target_accel;
        self.target_last_velocity = target_velocity();
    }

    //Turns ship to track a moving target. Automatically calculates target velocity.
    //Self frame of reference
    fn track(&mut self, target_heading: f64) {
        let angle_delta = angle_diff(heading(), target_heading);
        // let heading_vector = Vec2::rotate(vec2(1.0, 0.0), heading());
        let target_angular_velocity = angle_diff(self.last_target_heading, target_heading);

        let desired_velocity = get_optimal_arrive_velocity(
            angle_delta,
            max_angular_acceleration() * 1.0,
            target_angular_velocity,
        );

        let mut impulse = desired_velocity - angular_velocity();
        impulse = impulse.clamp(-max_angular_acceleration(), max_angular_acceleration());
        torque(impulse / TICK_LENGTH);

        // let mut impulse = (desired_velocity - angular_velocity()).signum() * max_angular_acceleration();
        // torque(impulse);

        self.graph3.add(angle_delta);
        self.graph3.tick();

        self.graph4.add(impulse);
        self.graph4.tick();

        self.last_target_heading = target_heading;
    }
}
