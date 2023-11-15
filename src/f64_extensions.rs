use oort_api::prelude::*;

pub trait F64Ex {
    fn move_towards(self, target: f64, max_delta: f64) -> f64;
    fn remap(self, min: f64, max: f64, new_min: f64, new_max: f64) -> f64;
}

impl F64Ex for f64 {
    fn move_towards(mut self, target: f64, max_delta: f64) -> f64 {
        return self + (target - self).clamp(-max_delta, max_delta);
    }

    fn remap(self, min: f64, max: f64, new_min: f64, new_max: f64) -> f64 {
        let t = (self - min) / (max - min);
        // let t = (self - min) / (max - min);
        // let t(max - min) = self - min;
        // let t(max - min) + min = self;
        let res = t * (new_max - new_min) + new_min;
        return res;
    }
}
