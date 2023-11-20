pub mod ais;
pub mod constants;
pub mod f64_extensions;
pub mod graphing;
pub mod kinematics;
pub mod vec_extensions;

// use crate::ais::ai::*;
use ais::ai::AI;
use ais::ai_selector::get_ai_for_scenario;
use oort_api::prelude::*;

pub struct Ship {
    ai: Box<dyn AI>,
}

impl Ship {
    pub fn new() -> Ship {
        let ai = get_ai_for_scenario();
        debug!("Scenario: {}", scenario_name());
        debug!("Active AI: {}", ai.name());
        return Ship { ai: ai };
    }

    pub fn tick(&mut self) {
        self.ai.tick();
    }
}
