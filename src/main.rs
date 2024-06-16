use better_game::*;


fn main() {
    let player = game::Player::new("Defender94".to_string());

    let mut content: Vec<Box<dyn game::Object>> = Vec::new();
    content.push(Box::new(game::object::Food::new(format!("Apple"), 10)));

    let level = game::Level::new("Some level".to_string(), content);

    let mut game = game::Game::new(player, level);
    better_game::run(&mut game);
}
