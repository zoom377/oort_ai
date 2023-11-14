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
        let pos_delta: Vec2 = Default::default();
        for _ in 0..100 {
            
        }

        return Default::default();
    }
}
