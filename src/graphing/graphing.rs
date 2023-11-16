use oort_api::prelude::*;
use std::collections::VecDeque;

pub trait F64Ex {
    fn move_towards(self, target: f64, max_delta: f64) -> f64;
    fn lerp(self, min: f64, max: f64) -> f64;
    fn lerp_inverse(self, min: f64, max: f64) -> f64;
    fn remap(self, min: f64, max: f64, new_min: f64, new_max: f64) -> f64;
}
impl F64Ex for f64 {
    fn move_towards(self, target: f64, max_delta: f64) -> f64 {
        return self + (target - self).clamp(-max_delta, max_delta);
    }
    fn lerp(self, min: f64, max: f64) -> f64 {
        return self * (max - min) + min;
    }
    fn lerp_inverse(self, min: f64, max: f64) -> f64 {
        return (self - min) / (max - min);
    }
    fn remap(self, min: f64, max: f64, new_min: f64, new_max: f64) -> f64 {
        let t = self.lerp_inverse(min, max);
        let res = t.lerp(new_min, new_max);
        return res;
    }
}

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
    pub epsilon_squared: f64,
    pub color: u32,
    pub title: String,
    pub show_labels: bool,
    pub auto_grow: bool,
    pub auto_shrink: bool,
    pub debug: bool,
    //Don't set this. Haven't figured out private fields yet.
    pub data: VecDeque<Datum>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            position: vec2(-500.0, -500.0),
            size: vec2(1000.0, 1000.0),
            min: 0.0,
            max: 0.0,
            timespan: 3.0,
            epsilon_squared: 20.0,
            color: 0xff0000,
            title: String::new(),
            show_labels: true,
            auto_grow: true,
            auto_shrink: true,
            debug: false,
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
            let distance_from_line =
                Graph::point_distance_to_line_squared(datum_world_position, line_start, line_end);

            //If change from last point is insignificant, move current point rather than adding a new one.
            if distance_from_line < self.epsilon_squared {
                let last_index = self.data.len() - 1;
                self.data[last_index] = datum;
                return;
            }
        }

        self.data.push_back(datum);
    }

    pub fn tick(&mut self) {
        self.remove_hidden_points();
        self.shrink_grow();
        self.draw_axes();
        self.draw_labels();
        let lines_drawn = self.draw_curve();

        if self.debug {
            let max_possible_lines = current_tick() as i32 - self.get_start_tick() + 1;
            debug!("Lines: {}/{}", lines_drawn, max_possible_lines);
        }
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

            let t = F64Ex::lerp_inverse(
                self.get_start_tick() as f64,
                self.data[0].tick as f64,
                self.data[1].tick as f64,
            );

            let new_tick = t.lerp(self.data[0].tick as f64, self.data[1].tick as f64);
            let new_val = t.lerp(self.data[0].value, self.data[1].value);

            self.data[0] = Datum {
                tick: new_tick as i32,
                value: new_val,
            };
        }
    }

    //Returns lines drawn
    fn draw_curve(&self) -> i32 {
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

        return lines_drawn;
    }

    //Drawing text is quite expensive!
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

            if !self.title.is_empty() {
                draw_text!(
                    self.get_datum_world_position(&Datum {
                        value: self.min,
                        tick: current_tick() as i32
                            - (self.timespan / TICK_LENGTH / 2.0).round() as i32
                    }),
                    self.color,
                    "{}",
                    self.title
                );
            }
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

        //Draw zero line
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

    fn get_start_tick(&self) -> i32 {
        return current_tick() as i32 - (self.timespan / TICK_LENGTH).round() as i32;
    }

    fn point_distance_to_line_squared(p: Vec2, l1: Vec2, l2: Vec2) -> f64 {
        return ((l2.x - l1.x) * (l1.y - p.y) - (l1.x - p.x) * (l2.y - l1.y)).powf(2.0)
            / ((l2.x - l1.x).powf(2.0) + (l2.y - l1.y).powf(2.0));
    }
}
