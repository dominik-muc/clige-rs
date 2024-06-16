use crate::game::Container;

use super::{Action, Item, Message, Object, Player};

/// Represents diffrent kinds of weapons in game.
///
/// Player can equip diffrent weapons to change his damage.
///
/// Implements `Item`.
pub struct Weapon {
    name: String,
    health: i32,
}

impl Weapon {
    pub fn new(name: String, health: i32) -> Self {
        Self { name, health }
    }
}

impl Object for Weapon {
    fn handle(self: Box<Self>, sender: &mut Player, action: Action) -> Result<Message, String> {
        match action {
            Action::Equip => {
                let current_weapon = sender.take_weapon();
                match current_weapon {
                    Some(weapon) => sender.add_item(Box::new(weapon)),
                    None => ()
                }
                
                Ok(Message::Remove)
            }
            _ => (self as Box<dyn Item>).handle_default(sender, action),
        }
    }
    fn draw(&self) {
        color_print::cprintln!("{} <green, bold>{} HP", self.name, self.health);
    }
}

impl Item for Weapon {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
