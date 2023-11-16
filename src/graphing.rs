use std::collections::{btree_set::Intersection, VecDeque};

use crate::f64_extensions::F64Ex;
use oort_api::prelude::{maths_rs::lerp, *};

pub struct Datum {
    pub value: f64,
    pub tick: i32,
}

pub struct Graph {
    pub position: Vec2,
    pub size: Vec2,
    pub min: f64,
    pub max: f64,
    pub timespan: f64,
    pub color: u32,
    pub epsilon_squared: f64,
    pub show_labels: bool,
    pub auto_grow: bool,
    pub auto_shrink: bool,
    pub data: VecDeque<Datum>, //Don't set this. Haven't figured out private fields yet.
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            position: vec2(-500.0, -500.0),
            size: vec2(1000.0, 1000.0),
            min: 0.0,
            max: 0.0,
            timespan: 30.0,
            color: 0xff0000,
            epsilon_squared: 100.0,
            show_labels: true,
            auto_grow: true,
            auto_shrink: false,
            data: VecDeque::new(),
        }
    }
}

impl Graph {
    pub fn new() -> Graph {
        return Default::default();
    }

    pub fn add(&mut self, value: f64) {
        let datum = Datum {
            value: value,
            tick: current_tick() as i32,
        };

        if self.data.len() >= 3 {
            let datum_world_position = self.get_datum_world_position(&datum);
            let line_start = self.get_datum_world_position(&self.data[self.data.len() - 3]);
            let line_end = self.get_datum_world_position(&self.data[self.data.len() - 2]);
            debug!("Epsilon squared: {}", self.epsilon_squared);
            let distance_from_line =
                Graph::point_distance_to_line_squared(datum_world_position, line_start, line_end);

            if distance_from_line < self.epsilon_squared {
                let last_index = self.data.len() - 1;
                self.data[last_index] = datum;
                return;
            }
        }

        self.data.push_back(datum);
    }

    pub fn tick(&mut self) {
        // Pop invisible data points
        // {
        //     let mut first_visible_tick = 0;
        //     for pair in self.data.iter().enumerate() {
        //         if (pair.1.tick as f64) >= self.get_start_tick() as f64 {
        //             //Found first visible data point. Everything before must be invisible
        //             first_visible_tick = pair.1.tick;
        //             break;
        //         }
        //     }

        //     let mut last_pop: Option<Datum> = None;
        //     while let Some(front) = self.data.front() {
        //         if front.tick == first_visible_tick {
        //             break;
        //         }
        //         last_pop = self.data.pop_front();
        //     }

        //     if let Some(pop) = last_pop {
        //         self.data.push_front(pop);
        //     }
        // }

        self.remove_hidden_points();
        self.shrink_grow();
        self.draw_axes();
        self.draw_labels();
        self.draw_curve();

        debug!("Data points: {}", self.data.len());
    }

    fn remove_hidden_points(&mut self) {
        let mut last_front: Option<Datum> = None;

        while let Some(front) = self.data.front() {
            if front.tick >= self.get_start_tick() {
                break;
            }
            last_front = self.data.pop_front();
        }

        //Adjust earliest data point as it leaves the graph for smoother appearance
        if let Some(last_front) = last_front {
            self.data.push_front(last_front);
            // let first_point = self.get_datum_world_position(&self.data[0]);
            // let second_point = self.get_datum_world_position(&self.data[1]);

            let t = F64Ex::lerp_inverse(
                self.get_start_tick() as f64,
                self.data[0].tick as f64,
                self.data[1].tick as f64,
            );

            let new_tick = lerp(self.data[0].tick as f64, self.data[1].tick as f64, t);
            let new_val = lerp(self.data[0].value, self.data[1].value, t);

            self.data[0] = Datum {
                tick: new_tick as i32,
                value: new_val,
            };
        }
    }

    fn draw_curve(&self) {
        // let visible_range = self.get_visible_indices_range();
        let mut is_first_point = true;
        let mut last_point: Vec2 = Default::default();
        let mut lines_drawn = 0;

        for pair in self.data.iter().enumerate() {
            let point = self.get_datum_world_position(&pair.1);
            if is_first_point == true {
                is_first_point = false;
            } else {
                draw_line(last_point, point, self.color);
                lines_drawn += 1;
            }
            last_point = point;
        }
        debug!("Lines drawn: {}", lines_drawn);
    }

    fn draw_labels(&self) {
        if self.show_labels {
            draw_text!(
                self.get_datum_world_position(&Datum {
                    value: self.max,
                    tick: self.get_start_tick()
                }),
                self.color,
                "{:.2}",
                self.max
            );
            draw_text!(
                self.get_datum_world_position(&Datum {
                    value: self.min,
                    tick: self.get_start_tick()
                }),
                self.color,
                "{:.2}",
                self.min
            );
        }

        //Draw zero line and label
        if self.min < 0.0 && self.max > 0.0 {
            draw_line(
                self.get_datum_world_position(&Datum {
                    value: 0.0,
                    tick: self.get_start_tick(),
                }),
                self.get_datum_world_position(&Datum {
                    value: 0.0,
                    tick: current_tick() as i32,
                }),
                0xcccccc,
            );
        }
    }

    fn draw_axes(&self) {
        //Draw axes
        draw_line(
            self.get_datum_world_position(&Datum {
                value: self.min,
                tick: self.get_start_tick(),
            }),
            self.get_datum_world_position(&Datum {
                value: self.min,
                tick: current_tick() as i32,
            }),
            0xffffff,
        );
        draw_line(
            self.get_datum_world_position(&Datum {
                value: self.min,
                tick: self.get_start_tick(),
            }),
            self.get_datum_world_position(&Datum {
                value: self.max,
                tick: self.get_start_tick(),
            }),
            0xffffff,
        );
    }

    fn shrink_grow(&mut self) {
        let mut largest = f64::MIN;
        let mut smallest = f64::MAX;
        for datum in &self.data {
            largest = largest.max(datum.value);
            smallest = smallest.min(datum.value);
        }

        if self.auto_shrink {
            self.max = self.max.min(largest);
            self.min = self.min.max(smallest);
        }

        if self.auto_grow {
            self.max = self.max.max(largest);
            self.min = self.min.min(smallest);
        }
    }

    fn get_datum_world_position(&self, datum: &Datum) -> Vec2 {
        let start_tick = self.get_start_tick();
        return vec2(
            (datum.tick as f64).remap(
                start_tick as f64,
                current_tick() as f64,
                self.position.x,
                self.position.x + self.size.x,
            ),
            datum.value.remap(
                self.min,
                self.max,
                self.position.y,
                self.position.y + self.size.y,
            ),
        );
    }

    fn get_visible_indices_range(&self) -> (usize, usize) {
        let start_tick = self.get_start_tick();
        let mut range = (0, self.data.len() - 1);
        while self.data[range.0].tick < start_tick - 1 {
            range.0 += 1;
        }
        return range;
    }

    fn get_start_tick(&self) -> i32 {
        return current_tick() as i32 - (self.timespan / TICK_LENGTH).round() as i32;
    }

    fn point_distance_to_line(p: Vec2, l1: Vec2, l2: Vec2) -> f64 {
        return ((l2.x - l1.x) * (l1.y - p.y) - (l1.x - p.x) * (l2.y - l1.y))
            / f64::sqrt((l2.x - l1.x).powf(2.0) + (l2.y - l1.y).powf(2.0));
    }

    fn point_distance_to_line_squared(p: Vec2, l1: Vec2, l2: Vec2) -> f64 {
        return ((l2.x - l1.x) * (l1.y - p.y) - (l1.x - p.x) * (l2.y - l1.y)).powf(2.0)
            / ((l2.x - l1.x).powf(2.0) + (l2.y - l1.y).powf(2.0));
    }
}
