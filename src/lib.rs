pub mod f64_extensions;
pub mod graphing;
pub mod kinematics;
pub mod vec_extensions;

use f64_extensions::F64Ex;
use graphing::graphing::Graph;
use kinematics::*;
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s

#[derive(Default)]
pub struct Ship {
    tar_prev_vel: Vec2,
    last_target_heading: f64,
    graph1: Graph,
    graph2: Graph,
    graph3: Graph,
    graph4: Graph,
}

impl Ship {
    pub fn new() -> Ship {
        return Ship {
            graph1: Graph {
                title: String::from("ang del"),
                position: vec2(-750.0, 500.0),
                size: vec2(1500.0, 400.0),
                color: 0xff0000,
                debug: true,
                ..Default::default()
            },
            graph2: Graph {
                title: String::from("des vel"),
                position: vec2(-750.0, 0.0),
                size: vec2(1500.0, 400.0),
                timespan: 3.0,
                color: 0xff00ff,
                debug: true,
                ..Default::default()
            },
            graph3: Graph {
                title: String::from("ang vel del"),
                position: vec2(-750.0, -500.0),
                size: vec2(1500.0, 400.0),
                timespan: 3.0,
                color: 0x00ff00,
                debug: true,
                ..Default::default()
            },
            graph4: Graph {
                title: String::from("ang acc"),
                position: vec2(-750.0, -1000.0),
                size: vec2(1500.0, 400.0),
                auto_shrink: false,
                timespan: 3.0,
                color: 0xffff00,
                debug: true,
                ..Default::default()
            },
            ..Default::default()
        };
    }

    pub fn tick(&mut self) {
        let tar_acc = target_velocity() - self.tar_prev_vel;
        let tar_angle = (target() - position()).angle();
        set_radar_heading(tar_angle);

        let tar_blt_intercept = target()
            + predict_bullet_intercept(
                target() - position(),
                target_velocity() - velocity(),
                tar_acc,
                BULLET_SPEED,
            );

        let blt_intercept_angle = tar_blt_intercept.angle();

        draw_square(tar_blt_intercept, 50.0, 0xffff00);
        self.track(blt_intercept_angle);
    }

    //Turns ship to track a moving target. Automatically calculates target velocity.
    //Self frame of reference
    fn track(&mut self, target_heading: f64) {
        let angle_delta = angle_diff(heading(), target_heading);
        let heading_vector = Vec2::rotate(vec2(1.0, 0.0), heading());
        let target_angular_velocity = angle_diff(self.last_target_heading, target_heading);
        
        let desired_velocity = get_optimal_arrive_velocity(angle_delta, max_angular_acceleration(), target_angular_velocity);
        let desired_impulse = desired_velocity - angular_velocity();
        let acc = (desired_velocity - angular_velocity()).signum() * max_angular_acceleration() * 1.0;

        // if desired_impulse.abs() < max_angular_acceleration() {
        //     let t = F64Ex::lerp_inverse(desired_velocity, angular_velocity(), angular_velocity() + max_angular_acceleration());
        //     t.lerp(0.0, )
        // }

        torque(acc);


        self.graph1.add(angle_delta.to_degrees());
        self.graph1.tick();

        self.graph2.add(desired_velocity);
        self.graph2.tick();

        self.graph3.add(desired_velocity - angular_velocity());
        self.graph3.tick();
        
        self.graph4.add(acc);
        self.graph4.tick();

        self.last_target_heading = target_heading;
    }
}
