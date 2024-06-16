//! Player module

use super::{Container, Item, Level, Object, Weapon};

/// Struct for keeping track of the player.
pub struct Player {
    name: String,
    health: i32,
    max_health: i32,
    inventory: Box<Level>,
    weapon: Option<Weapon>,
}

impl Player {
    /// Constructs a new `Player` with given `name`. It has 100 health by default.
    pub fn new(name: String) -> Self {
        let inventory = Level::new("Inventory".to_string(), Vec::new());
        Self {
            name,
            health: 80,
            max_health: 100,
            inventory,
            weapon: None,
        }
    }

    /// Check wheter `Player` is still alive.
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Prints `Player` info to `stdout`.
    pub fn draw(&self) {
        color_print::cprintln!(
            "Name: <cyan, bold>{}</> HP: <green, bold>{}/{}",
            self.name,
            self.health,
            self.max_health
        )
    }

    pub fn add_item(&mut self, item: Box<impl Item + 'static>){
        self.inventory.add_child(item);
    }

    pub fn show_inventory(&self){
        self.inventory.draw();
    }

    /// Restores `amount` health to the player.
    pub fn restore_health(&mut self, amount: i32) {
        self.health = self.health.saturating_add(amount).clamp(0, self.max_health);
    }

    /// Take the weapon out of player's weapon slot.
    pub fn take_weapon(&mut self) -> Option<Weapon>{
        self.weapon.take()
    }

    /// Make player equip `weapon` to his weapon slot.
    pub fn equip_weapon(&mut self, weapon: Weapon){
        self.weapon = Some(weapon)
    }
}
