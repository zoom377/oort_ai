pub mod f64_extensions;
pub mod graphing;
pub mod kinematics;
pub mod vec_extensions;

use std::{default, ops::Index};

use f64_extensions::*;
use graphing::Graph;
use kinematics::*;
use oort_api::prelude::{*, maths_rs::vec};
use vec_extensions::*;

const BULLET_SPEED: f64 = 1000.0; // m/s



pub struct Ship {
    graph: Graph,
}

impl Ship {
    pub fn new() -> Ship {
        return Ship {
            graph: Graph {
                title: String::from("My Graph"),
                ..Default::default()
            },
        };
    }

    pub fn tick(&mut self) {
        self.graph.add()
    }
}
