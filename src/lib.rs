pub mod kinematics;

use oort_api::prelude::*;
use kinematics::kinematics::predict;

const BULLET_SPEED: f64 = 1000.0; // m/s



pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        set_radar_heading(radar_heading() + radar_width());
        if let Some(contact) = scan() {
            accelerate(0.1 * (contact.position - position()));
            fire(0);
            // predict(vel, acc, time)
        }
    }
}
