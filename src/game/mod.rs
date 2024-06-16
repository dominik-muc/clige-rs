mod player;

pub use player::Player;
mod level;
pub use level::Level;

pub mod object;
pub use object::*;

/// Possible states of the game.
pub enum State<'a> {
    Lost(u32),
    Won(u32),
    InProgress(&'a mut Level, &'a mut Player),
}

/// Struct `Game` is responsible for keeping track of the game state.
///
/// It holds reference to the player and the level he's in.
pub struct Game {
    player: Player,
    level: Box<Level>,
    score: u32,
}

impl Game {
    pub fn new(player: Player, level: Box<Level>) -> Self {
        Self {
            player,
            level,
            score: 0,
        }
    }

    /// Returns the state of the game, depending wheter player is still alive or not.
    pub fn get_state(&mut self) -> State {
        // The player cannot win.

        if !self.player.is_alive() {
            return State::Lost(self.score);
        }

        State::InProgress(&mut self.level, &mut self.player)
    }
}

/// Trait representing a container, that in some way contains `Objects`.
pub trait Container<T> {
    /// Add a new object to the container.
    fn add_child(&mut self, object: Box<T>);
}
