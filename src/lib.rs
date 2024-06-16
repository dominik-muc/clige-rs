pub mod game;
pub mod generator;

use game::Action;
use game::Game;
use game::State;

/// Runs the game loop.
pub fn run(game: &mut Game) {
    loop {
        let state = game.get_state();
        match state {
            State::Lost(score) => lose_screen(score),
            State::Won(score) => win_screen(score),
            State::InProgress(location, player) => {
                player.draw();
                location.draw();
                draw_input_hint();
                let (action, target) = get_user_input();
                if action == Action::Inventory {
                    player.show_inventory();
                } else {
                location
                    .handle(player, target, action)
                    .expect("For now, lets assume that it works")
                }
            }
        }
    }
}

fn lose_screen(score: u32) {
    color_print::cprintln!(
        "\
You've <red, bold>lost!</> You scored: <yellow, bold>{}",
        score
    );
}

fn win_screen(score: u32) {
    color_print::cprintln!(
        "\
You've <green, bold>won!</> You scored: <yellow, bold>{}",
        score
    );
}

fn draw_input_hint() {
    color_print::cprintln!(
        "\
What do you want to do?
Type <cyan, bold>action <<id>></>. To view possible actions, type <red, bold>help."
    );
}

fn draw_actions_hint() {
    color_print::cprintln!(
        "\
Available actions: <cyan, bold>
attack
take
throw
eat
enter"
    );
}

/// Reads line from `stdin`, then tries to parse it into action.
/// If the action provided isn't valid, it reads another line.
/// ### Panics
/// Function panics if it can't open `stdin`.
fn get_user_input() -> (Action, i32) {
    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Can't open stdin.");

        match parse_input(&input) {
            Ok((Action::Help, _)) => draw_actions_hint(),
            Ok(action) => break action,
            Err(e) => println!("Error occured: {}. Please try again.", e),
        }
    }
}

/// This function tries to parse `input` into pair of action and target.
/// If the action is `help`, it doesn't read the target, returning `0`.
///
/// Function skips any tokens after succesfully parsing.
fn parse_input(input: &str) -> Result<(Action, i32), String> {
    let mut tokens = input.split_ascii_whitespace();
    match tokens.next() {
        Some("help") => Ok((Action::Help, 0)),
        Some("inventory") => Ok((Action::Inventory, 0)),
        Some(token) => {
            let action = action_of_str(token)?;
            let id: i32 = match tokens.next() {
                Some(s) => match s.parse() {
                    Ok(n) => n,
                    _ => return Err(format!("Invalid target: {}", s)),
                },
                None => return Err(format!("Provide a target")),
            };
            Ok((action, id))
        }
        None => Err(format!("Provide an action")),
    }
}

/// Parses `str` into `Action`.
fn action_of_str(s: &str) -> Result<Action, String> {
    match s {
        "attack" => Ok(Action::Attack),
        "inventory" => Ok(Action::Inventory),
        "back" => Ok(Action::Back),
        "eat" => Ok(Action::Eat),
        "enter" => Ok(Action::Enter),
        "help" => Ok(Action::Help),
        "take" => Ok(Action::Take),
        "throw" => Ok(Action::Throw),
        _ => Err(color_print::cformat!("Unknown action: <red, bold>{}</>", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(parse_input("attack 4").unwrap(), (Action::Attack, 4));
        assert_eq!(
            parse_input("wtf").unwrap_err(),
            color_print::cformat!("Unknown action: <red, bold>wtf</>")
        );
    }
}
