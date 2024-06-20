use std::ops::Deref;

use crate::{error::GameError, Entity};

use super::{Action, Item, Message, Object, Player};

/// Main container for game objects.
///
/// Each `Level` has a name and a vector of owned objects.
pub struct Level {
    name: String,
    content: Vec<Box<dyn Object>>,
}

impl Level {
    /// Constructs `Level` with given `name` and `content`.
    pub fn new(name: String, content: Vec<Box<dyn Object>>) -> Self {
        Self { name, content }
    }

    /// Get level's name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Prints on screen it's own name and then all stored objects.
    pub fn draw(&self) {
        color_print::cprintln!(
            "\
You are in <yellow, bold>{}</>.
Here you can find:",
            self.name
        );

        let mut i = 0;
        for object in &self.content {
            print!("\t{}. ", i);
            object.draw();
            i += 1;
        }
    }

    /// Handles players' actions.
    ///
    /// `sender` is the `Entity` that issued an action.
    ///
    /// `target` is the index of targeted object in `content`
    pub fn handle(
        &mut self,
        mut sender: &mut dyn Entity,
        target: i32,
        action: Action,
    ) -> Result<(), GameError> {
        let id = target as usize;

        if id >= self.content.len(){
            return Err(GameError::InvalidTarget)
        }

        let object = self.content.remove(id);

        let message = object.handle(sender, action)?;

        match message {
            Message::Keep(object) => self.content.insert(id, object),
            Message::Remove => {}
            Message::Equip(weapon) => sender.equip_weapon(weapon),
            Message::ChangeLocation(new_location) => sender.change_location(new_location),
        }

        Ok(())
    }

    pub fn add_child(&mut self, item: Box<impl Item + 'static>) {
        self.content.push(item)
    }

    pub fn get_objects(&mut self) -> &mut Vec<Box<dyn Object>>{
        &mut self.content
    }
}
