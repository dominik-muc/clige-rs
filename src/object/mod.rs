pub mod entity;
pub mod item;

pub trait Object {
    fn get_name(&self) -> &str;
}

pub trait Item: Object {
    fn use_on(&mut self, target: impl Object);
}

pub trait Entity: Object {
    fn take_damage(&mut self);
    fn heal(&mut self);
    fn interact(&mut self, sender: impl Object);
}


pub trait ObjectFactory<'a> {
    fn new() -> Self;
    fn name(self, name: &'a str) -> Self;
    fn create_object(&self) -> impl Object;
}

pub trait ItemFactory<'a>: ObjectFactory<'a> {}

pub trait EntityFactory<'a>: ObjectFactory<'a> {}




