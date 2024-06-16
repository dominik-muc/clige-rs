use super::{Action, Item, Message, Object, Player};

/// Represents diffrent kinds of food in game.
///
/// Player can eat food to restore health.
///
/// Implements `Item`.
pub struct Food {
    name: String,
    health: i32,
}

impl Food {
    pub fn new(name: String, health: i32) -> Self {
        Self { name, health }
    }
}

impl Object for Food {
    fn handle(self: Box<Self>, sender: &mut Player, action: Action) -> Result<Message, String> {
        match action {
            Action::Eat => {
                sender.restore_health(self.health);
                Ok(Message::Remove)
            }
            _ => (self as Box<dyn Item>).handle_default(sender, action),
        }
    }
    fn draw(&self) {
        color_print::cprintln!("{} <green, bold>{} HP", self.name, self.health);
    }
}

impl Item for Food {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
