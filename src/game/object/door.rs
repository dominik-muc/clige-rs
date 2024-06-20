use crate::{Action, Entity, GameError, Level, Message, Object};

/// This struct holds reference to some `Level`.
/// 
/// On enter, it will change the entity's location to this `Level`.
pub struct Door {
    level: *mut Level
}

impl Door{
    pub fn new(level: *mut Level) -> Box<Self>{
        Box::new(Self {level})
    }
}

impl Object for Door {
    fn handle(
        self: Box<Self>,
        mut sender: &mut dyn Entity,
        action: Action,
    ) -> Result<Message, GameError> {
        match action {
            Action::Enter => Ok(Message::ChangeLocation(self.level)),
            _ => Err(GameError::InvalidAction)
        }
    }

    fn draw(&self) {
        unsafe {color_print::cprintln!("Door: <yellow, bold>{}", (*self.level).get_name())}
    }
}