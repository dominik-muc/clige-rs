use super::{object::Object, Action, Container, Item, Message, Player, Weapon};



/// Main container for game objects.
///
/// Each `Level` has a name and a vector of owned objects.
pub struct Level {
    name: String,
    content: Vec<Box<dyn Object>>,
}

impl Level {
    /// Constructs `Level` with given `name` and `content`.
    pub fn new(name: String, content: Vec<Box<dyn Object>>) -> Box<Self> {
        Box::new(Self { name, content })
    }

    /// Prints on screen it's own name and then all stored objects.
    pub fn draw(&self) {
        color_print::cprintln!(
            "\
You are in <yellow, bold>{}</>.
Here you can find:",
            self.name
        );

        let mut i = 0;
        for object in &self.content {
            print!("\t{}. ", i);
            object.draw();
            i += 1;
        }
    }

    /// Handles player's actions.
    ///
    /// `sender` is the player that issued an action.
    ///
    /// `target` is the index of targeted object in `content`
    pub fn handle(
        &mut self,
        sender: &mut Player,
        target: i32,
        action: Action,
    ) -> Result<(), String> {
        let id = target as usize;

        let object = self.content.remove(id);

        let message = object.handle(sender, action)?;

        match message {
            Message::Keep(object) => self.content.insert(id, object),
            Message::Remove => {}
            Message::Move(player, item) => player.add_item(item),
            Message::Equip(weapon) => sender.equip_weapon(weapon),
        }

        Ok(())
    }

    pub fn add_child(&mut self, item: Box<impl Item + 'static>){
        self.content.push(item)
    }
}
