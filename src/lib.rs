pub mod game;

use game::Action;
use game::Game;
use game::State;

pub fn run(game: &mut Game) {
    loop {
        let state = game.get_state();
        match state {
            State::Lost(score) => (),
            State::Won(score) => (),
            State::InProgress(location, health) => {
                location.draw();
                draw_input_hint();
                let input = get_user_input();
                location
                    .handle(input.0, input.1)
                    .expect("For now, lets assume that it works")
            }
        }
    }
}

pub fn draw_input_hint() {}

pub fn get_user_input() -> (i32, Action) {
    let mut input = String::new();
    loop {
        std::io::stdin()
            .read_line(&mut input)
            .expect("Can't open stdin.");

        match parse_input(&input) {
            Ok(action) => break action,
            Err(e) => println!("Error occured: {}. Please try again.", e),
        }
    }
}

pub fn parse_input(input: &str) -> Result<(i32, Action), String> {
    let mut tokens = input.split_ascii_whitespace();
    match tokens.next() {
        Some("attack") => Ok((0, Action::Attack)),
        Some(other) => Err(format!("Unkown action: {}", other)),
        None => Err(format!("Provide an action")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(parse_input("attack 4").unwrap(), (4, Action::Attack));
        assert_eq!(parse_input("wtf").unwrap_err(), "Unkown action: wtf");
    }
}
