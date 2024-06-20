use crate::{Level, Weapon};

use super::{Action, Message, Object};
mod enemy;
mod player;
pub use enemy::*;
pub use player::*;

/// Trait for defining game items.
///
/// Each `Item` has a name. It can be picked up or throwed out of inventory.
pub trait Entity: Object {
    /// Return the name of an `Entity`.
    fn get_name(&self) -> &str;

    /// Check wheter `Entity` is still alive.
    fn is_alive(&self) -> bool;

    /// Deal `amount` damage to the `Entity`.
    fn take_damage(&mut self, amount: i32);

    /// Get attack damage from `Entity`.
    fn attack(&self) -> i32;

    /// Restore `amount` health to the `Entity`.
    fn restore_health(&mut self, amount: i32);

    /// Make `entity`` equip `weapon` to it's weapon slot.
    fn equip_weapon(&mut self, weapon: Box<Weapon>);

    /// Change location of an `Entity`.
    fn change_location(&mut self, location: *mut Level);

    /// Add score to the `Entity`.
    fn add_score(&mut self, amount: i32);
}
