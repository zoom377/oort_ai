pub mod kinematics;

use kinematics::kinematics::predict;
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        // predict(vel, acc, time)
    }
}
