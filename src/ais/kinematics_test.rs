use super::ai::AI;
use crate::{graphing::graphing::Graph, kinematics::*};
use oort_api::prelude::*;

#[derive(Default)]
pub struct KinematicsTest {
    prediction_iterative: Vec<Vec2>,
    graph1: Graph,
    initial_accel: Vec2,
    initial_jerk: Vec2,
    accel: Vec2,
}

// const INITIAL_VELOCITY: (f64, f64) = (0.0, 0.0);
// const INITIAL_ACCEL: (f64, f64) = (0.0, 0.0);

impl KinematicsTest {
    pub fn new() -> KinematicsTest {
        KinematicsTest {
            graph1: Graph {
                ..Default::default()
            },
            initial_accel: vec2(max_forward_acceleration(), 0.0),
            // initial_jerk: vec2(-max_forward_acceleration() / 5.0, 0.0),
            initial_jerk: vec2(0.0, 0.0),
            accel: vec2(max_forward_acceleration(), 0.0),
            ..Default::default()
        }
    }
}

impl AI for KinematicsTest {
    fn name(&self) -> String {
        return "KinematicsTest".into();
    }

    fn tick(&mut self) {
        // let accel_vec = vec2(1.0, 0.0) * self.initial_accel;
        // let jerk = -max_forward_acceleration() / 2.0;
        // let jerk = 0.0;

        // if current_tick() == 0 {
        //     self.prediction_iterative.push(position());
        //     for i in 1..150 {
        //         // let predicted_position = vec2(
        //         //     delta_distance_iterative(i, velocity().x, accel_vec.x, jerk),
        //         //     delta_distance_iterative(i, velocity().y, accel_vec.y, 0.0),
        //         // );
        //         let predicted_position = vec2(
        //             delta_distance((i as f64) * TICK_LENGTH, velocity().x, accel_vec.x, jerk),
        //             delta_distance((i as f64) * TICK_LENGTH, velocity().y, accel_vec.y, 0.0),
        //         );
        //         self.prediction_iterative.push(predicted_position);
        //     }
        // }

        // if (current_tick() as usize) < self.prediction_iterative.len() {
        //     let pred_pos = self.prediction_iterative[current_tick() as usize];
        //     draw_diamond(position(), 20.0, 0xff0000);
        //     draw_diamond(pred_pos, 20.0, 0x00ff00);

        //     self.graph1.add(position().x - pred_pos.x);
        //     self.graph1.tick();
        // }

        let predicted_position = vec2(
            delta_distance_2(
                (current_tick() as f64) * TICK_LENGTH,
                0.0,
                self.initial_accel.x,
                self.initial_jerk.x,
            ),
            delta_distance_2(
                (current_tick() as f64) * TICK_LENGTH,
                0.0,
                self.initial_accel.y,
                self.initial_jerk.y,
            ),
        );

        self.graph1.add(position().x - predicted_position.x);
        self.graph1.tick();

        accelerate(self.accel);
        debug!("{}", self.initial_accel);
        self.accel += self.initial_jerk * TICK_LENGTH;
    }
}
