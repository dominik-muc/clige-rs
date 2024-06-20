pub mod object;
pub use object::*;

use crate::*;

/// Possible states of the game.
pub enum GameState {
    Lost(i32),
    Won(i32),
    InProgress,
}

/// Struct `Game` is responsible for keeping track of the game state.
///
/// It holds reference to the player and the level he's in.
pub struct Game {
    player: Box<Player>,
}

impl Game {
    /// Create an instance of a `Game`.
    pub fn new(player: Box<Player>) -> Self {
        Self {
            player,
        }
    }

    /// Runs the main loop
    pub fn run_loop(&mut self) {
        clear_console();
        loop {
            self.draw();
            draw_input_hint();
            let (action, target) = get_user_input();
            let result = self
                .handle_input(target, action);
            
            let state = match result{
                Ok(state) => {clear_console(); state},
                Err(e) => {clear_console(); println!("Error occured: {}.\nPlease try again.\n", e.what()); continue}
            };
            match state {
                GameState::Lost(score) => break lose_screen(score),
                GameState::Won(score) =>break win_screen(score),
                GameState::InProgress => ()
            }
        }
    }

    /// Handles player input and returns the resulting `GameState`.
    pub fn handle_input(&mut self, target: i32, action: Action) -> Result<GameState, GameError>{
        let location = self.player.get_location();

        // This is the antichrist himself.
        // I will handle it better, when I get the idea how.
        // For now, this should do.
        unsafe {(*location).handle(&mut *(self.player), target, action)?;}

        // The player cannot win.

        if !self.player.is_alive() {
            return Ok(GameState::Lost(self.player.get_score()));
        }

        Ok(GameState::InProgress)
    }

    /// Draws the game.
    pub fn draw(&self){
        self.player.draw();
        let location = self.player.get_location();

        // :(
        unsafe {(*location).draw()}
    }

}
