pub mod f64_extensions;
pub mod graphing;
pub mod kinematics;
pub mod vec_extensions;


use graphing::graphing::Graph;
use kinematics::*;
use oort_api::prelude::{*};

const BULLET_SPEED: f64 = 1000.0; // m/s

#[derive(Default)]
pub struct Ship {
    tar_prev_vel: Vec2,
    last_target_heading: f64,
    graph: Graph,
    graph2: Graph,
}

impl Ship {
    pub fn new() -> Ship {
        return Ship {
            graph: Graph {
                title: String::from("bullet ttt"),
                position: vec2(-500.0, 0.0),
                size: vec2(1000.0, 400.0),
                auto_shrink: false,
                ..Default::default()
            },
            graph2: Graph{
                title: String::from("delta angle"),
                position: vec2(-500.0, -500.0),
                size : vec2(1000.0, 400.0),
                color: 0xff00ff,
                ..Default::default()
            },
            ..Default::default()
        };
    }

    pub fn tick(&mut self) {
        let tar_acc = target_velocity() - self.tar_prev_vel;

        let tar_blt_intercept = target()
            + predict_bullet_intercept(
                target() - position(),
                target_velocity() - velocity(),
                tar_acc,
                BULLET_SPEED,
            );

        let intercept_angle = tar_blt_intercept.angle();
        
        draw_square(tar_blt_intercept, 50.0, 0x0000ff);
        self.graph.add(tar_blt_intercept.length() / BULLET_SPEED);
        self.graph.tick();

        self.track(intercept_angle);

    }
    
    //Turns ship to track a moving target. Automatically calculates target velocity.
    //Self frame of reference
    fn track(&mut self, target_heading: f64) {
        let angle_delta = angle_diff(heading(), target_heading);
        set_radar_heading(target_heading);

        let critical_deccel_angle =
        get_critical_deccel_angle(angular_velocity(), max_angular_acceleration());
        turn(angle_delta * 1000.0);

        self.graph2.add(angle_delta);
        self.graph2.tick();

        let heading_vector = Vec2::rotate(vec2(1.0, 0.0), heading());
        let l1 = heading_vector.rotate(critical_deccel_angle) * 500.0;
        let l2 = heading_vector.rotate(-critical_deccel_angle) * 500.0;

        self.last_target_heading = target_heading;
    }
}
