use crate::{error::GameError, Entity};
use std::fmt::Display;

use super::{Action, Item, Message, Object};

/// Represents different kinds of weapons in game.
///
/// Entities can equip different weapons to change their's damage.
pub struct Weapon {
    name: String,
    damage: i32,
}

impl Weapon {
    pub fn new(name: String, damage: i32) -> Box<Self> {
        Box::new(Self { name, damage })
    }
    pub fn get_damage(&self) -> i32 {
        self.damage
    }
}

impl Object for Weapon {
    fn handle(
        self: Box<Self>,
        _sender: &mut dyn Entity,
        action: Action,
    ) -> Result<Message, GameError> {
        match action {
            Action::Throw => Ok(Message::Remove),
            Action::Equip => Ok(Message::Equip(self)),
            _ => Ok(Message::Keep(self)),
        }
    }
    fn draw(&self) {
        color_print::cprintln!("{} [Attack: <red, bold>{} HP</>]", self.name, self.damage);
    }
}

impl Item for Weapon {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {} DMG", self.name, self.damage)
    }
}
