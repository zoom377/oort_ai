use oort_api::prelude::{
    maths_rs::{approx, prelude::FloatOps},
    *,
};

use crate::f64_extensions::F64Ex;

/*
Equations of motion
These are only accurate when acceleration is constant over the timespan.

v = v0 + at
x = t((v + v0) / 2)
x = v0t + 0.5at^2
v^2 = v0^2 + 2ax

*/

pub fn delta_distance(time: f64, inital_velocity: f64, accel: f64, jerk: f64) -> f64 {
    const ONE_SIXTH: f64 = 1.0 / 6.0;
    time * (inital_velocity + 0.5 * accel * time + 0.00833333333333333 * accel + ONE_SIXTH * jerk * time.powf(2.0))
}

pub fn delta_distance_iterative(time: f64, mut velocity: f64, mut accel: f64, jerk: f64) -> f64 {
    let mut ticks = (time / TICK_LENGTH).round() as i32;
    // debug!("ticks: {}", ticks);
    let mut distance = 0.0;

    while ticks > 0 {
        velocity += accel * TICK_LENGTH;
        distance += velocity * TICK_LENGTH;
        // accel += jerk * TICK_LENGTH;
        ticks -= 1;
    }

    return distance;
}

//Ship frame of reference
pub fn predict_intercept(
    enm_pos: Vec2,
    enm_vel: Vec2,
    enm_acc: Vec2,
    enm_jerk: Vec2,
    blt_spd: f64,
) -> Vec2 {
    let mut iterations = 4;
    let mut intercept = enm_pos;
    let mut ttt = intercept.length() / blt_spd;

    while iterations > 0 {
        intercept = enm_pos
            + vec2(
                delta_distance(ttt, enm_vel.x, enm_acc.x, enm_jerk.x),
                delta_distance(ttt, enm_vel.y, enm_acc.y, enm_jerk.y),
            );
        // intercept = enm_pos
        //     + vec2(
        //         delta_distance_iterative(ttt, enm_vel.x, enm_acc.x, enm_jerk.x),
        //         delta_distance_iterative(ttt, enm_vel.y, enm_acc.y, enm_jerk.y),
        //     );
        ttt = intercept.length() / blt_spd;
        iterations -= 1;
    }

    return intercept;
}

pub fn get_optimal_arrive_velocity(distance: f64, max_accel: f64, final_velocity: f64) -> f64 {
    //v^2 = v0^2 + 2ax
    //v^2 - v0^2 = 2ax
    //-(v0^2) = 2ax - v^2
    //v0^2 = -(2ax - v^2)
    //v0 = sqrt(-(2ax - v^2))
    let vel_sqr = (2.0 * max_accel * distance) - final_velocity.powf(2.0);
    let mut vel = vel_sqr.abs().sqrt();
    vel *= vel_sqr.signum();
    return vel;
}
