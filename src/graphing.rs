use std::{
    collections::{HashMap, HashSet, VecDeque},
    num::ParseIntError,
};

use crate::f64_extensions::F64Ex;
use oort_api::prelude::{maths_rs::powf, *};

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
    pub data: VecDeque<Datum>, //Don't set this. Haven't figured out private fields yet.
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
            timespan: 5.0,
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
        // if self.data.len() > 0 {
        //     let last = self.data[self.data.len() - 1].value;
        //     let delta = value - last;
        //     if delta.abs() < min_delta {
        //         return;
        //     }
        // }

        self.data.push_back(Datum {
            value: value,
            tick: current_tick() as i32,
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
        let start_tick = current_tick() as i32 - (self.timespan / TICK_LENGTH).round() as i32;

        // Pop invisible data points
        {
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
        }

        self.shrink_grow();
        

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
                self.get_datum_world_position(&Datum {
                    value: 0.0,
                    tick: start_tick,
                }),
                self.get_datum_world_position(&Datum {
                    value: 0.0,
                    tick: current_tick() as i32,
                }),
                0xcccccc,
            );
        }

        //Draw curve
        //Using Douglas-Puecker algo to reduce lines drawn
        if self.data.len() < 2 {
            return;
        }

        let epsilon = 0.0;
        let visible_range = self.get_visible_range_indices();
        let mut critical_points = Vec::<Vec2>::new();
        let mut stack = VecDeque::<(usize, usize)>::new();

        stack.push_back(visible_range);
        critical_points.push(self.get_datum_world_position(&self.data[0]));

        while stack.front().is_some() {
            let current = stack.pop_front().unwrap();
            let line_start = self.get_datum_world_position(&self.data[current.0]);
            let line_end = self.get_datum_world_position(&self.data[current.1]);
            let mut largest_distance = f64::MIN;
            let mut largest_index: usize;

            if current.0 + 1 <= current.1 - 1 {
                break;
            }

            for index in current.0 + 1..current.1 - 1 {
                let datum_world_position = self.get_datum_world_position(&self.data[index]);
                let distance_from_line =
                    Graph::point_distance_to_line(datum_world_position, line_start, line_end);
                if distance_from_line > largest_distance && distance_from_line >= epsilon {
                    largest_index = index;
                    largest_distance = distance_from_line;
                    critical_points.push(datum_world_position);
                    stack.push_back((current.0, index));
                    stack.push_back((index, current.1));
                }
            }
        }
        critical_points.push(self.get_datum_world_position(&self.data[self.data.len() - 1]));
        debug!("data count: {}", self.data.len());
        debug!("visible range: {},{}", visible_range.0, visible_range.1);
        debug!("critical data count: {}", critical_points.len());

        //Draw critical points
        let mut is_first_point = true;
        let mut last_point: Vec2 = Default::default();
        for point in critical_points {
            if is_first_point == true {
                is_first_point = false;
            } else {
                draw_line(last_point, point, self.color)
            }
            last_point = point;
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

    fn get_visible_range_indices(&self) -> (usize, usize) {
        let start_tick = self.get_start_tick();
        let mut range = (0, self.data.len() - 1);
        while self.data[range.0].tick < start_tick {
            range.0 += 1;
        }
        return range;
    }

    fn get_start_tick(&self) -> i32 {
        return current_tick() as i32 - (self.timespan / TICK_LENGTH).round() as i32;
    }

    fn point_distance_to_line(p: Vec2, l1: Vec2, l2: Vec2) -> f64 {
        return ((l2.x - l1.x) * (l1.y - p.y) - (l1.x - p.x) * (l2.y - l1.y))
            / f64::sqrt(f64::powf(l2.x - l1.x, 2.0) + f64::powf(l2.y - l1.y, 2.0));
    }
}
