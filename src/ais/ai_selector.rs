use oort_api::prelude::*;

use super::{ai::AI, deflection::deflection::Deflection};

pub fn get_ai_for_scenario() -> &'static dyn AI {
    return match scenario_name() {
        "deflection" => &Deflection {},
        &_ => &Deflection {},
    };
}
