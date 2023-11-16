use std::collections::{HashSet, VecDeque};

use crate::f64_extensions::F64Ex;
use oort_api::prelude::{maths_rs::powf, *};

fn log_time(label: String) {
    static mut LAST_TIME: f64 = 0.0;
    unsafe {
        let duration = current_time() - LAST_TIME;
        debug!("{}: {:.2}ms", label, duration * 1000.0);
        LAST_TIME = current_time();
        
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
    pub color: u32,
    pub line_quality: f64,
    pub show_labels: bool,
    pub auto_grow: bool,
    pub auto_shrink: bool,
    pub data: VecDeque<Datum>, //Don't set this. Haven't figured out private fields yet.
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            position: vec2(0.0, 0.0),
            size: vec2(100000.0, 250.0),
            min: 0.0,
            max: 0.0,
            timespan: 5.0,
            color: 0xff0000,
            line_quality: 0.0001,
            show_labels: true,
            auto_grow: true,
            auto_shrink: true,
            data: VecDeque::new(),
        }
    }
}

impl Graph {
    pub fn new() -> Graph {
        return Default::default();
    }

    pub fn add(&mut self, value: f64) {
        self.data.push_back(Datum {
            value: value,
            tick: current_tick() as i32,
        });
    }

    pub fn tick(&mut self) {
        log_time(String::from("Start"));
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
                    // debug!("{}, {}", pair.1.tick as f64, start_tick as f64);
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

        let epsilon_squared = self.get_epsilon_squared();
        let visible_range = self.get_visible_indices_range();
        let mut critical_points_indices = HashSet::<usize>::new();
        let mut stack = VecDeque::<(usize, usize)>::new();
        debug!("Visible range: [{}..{}]", visible_range.0, visible_range.1);
        debug!("Epsilon^2: {}", epsilon_squared);

        stack.push_back(visible_range);
        critical_points_indices.insert(0);
        critical_points_indices.insert(self.data.len() - 1);
        let mut loops = 0;
        let mut calculations = 0;

        while stack.front().is_some() {
            let current = stack.pop_front().unwrap();
            let line_start = self.get_datum_world_position(&self.data[current.0]);
            let line_end = self.get_datum_world_position(&self.data[current.1]);
            let mut largest_distance = 0.0;
            let mut largest_index: usize = 0;
            loops += 1;

            if current.0 + 1 >= current.1 - 1 {
                //<= 0 points between these two
                continue;
            }

            //Find furthest point from current line
            for index in current.0 + 1..current.1 - 1 {
                let datum_world_position = self.get_datum_world_position(&self.data[index]);
                let distance_from_line = Graph::point_distance_to_line_squared(
                    datum_world_position,
                    line_start,
                    line_end,
                );
                calculations += 1;
                if distance_from_line > largest_distance {
                    largest_index = index;
                    largest_distance = distance_from_line;
                }
            }

            //Subdivide line if a point exceeding epsilon was found
            if largest_distance >= epsilon_squared {
                critical_points_indices.insert(largest_index);
                stack.push_back((current.0, largest_index));
                stack.push_back((largest_index, current.1));
            }
        }

        //Add last graph point to be drawn

        debug!("Data points: {}", self.data.len());
        debug!("Critical data points: {}", critical_points_indices.len());
        debug!("Point-line distance calculations: {}", calculations);
        debug!("Loops: {}", loops);
        // debug!("Visible range: {},{}", visible_range.0, visible_range.1);

        //Draw curve
        let mut is_first_point = true;
        let mut last_point: Vec2 = Default::default();
        let mut lines_drawn = 0;
        for index in visible_range.0..visible_range.1 + 1 {
            if critical_points_indices.contains(&index) {
                let point = self.get_datum_world_position(&self.data[index]);
                if is_first_point == true {
                    is_first_point = false;
                } else {
                    draw_line(last_point, point, self.color);
                    lines_drawn += 1;
                }
                last_point = point;
            }
        }
        debug!("Lines drawn: {}", lines_drawn);
        log_time(String::from("End"));
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

        //distance to line = ((l2.x - l1.x) * (l1.y - p.y) - (l1.x - p.x) * (l2.y - l1.y)) / f64::sqrt(f64::powf(l2.x - l1.x, 2.0) + f64::powf(l2.y - l1.y, 2.0))
        //distance to line^2 * f64::powf(l2.x - l1.x, 2.0) + f64::powf(l2.y - l1.y, 2.0) = ((l2.x - l1.x) * (l1.y - p.y) - (l1.x - p.x) * (l2.y - l1.y))^2
        //distance to line^2 = ((l2.x - l1.x) * (l1.y - p.y) - (l1.x - p.x) * (l2.y - l1.y))^2 / f64::powf(l2.x - l1.x, 2.0) + f64::powf(l2.y - l1.y, 2.0)
    }

    fn get_epsilon_squared(&self) -> f64 {
        return (self.size.x.powf(2.0) + self.size.y.powf(2.0)) * self.line_quality;
    }
}
