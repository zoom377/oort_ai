pub mod kinematics;

// use kinematics::kinematics::predict;
// use crate::kinematics::kinematics::predict_iterative;
use kinematics::kinematics::*;
use oort_api::prelude::*;


const BULLET_SPEED: f64 = 1000.0; // m/s

pub struct Ship {
    tar_prev_vel : Vec2,
    tick_rate: i32
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            tar_prev_vel: Default::default(),
            tick_rate: (1.0 / TICK_LENGTH).round() as i32
        }
    }

    pub fn tick(&mut self) {

        let tar_acc = target_velocity() - self.tar_prev_vel;
        let tar_pred = target() + predict(target_velocity(), tar_acc, 1.0);
        let tar_pred_iter = target() + predict_iterative(target_velocity(), tar_acc, 60);
        
        draw_square(tar_pred, 25.0, 0xff0000);
        draw_square(tar_pred_iter, 30.0, 0x00ff00);
        debug!("TICK RATE: {}", 1.0 / TICK_LENGTH);
    }
}
