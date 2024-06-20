//! Player module

use crate::{error::GameError, Action, Entity, Level, Message, Object, Weapon};

/// Struct for keeping track of the player.
pub struct Player {
    name: String,
    health: i32,
    max_health: i32,
    location: *mut Level,
    weapon: Box<Weapon>,
    score: i32
}

impl Player {
    /// Constructs a new `Player` with given `name`. It has 100 health by default and no weapon equipped.
    pub fn new(name: String, location: *mut Level) -> Self {
        Self {
            name,
            health: 80,
            max_health: 100,
            score: 0,
            weapon: Weapon::new("None".to_string(), 0),
            location
        }
    }

    pub fn get_location(&self) -> *mut Level{
        self.location
    }

    pub fn get_score(&self) -> i32{
        self.score
    }
}

impl Object for Player {
    fn draw(&self) {
        color_print::cprintln!(
            "Name: <cyan, bold>{}</> HP: <green, bold>{}/{}</> Weapon: <red,bold>{}</>",
            self.name,
            self.health,
            self.max_health,
            self.weapon
        )
    }

    fn handle(
        mut self: Box<Self>,
        sender: &mut dyn Entity,
        action: Action,
    ) -> Result<Message, GameError> {
        match action {
            Action::Attack => {
                self.take_damage(sender.attack());

                // We check whether Player is still alive in `Game` struct.

                Ok(Message::Keep(self))
            }
            _ => Err(GameError::InvalidAction),
        }
    }
}

impl Entity for Player {
    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
    }

    fn attack(&self) -> i32 {
        self.weapon.get_damage()
    }

    fn restore_health(&mut self, amount: i32) {
        self.health = self.health.saturating_add(amount).clamp(0, self.max_health);
    }

    fn equip_weapon(&mut self, weapon: Box<Weapon>) {
        self.weapon = weapon
    }

    fn change_location(&mut self, location: *mut Level) {
        self.location = location;
    }

    fn add_score(&mut self, amount: i32) {
        self.score += amount;
    }
}

