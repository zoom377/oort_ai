pub mod f64_extensions;
pub mod graphing;
pub mod kinematics;
pub mod vec_extensions;

use std::{default, ops::Index};

use graphing::Graph;
use kinematics::*;
use oort_api::prelude::*;
use vec_extensions::*;
use f64_extensions::*;

const BULLET_SPEED: f64 = 1000.0; // m/s

#[derive(Default)]
pub struct Ship {
    tar_prev_vel: Vec2,
    last_target_heading: f64,
    graph: Graph,
}

impl Ship {
    pub fn new() -> Ship {
        return Ship {
            graph: Graph {
                position: vec2(-500.0, -500.0),
                size: vec2(1000.0, 1000.0),
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

        // debug!("{}", (5.0).remap(2.0, 6.0, -1.0, 1.0));

        draw_square(tar_blt_intercept, 50.0, 0x0000ff);
        // self.graph.add(tar_blt_intercept.y);
        self.graph.tick();
    }

    //Turns ship to track a moving target. Automatically calculates target velocity.
    //Self frame of reference
    fn track(&mut self, target_heading: f64) {
        let angle_delta = angle_diff(heading(), target_heading);
        //dx !
        //t ?
        //v0 !
        //v !
        //a !

        //Rearranging to get critical angle from target at which we should start deccelerating
        //v^2 = v0^2 + 2adx
        //v^2 - v0^2 = 2adx
        //(v^2 - v0^2) / 2a = dx
        //(0 - v0^2) / 2a = dx
        //-v0^2 / 2a = dx
        let critical_deccel_angle =
            get_critical_deccel_angle(angular_velocity(), max_angular_acceleration());
        // let mut turn: f64 = 0.0;
        // if angle_delta < 0.0 {
        //     turn = -1.0;
        //     if angle_delta.abs() > critical_deccel_angle {
        //         turn *= -1.0;
        //     }
        // } else {
        //     turn = 1.0;
        //     if angle_delta.abs() <= critical_deccel_angle {
        //         turn *= -1.0;
        //     }
        // }
        // torque(turn * max_angular_acceleration());

        // turn(angle_delta * 30.0);
        turn(angle_delta * 30.0);
        self.graph.add(angle_delta);

        let heading_vector = Vec2::rotate(vec2(1.0, 0.0), heading());
        let l1 = heading_vector.rotate(critical_deccel_angle) * 500.0;
        let l2 = heading_vector.rotate(-critical_deccel_angle) * 500.0;
        // draw_line(position(), position() + l1, 0xffffff);
        // draw_line(position(), position() + l2, 0xffffff);
        // debug!("Tar angle: {}", target_heading);
        // debug!("Crit angle: {}", critical_deccel_angle);
        // debug!("Angular velocity: {}", angular_velocity());

        self.last_target_heading = target_heading;
    }

}
