pub mod f64_extensions;
pub mod graphing;
pub mod kinematics;
pub mod vec_extensions;

use std::{default, ops::Index};

use f64_extensions::*;
use graphing::Graph;
use kinematics::*;
use oort_api::prelude::{*, maths_rs::vec};
use vec_extensions::*;

const BULLET_SPEED: f64 = 1000.0; // m/s

#[derive(Default)]
pub struct Ship {
    tar_prev_vel: Vec2,
    last_target_heading: f64,
    graph: Graph,
    graph2: Graph,
}

impl Ship {
    pub fn new() -> Ship {
        return Ship {
            graph: Graph {
                position: vec2(-500.0, 0.0),
                size: vec2(1000.0, 400.0),
                ..Default::default()
            },
            graph2: Graph{
                position: vec2(-500.0, -500.0),
                size : vec2(1000.0, 400.0),
                ..Default::default()
            },
            ..Default::default()
        };
    }

    pub fn tick(&mut self) {
        let tar_acc = target_velocity() - self.tar_prev_vel;

        let tar_blt_intercept = target()
            + predict_bullet_intercept(
                target() - position(),
                target_velocity() - velocity(),
                tar_acc,
                BULLET_SPEED,
            );

        let intercept_angle = tar_blt_intercept.angle();
        self.track(intercept_angle);

        draw_square(tar_blt_intercept, 50.0, 0x0000ff);
        self.graph.add(
            f64::sin(current_tick() as f64)
                + f64::sin(current_tick() as f64 / 2.0)
                + f64::sin(current_tick() as f64 / 4.0),
        );
        self.graph.tick();

        self.graph2.add(target_velocity().dot(Vec2::from(heading())));
        self.graph2.tick();
    }

    //Turns ship to track a moving target. Automatically calculates target velocity.
    //Self frame of reference
    fn track(&mut self, target_heading: f64) {
        let angle_delta = angle_diff(heading(), target_heading);

        let critical_deccel_angle =
            get_critical_deccel_angle(angular_velocity(), max_angular_acceleration());
        turn(angle_delta * 30.0);

        let heading_vector = Vec2::rotate(vec2(1.0, 0.0), heading());
        let l1 = heading_vector.rotate(critical_deccel_angle) * 500.0;
        let l2 = heading_vector.rotate(-critical_deccel_angle) * 500.0;

        self.last_target_heading = target_heading;
    }
}
