mod player;

pub use player::Player;
mod level;

pub use level::Level;

use crate::game::utils::get_integer;

mod object;
mod utils;
pub struct Game<'a> {
    player: Player<'a>,
    levels: Vec<&'a Level<'a>>,
}

impl<'a> Game<'a> {
    pub fn new(player: Player<'a>, levels: Vec<&'a Level>) -> Self {
        Self { player, levels }
    }
    pub fn run(self) {
        loop {
            let level = self.player.get_location();
            let contents = level.get_contents();
            println!("Jesteś na poziomie: {}", level.get_name());
            println!("Znajduje się tutaj: ");
            for entity in contents {}
            print!("Podaj numer: ");
            let choice = get_integer();

            /* contents.get(choice as usize); */
        }
    }
}
