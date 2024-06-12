use better_game::*;
use game::{Game, Level, Player};

fn main() {
    let player = Player::new();
    let level = Level::new();
    let mut game = Game::new(player, level);
    better_game::run(&mut game);
}
