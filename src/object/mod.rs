pub mod entity;
pub mod item;

use std::any::Any;

pub trait Object: Any {
    fn get_name(&self) -> String;
}

pub trait Item: Object {
    fn use_on(&mut self, target: Box<dyn Object>);
}

pub trait Entity: Object + Any {
    fn take_damage(&mut self, amount: i32);
    //fn heal(&mut self);
    //fn interact(&mut self, sender: impl Object);
}

/* impl<T: Object> Entity for T {
    fn take_damage(&mut self, amount: i32) {
        println!("{} took {} damage.", self.get_name(), amount);
    }
} */

pub trait ObjectFactory<'a> {
    fn new() -> Self;
    fn name(self, name: &'a str) -> Self;
    fn create_object(&self) -> impl Object;
}

pub trait ItemFactory<'a>: ObjectFactory<'a> {}

pub trait EntityFactory<'a>: ObjectFactory<'a> {}
