mod entity;
mod item;
mod level;
mod door;

pub use entity::*;
pub use item::*;
pub use level::*;
pub use door::*;

use crate::error::GameError;

/// Possible actions player can take on an `Object`.
#[derive(PartialEq, Debug)]
pub enum Action {
    Help,
    Attack,
    Throw,
    Take,
    Eat,
    Enter,
    Equip,
    Back,
}

/// What the parent of an `Object` should do with it, after it's done handling logic.
pub enum Message {
    Keep(Box<dyn Object>),
    Remove,
    Equip(Box<Weapon>),
    ChangeLocation(*mut Level)
}

/// All game objects must implement this trait.
///
/// Each `Object` can be drawn on screen and can be interacted with.
pub trait Object {
    /// Handles interactions.
    fn handle(
        self: Box<Self>,
        sender: &mut dyn Entity,
        action: Action,
    ) -> Result<Message, GameError>;

    /// Prints itself to `stdout`.
    fn draw(&self);
}
