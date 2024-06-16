use super::Container;
pub use super::Player;

mod food;
mod weapon;
mod item;

pub use food::*;
pub use weapon::*;
pub use item::*;

/// Possible actions player can take on an `Object`.
#[derive(PartialEq, Debug)]
pub enum Action {
    Help,
    Attack,
    Inventory,
    Take,
    Throw,
    Eat,
    Enter,
    Equip,
    Back
}

/// What the parent of an `Object` should do with it, after it's done handling logic.
pub enum Message<'a> {
    Keep(Box<dyn Object>),
    Remove,
    Move(&'a mut Player, Box<dyn Item>),
    Equip(Weapon),
}

/// All game objects must implement this trait.
///
/// Each `Object` can be drawn on screen and can be interacted with.
pub trait Object {
    /// Handles interactions.
    fn handle(self: Box<Self>, sender: &mut Player, action: Action) -> Result<Message, String>;

    /// Prints itself to `stdout`.
    fn draw(&self);
}
