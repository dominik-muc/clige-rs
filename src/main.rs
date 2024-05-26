mod game;
mod object;
use game::Game;
use game::Level;
use game::Player;

fn main() {
    let levels: Vec<&Level> = Vec::new();
    let player = Player::new(levels[0]);
    let game = Game::new(player, levels);

    game.run();
}
