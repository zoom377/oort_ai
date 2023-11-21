use oort_api::prelude::*;

use crate::f64_extensions::F64Ex;

//Good
pub fn delta_distance(time: f64, inital_velocity: f64, accel: f64, jerk: f64) -> f64 {
    const ONE_SIXTH: f64 = 1.0 / 6.0;
    time * (inital_velocity
        + 0.5 * accel * time
        + 0.00833333333333333 * accel
        + ONE_SIXTH * jerk * time.powf(2.0))
}

pub fn delta_distance_2(time:f64, initial_velocity: f64, accel: f64, jerk: f64) -> f64{
    const ONE_SIXTH: f64 = 1.0 / 6.0;
    time * (accel * (0.5 * time + 0.00833333) + initial_velocity)
}

// pub fn delta_distance_3(time:f64, initial_velocity: f64, accel: f64, jerk: f64) -> f64{
//     const ONE_SIXTH: f64 = 1.0 / 6.0;
//     //dx = vt
//     //dv = at
//     //da = jt
//     //x = 0.5at^2 + v0t
//     //x = 0.5at^2 + v0t + 0.00833at
// }

//authoritative function, should work as game physics simulation does
pub fn delta_distance_iterative(
    mut ticks: i32,
    mut velocity: f64,
    mut accel: f64,
    jerk: f64,
) -> f64 {
    let mut distance = 0.0;

    while ticks > 0 {
        velocity += accel * TICK_LENGTH;
        distance += velocity * TICK_LENGTH;
        accel += jerk * TICK_LENGTH;
        ticks -= 1;
    }

    return distance;
}

//Good
pub fn predict_intercept(
    enm_pos: Vec2,
    enm_vel: Vec2,
    enm_acc: Vec2,
    enm_jerk: Vec2,
    spd: f64,
) -> Vec2 {
    let mut iterations = 4;
    let mut intercept = enm_pos;
    let mut ttt = intercept.length() / spd;

    while iterations > 0 {
        intercept = enm_pos
            + vec2(
                delta_distance(ttt, enm_vel.x, enm_acc.x, enm_jerk.x),
                delta_distance(ttt, enm_vel.y, enm_acc.y, enm_jerk.y),
            );
        ttt = intercept.length() / spd;
        iterations -= 1;
    }

    return intercept;
}

pub fn get_ttt(distance: f64, velocity: f64, accel: f64) -> f64 {
    //displacement = time * (inital_velocity + 0.5 * accel * time + 0.00833333333333333 * accel + ONE_SIXTH * jerk * time.powf(2.0))
    let squared = (accel.powf(2.0)
        + 240.0 * accel * velocity
        + 28_800.0 * accel * distance
        + 14_400.0 * velocity.powf(2.0));

    let squared_abs = squared.abs();

    let t = (squared_abs.sqrt() - accel - 120.0 * velocity) / 120.0;

    return t * squared.signum();
}

pub fn get_ttt_2(distance: f64, initial_velocity: f64, accel: f64) -> f64 {
    //displacement = time * (inital_velocity + 0.5 * accel * time + 0.00833333333333333 * accel + ONE_SIXTH * jerk * time.powf(2.0))

    let t = ((accel.powf(2.0)
        + 240.0 * accel * (120.0 * distance + initial_velocity)
        + 14_400.0 * initial_velocity.powf(2.0))
    .sqrt()
        - accel
        - 120.0 * initial_velocity)
        / 120.0
        * accel;

    return t;
}

pub fn get_optimal_arrival_velocity_v3(distance: f64, time: f64, max_accel: f64) -> f64 {
    let res = distance / time - (1.0 / 120.0) * max_accel * (60.0 * time + 1.0);
    return res;
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

// pub fn get_optimal_arrive_velocity_2(distance: f64, velocity: f64, accel: f64, jerk: f64) -> f64 {
//     //displacement = time * (inital_velocity + 0.5 * accel * time + 0.00833333333333333 * accel + ONE_SIXTH * jerk * time.powf(2.0))
//     let ttt = get_ttt(distance, velocity, accel);
//     let optimal_velocity = -(1.0 / 120.0) * accel * (60.0 * ttt + 1.0) + (distance / ttt)
//         - (jerk * ttt.powf(2.0)) / 6.0;

//     debug!("ttt: {}", ttt);
//     debug!("optimal vel: {}", optimal_velocity);
//     return -optimal_velocity / 3.0;
// }
