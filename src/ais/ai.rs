pub trait AI {
    fn name(&self) -> String;
    fn tick(&mut self);
}
