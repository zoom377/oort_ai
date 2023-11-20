use oort_api::prelude::*;

use crate::ais::deflection::Deflection;
use crate::ais::ai::AI;

pub fn get_ai_for_scenario() -> Box<dyn AI> {
    return match scenario_name() {
        "deflection" => Box::new(Deflection::new()),
        &_ => Box::new(Deflection::new()),
    };
}
