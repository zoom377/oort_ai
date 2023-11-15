use crate::f64_extensions::F64Ex;
use oort_api::prelude::*;

pub struct Datum {
    pub value: f64,
    pub tick: u32,
}

pub struct Graph {
    pub label: String,
    pub position: Vec2,
    pub size: Vec2,
    pub min: f64,
    pub max: f64,
    pub time_span: f64,
    pub color: u32,
    pub data: Vec<Datum>,
    pub min_delta: f64,
    pub auto_grow: bool,
    pub auto_shrink: bool,
}

fn align_text(text: String, mut pos: Vec2, anchor: Vec2) -> Vec2 {
    const CHAR_WIDTH: f64 = 50.0;
    const CHAR_HEIGHT: f64 = 25.0;

    for _ in text.chars() {
        pos.x -= CHAR_WIDTH * anchor.x;
    }
    pos.y += CHAR_HEIGHT * (1.0 - anchor.y);

    return pos;
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            label: String::from("value"),
            position: vec2(0.0, 0.0),
            size: vec2(1000.0, 250.0),
            min: -10.0,
            max: 10.0,
            time_span: 10.0,
            color: 0xff0000,
            data: Vec::new(),
            min_delta: f64::EPSILON,
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
        //Dont add if the difference from last datum is insignificant
        if self.data.len() > 0 {
            let last = self.data[self.data.len() - 1].value;
            let delta = value - last;
            if delta.abs() < self.min_delta {
                return;
            }
        }

        self.data.push(Datum {
            value: value,
            tick: current_tick(),
        });
    }

    pub fn tick(&mut self) {
        self.shrink_grow();

        let left = self.position.x;
        let right = left + self.size.x;
        let bottom = self.position.y;
        let top = bottom + self.size.y;

        let top_left = vec2(left, top);
        let bottom_left = vec2(left, bottom);
        let zero_height = (0.0).remap(self.min, self.max, bottom, top);

        let start_tick = current_tick() as i32 - (self.time_span / TICK_LENGTH).round() as i32;

        // debug!("draw line from {} to {}", last_point, point);
        debug!("self.position: {}", self.position);
        debug!("self.size: {}", self.size);
        debug!("self.timespan: {}", self.time_span);
        debug!("self.data.len(): {}", self.data.len());

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

        //Draw labels
        draw_text!(
            align_text(self.max.to_string(), top_left, vec2(0.5, 0.5)),
            self.color,
            "{}",
            self.max
        );

        draw_text!(
            align_text(self.min.to_string(), bottom_left, vec2(0.5, 0.5)),
            self.color,
            "{}",
            self.min
        );

        //Draw centre line and label
        if (self.min <= 0.0 && self.max >= 0.0) {
            draw_line(
                vec2(self.position.x, self.position.y + zero_height),
                vec2(self.position.x + self.size.x, self.position.y + zero_height),
                0xcccccc,
            );

            draw_text!(
                align_text(
                    (0).to_string(),
                    vec2(left, bottom + zero_height),
                    vec2(0.5, 0.5)
                ),
                self.color,
                "{}",
                0
            );
        }

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
        debug!("small: {}, large: {}", smallest, largest);

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
