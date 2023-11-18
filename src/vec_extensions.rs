use oort_api::prelude::*;

pub trait VecEx {
    fn cross(self) -> Vec2;
    fn length_squared(self) -> f64;
}

impl VecEx for Vec2 {
    fn cross(mut self) -> Vec2 {
        self = self.normalize();
        return vec2(-self.y, self.x);
    }

    fn length_squared(self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0)
    }
}
