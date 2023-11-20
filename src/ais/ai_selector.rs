use oort_api::prelude::*;

use crate::ais::deflection::Deflection;
use crate::ais::ai::AI;

pub fn get_class_ai(class: Class) -> Box<dyn AI>{
    match class {
        Class::Fighter => Box::new(Deflection::new()),
        Class::Frigate => Box::new(Deflection::new()),
        Class::Cruiser => Box::new(Deflection::new()),
        Class::Asteroid => Box::new(Deflection::new()),
        Class::Target => Box::new(Deflection::new()),
        Class::Missile => Box::new(Deflection::new()),
        Class::Torpedo => Box::new(Deflection::new()),
        Class::Unknown => Box::new(Deflection::new()),
    }
}


//future possibility: match ai against class AND scenario

// pub fn get_ai_for_scenario() -> Box<dyn AI> {
//     return match scenario_name() {
//         "tutorial_guns" => Box::new(Deflection::new()),
//         "tutorial_acceleration" => Box::new(Deflection::new()),
//         "tutorial_acceleration2" => Box::new(Deflection::new()),
//         "tutorial_rotation" => Box::new(Deflection::new()),
//         "tutorial_lead" => Box::new(Deflection::new()),
//         "tutorial_deflection" => Box::new(Deflection::new()),
//         "tutorial_radar" => Box::new(Deflection::new()),
//         "tutorial_search" => Box::new(Deflection::new()),
//         "tutorial_radio" => Box::new(Deflection::new()),
//         "tutorial_missiles" => Box::new(Deflection::new()),
//         "tutorial_squadron" => Box::new(Deflection::new()),
//         "tutorial_frigate" => Box::new(Deflection::new()),
//         "tutorial_cruiser" => Box::new(Deflection::new()),
//         &_ => Box::new(Deflection::new()),
//     };
// }
