pub trait F64Ex {
    fn move_towards(self, target: f64, max_delta: f64) -> f64;
    fn remap(self, min: f64, max: f64, new_min: f64, new_max: f64) -> f64;
    fn lerp_inverse(self, min: f64, max: f64) -> f64;
    fn lerp(self, min: f64, max: f64) -> f64;
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
