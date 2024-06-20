//! Defines shared behaviour among all game items.

use super::{Action, Message, Object};

mod food;
mod weapon;

pub use food::*;
pub use weapon::*;

/// Trait for defining game items.
///
/// Each `Item` has a name.
pub trait Item: Object {
    /// Return the name of an `Item`.
    fn get_name(&self) -> &str;
}