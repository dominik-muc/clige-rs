mod player;

use std::ops::DerefMut;

pub use player::Player;
mod level;

pub use level::Level;

use crate::game::utils::get_integer;

mod object;
mod utils;

use object::Object;
pub struct Game<'a> {
    player: Player<'a>,
    levels: Vec<&'a Level<'a>>,
}

#[derive(PartialEq, Debug)]
pub enum Action {
    Attack,
}

impl<'a> Game<'a> {
    pub fn new(player: Player<'a>, levels: Vec<&'a Level>) -> Self {
        Self { player, levels }
    }
    pub fn get_state(&mut self) {}

    pub fn handle(&mut self, target: i32, action: Action) -> Result<(), ()> {
        let level = self.player.get_location();
        let mut contents = level.get_contents();
        let object = match contents.get_mut(target as usize) {
            Some(o) => o,
            None => return Err(()),
        };
        object.handle(action)
    }
}
