use super::{Action, Message, Object, Player};

/// Trait for defining game items.
///
/// Each `Item` has a name. It can be picked up or throwed out of inventory.

pub trait Item : Object{
    /// Return the name of an `Item`.
    fn get_name(&self) -> &str;
}

/* impl Object for dyn Item {
    fn handle(self: Box<Self>, sender: &mut Player, action: Action) -> Result<Message, String> {
        match action {
            Action::Take => Ok(Message::Move(sender, self)),
            Action::Throw => Ok(Message::Remove),
            _ => Err(format!("You cannot do that.")),
        }
    }

    fn draw(&self) {
        println!("{}", self.get_name())
    }
}
 */
impl dyn Item {
    pub fn handle_default(self: Box<Self>, sender: &mut Player, action: Action) -> Result<Message, String> {
        match action {
            Action::Take => Ok(Message::Move(sender, self)),
            Action::Throw => Ok(Message::Remove),
            _ => Err(format!("You cannot do that.")),
        }
    }
}
