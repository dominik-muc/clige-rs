mod player;

use std::{ops::DerefMut, rc::Rc};

pub use player::Player;
mod level;

pub use level::Level;

use crate::game::utils::get_integer;

mod object;
mod utils;

use object::Object;
pub struct Game {
    player: Player,
    level: Box<Level>,
}

#[derive(PartialEq, Debug)]
pub enum Action {
    Attack,
}

pub enum State<'a> {
    Lost(i32),
    Won(i32),
    InProgress(&'a mut Level, u32),
}

impl Game {
    pub fn new(player: Player, level: Box<Level>) -> Self {
        Self { player, level }
    }
    pub fn get_state(&mut self) -> State {
        State::InProgress(&mut self.level, self.player.get_health())
    }

    /* pub fn handle(&mut self, target: i32, action: Action) -> Result<(), ()> {
        let level = self.player.get_location();
        let mut contents = level.get_contents();
        let object = match contents.get_mut(target as usize) {
            Some(o) => o,
            None => return Err(()),
        };
        object.handle(action)
    } */
}
