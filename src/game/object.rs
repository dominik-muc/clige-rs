use super::Action;

pub trait Object {
    fn handle(&mut self, action: Action) -> Result<(), ()>;
    fn draw(&self);
}
