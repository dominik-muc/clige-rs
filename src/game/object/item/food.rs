use crate::{error::GameError, Entity};

use super::{Action, Item, Message, Object};

/// Represents different kinds of food in game.
///
/// Entities can eat food to restore health.
pub struct Food {
    name: String,
    health: i32,
}

impl Food {
    pub fn new(name: String, health: i32) -> Box<Self> {
        Box::new(Self { name, health })
    }
}

impl Object for Food {
    fn handle(
        self: Box<Self>,
        sender: &mut dyn Entity,
        action: Action,
    ) -> Result<Message, GameError> {
        match action {
            Action::Throw => Ok(Message::Remove),
            Action::Eat => {
                sender.restore_health(self.health);
                Ok(Message::Remove)
            }
            _ => Ok(Message::Keep(self)),
        }
    }
    fn draw(&self) {
        color_print::cprintln!(
            "{} [Restore: <green, bold>{} HP</>]",
            self.name,
            self.health
        );
    }
}

impl Item for Food {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
