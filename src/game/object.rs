use super::Action;

pub trait Object {
    fn handle(&mut self, action: Action) -> Result<(), String>;
    fn draw(&self);
}

pub struct Item {

}

impl Object for Item {
    fn handle(&mut self, action: Action) -> Result<(), String> {
        match action {
            _ => Err(format!("You cannot do that."))
        }
    }
    fn draw(&self) {
        println!("Im an item!")
    }
}