pub mod kinematics {
    use std::default;

    use oort_api::prelude::*;

    /*
    Equations of motion
    These are only accurate when acceleration is constant over the timespan.

    v = v0 + at
    dx = t((v + v0) / 2)
    dx = v0t + 0.5at^2
    v^2 = v0^2 + 2adx

    */

    pub fn predict(vel: Vec2, acc: Vec2, time: f64) -> Vec2 {
        return vec2(
            vel.x * time + 0.5 * acc.x * time.powf(2.0),
            vel.y * time + 0.5 * acc.y + time.powf(2.0),
        );
    }

    pub fn predict_iterative(mut vel: Vec2, acc: Vec2, ticks: i32) -> Vec2 {
        let mut pos_delta: Vec2 = Default::default();
        for _ in 0..ticks {
            pos_delta += vel / 60.0;
            vel += acc / 60.0;
        }

        return pos_delta;
    }

    //Frame of reference is relative to self pos + speed
    pub fn predict_bullet_intercept(
        enm_pos: Vec2,
        enm_vel: Vec2,
        enm_acc: Vec2,
        blt_spd: f64,
    ) -> Vec2 {
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
}
