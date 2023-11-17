use oort_api::prelude::{maths_rs::{approx, prelude::FloatOps}, *};

use crate::f64_extensions::F64Ex;

/*
Equations of motion
These are only accurate when acceleration is constant over the timespan.

v = v0 + at
x = t((v + v0) / 2)
x = v0t + 0.5at^2
v^2 = v0^2 + 2ax

*/

// pub fn get_track_torque()

pub fn predict(vel: Vec2, acc: Vec2, time: f64) -> Vec2 {
    return vec2(
        vel.x * time + 0.5 * acc.x * time.powf(2.0),
        vel.y * time + 0.5 * acc.y + time.powf(2.0),
    );
}

pub fn predict_iterative(mut vel: Vec2, acc: Vec2, ticks: i32) -> Vec2 {
    // let mut pos_delta: Vec2 = Default::default();
    let mut pos_delta: Vec2 = Default::default();
    for _ in 0..ticks {
        pos_delta += vel / 60.0;
        vel += acc / 60.0;
    }

    return pos_delta;
}

//Frame of reference is relative to self
pub fn predict_bullet_intercept(enm_pos: Vec2, enm_vel: Vec2, enm_acc: Vec2, blt_spd: f64) -> Vec2 {
    const ITERATIONS: i8 = 5;

    let mut intercept: Vec2 = enm_pos;
    let mut blt_time: f64 = intercept.length() / blt_spd;

    let mut i = 0;
    while i < ITERATIONS {
        intercept = predict(enm_vel, enm_acc, blt_time);
        blt_time = intercept.length() / blt_spd;
        i += 1;
    }

    return intercept;
}

pub fn get_critical_deccel_angle(
    mut angular_velocity: f64,
    angular_accel: f64,
) -> f64 {

    let mut angle = 0.0;
    angular_velocity = angular_velocity.abs();
    
    while !approx(angular_velocity, 0.0, f64::EPSILON) {
        angular_velocity = angular_velocity.move_towards(0.0, angular_accel * TICK_LENGTH);
        angle += angular_velocity;
    }

    return angle;
}


pub fn get_optimal_arrive_velocity(distance: f64, max_accel: f64, final_velocity: f64) -> f64{
    //v^2 = v0^2 + 2ax
    //v^2 - v0^2 = 2ax
    //-(v0^2) = 2ax - v^2
    //v0^2 = -(2ax - v^2)
    //v0 = sqrt(-(2ax - v^2))
    let vel_sqr = (2.0 * max_accel * distance) - final_velocity.powf(2.0);
    debug!("{}", vel_sqr);
    let mut vel = vel_sqr.abs().sqrt();
    vel *= vel_sqr.signum();
    debug!("{}", vel);
    return vel;
}