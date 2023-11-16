use std::collections::VecDeque;

use crate::f64_extensions::F64Ex;
use oort_api::prelude::*;

pub struct Datum {
    pub value: f64,
    pub tick: u32,
}

pub struct Graph {
    pub position: Vec2,
    pub size: Vec2,
    pub min: f64,
    pub max: f64,
    pub time_span: f64,
    pub color: u32,
    pub data: VecDeque<Datum>,
    pub min_delta: f64,
    pub enable_dynamic_min_delta: bool,
    pub show_labels: bool,
    pub auto_grow: bool,
    pub auto_shrink: bool,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            position: vec2(0.0, 0.0),
            size: vec2(1000.0, 250.0),
            min: 0.0,
            max: 0.0,
            time_span: 5.0,
            color: 0xff0000,
            data: VecDeque::new(),
            min_delta: f64::EPSILON,
            enable_dynamic_min_delta: true,
            show_labels: true,
            auto_grow: true,
            auto_shrink: true,
        }
    }
}

impl Graph {
    pub fn new() -> Graph {
        return Default::default();
    }

    pub fn add(&mut self, value: f64) {
        let mut min_delta = self.min_delta;
        if self.enable_dynamic_min_delta {
            min_delta = (self.max - self.min) / 100.0;
        }

        //Todo: Keep ALL data as it can't cost much.
        //Add a seperate pass over the vector for drawing lines/filtering.
        //30 seconds * 60 ticks per second = 1800 total ticks = 1800 total vec elements

        //Dont add if the difference from last datum is insignificant
        if self.data.len() > 0 {
            let last = self.data[self.data.len() - 1].value;
            let delta = value - last;
            if delta.abs() < min_delta {
                return;
            }
        }

        self.data.push_back(Datum {
            value: value,
            tick: current_tick(),
        });
    }

    pub fn tick(&mut self) {
        //Calculate visible boundaries of graph
        let left = self.position.x;
        let right = left + self.size.x;
        let bottom = self.position.y;
        let top = bottom + self.size.y;

        let top_left = vec2(left, top);
        let bottom_left = vec2(left, bottom);
        let start_tick = current_tick() as i32 - (self.time_span / TICK_LENGTH).round() as i32;

        // Pop invisible data points
        let mut first_visible_tick = 0;
        for pair in self.data.iter().enumerate() {
            if (pair.1.tick as f64) >= (start_tick as f64) {
                debug!("{}, {}", pair.1.tick as f64, start_tick as f64);
                //Found first visible data point. Everything before must be invisible
                first_visible_tick = pair.1.tick;
                break;
            }
        }

        let mut last_pop: Option<Datum> = None;
        while let Some(front) = self.data.front() {
            if front.tick == first_visible_tick {
                break;
            }
            last_pop = self.data.pop_front();
        }

        if let Some(pop) = last_pop {
            self.data.push_front(pop);
        }

        self.shrink_grow();

        let zero_height = (0.0).remap(self.min, self.max, bottom, top);

        debug!("self.data.len(): {}", self.data.len());

        //Draw labels
        if self.show_labels {
            draw_text!(top_left, self.color, "{:.2}", self.max);
            draw_text!(bottom_left, self.color, "{:.2}", self.min);
        }

        //Draw axes
        draw_line(
            vec2(self.position.x, self.position.y),
            vec2(self.position.x + self.size.x, self.position.y),
            0xffffff,
        );
        draw_line(
            vec2(self.position.x, self.position.y),
            vec2(self.position.x, self.position.y + self.size.y),
            0xffffff,
        );

        //Draw zero line and label
        if self.min < 0.0 && self.max > 0.0 {
            draw_line(
                vec2(self.position.x, self.position.y + zero_height),
                vec2(self.position.x + self.size.x, self.position.y + zero_height),
                0xcccccc,
            );
        }


        //Draw curve
        let mut is_first_point = true;
        let mut last_point: Vec2 = Default::default();

        for datum in &self.data {
            let x =
                f64::from(datum.tick).remap(start_tick as f64, current_tick() as f64, left, right);
            let y = datum.value.remap(self.min, self.max, bottom, top);
            let point = vec2(x, y);
            if point.x >= left && point.x < right {
                if is_first_point == true {
                    is_first_point = false;
                } else {
                    draw_line(last_point, point, self.color)
                }
                last_point = point;
            }
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
}
