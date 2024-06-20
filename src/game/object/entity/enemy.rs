use crate::{Action, Entity, GameError, Level, Message, Object, Weapon};

/// A simple enemy. If it's attacked by the `Player`, it attacks back.
///
/// In the future this behaviour will be handled by AI, so that enemy always attacks nearby `Player`.
pub struct Enemy {
    health: i32,
    weapon: Box<Weapon>,

}

impl Enemy {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            health: 10,
            weapon: Weapon::new("Enemy's sword".to_string(), 10),
        })
    }
}

impl Object for Enemy {
    fn handle(
        mut self: Box<Self>,
        mut sender: &mut dyn Entity,
        action: Action,
    ) -> Result<Message, GameError> {
        match action {
            Action::Attack => {
                // Attack the attacker. Later on this will be handled by Enemy's AI
                sender.take_damage(self.attack());

                // And then take damage
                self.take_damage(sender.attack());

                // Check whether I'm still alive
                if self.is_alive() {
                    Ok(Message::Keep(self))
                } else {
                    // Add score to the killer and get removed.
                    sender.add_score(100);
                    Ok(Message::Remove)
                }
            }
            _ => Ok(Message::Keep(self)),
        }
    }

    fn draw(&self) {
        println!("Enemy [HP: {}]", self.health)
    }
}

impl Entity for Enemy {
    fn get_name(&self) -> &str {
        "some enemy"
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
    }

    fn attack(&self) -> i32 {
        self.weapon.get_damage()
    }

    fn restore_health(&mut self, amount: i32) {
        self.health += amount;
    }

    fn equip_weapon(&mut self, weapon: Box<Weapon>) {
        self.weapon = weapon;
    }

    fn change_location(&mut self, location: *mut Level) {
        panic!("NOT IMPLEMENTED")
    }

    fn add_score(&mut self, amount: i32){
        panic!("NOT IMPLEMENTED")
    }
}
