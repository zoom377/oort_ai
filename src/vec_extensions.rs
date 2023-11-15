use oort_api::prelude::*;

pub trait VecEx {
    fn cross(self) -> Vec2;
}

impl VecEx for Vec2 {
    fn cross(mut self) -> Vec2 {
        self = self.normalize();
        return vec2(-self.y, self.x);
    }
}
