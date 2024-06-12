use std::io::{stdout, Write};

use super::{object::{Item, Object}, Action};

pub struct Level {
    name: String,
    content: Vec<Box<dyn Object>>,
}

impl Level {
    pub fn new() -> Box<Self> {
        let mut content: Vec<Box<dyn Object>> = Vec::new();
        content.push(Box::new(Item{}));
        content.get(0).expect("").draw();
        Box::new(Self {
            name: "Unnamed level".to_string(),
            content
        })
    }
    pub fn draw(&mut self) {
        color_print::cprintln!(
            "\
You are in <cyan, bold>{}</>.
Here you can find:",
            self.name
        );
        let mut i = 1;
        for object in &(self.content) {
            print!("\t{}. ", i);
            object.draw();
            i += 1;
        }
    }
    pub fn handle(&mut self, target: i32, action: Action) -> Result<(), ()> {
        let object = match self.content.get_mut(target as usize) {
            Some(o) => o,
            None => return Err(()),
        };
        object.handle(action)
    }
}
