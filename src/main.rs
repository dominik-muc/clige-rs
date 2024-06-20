use clige_rs::*;

/// This is a simple game showcasing implemented objects and interactions
fn main() {
    // Create empty other level
    let mut other_level = Level::new("Other level".to_string(), Vec::new());

    // Create the content for main level
    let mut content: Vec<Box<dyn game::Object>> = Vec::new();
    content.push(Food::new(format!("Apple"), 10));
    content.push(Weapon::new(format!("Sword"), 10));
    content.push(Enemy::new());
    content.push(Door::new(&mut other_level as *mut Level));

    // Create main level with given content
    let mut level = Level::new("Some level".to_string(), content);

    // Create the player inside the level
    let player = Player::new("Player".to_string(), &mut level as *mut Level);

    // Create game and run the loop
    let mut game = Game::new(Box::new(player));
    game.run_loop();
}
