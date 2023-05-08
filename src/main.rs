use enable_ansi_support::enable_ansi_support;
use rust_embed::RustEmbed;

mod gamedata;
mod communication;
mod game;


#[derive(RustEmbed, Debug)]
#[folder = "texts/"]
pub struct Texts;

fn main() {
    let _ansi_supported = enable_ansi_support().is_ok();


    communication::print("welcome.txt");
    let mut game = None;
    loop {
        match communication::many_commands_with_exit(&vec![
            "Start".to_string(),
            "Exit".to_string(),
        ], true) {
            Some(choice) => {
                if choice == 0 {
                    game = match game {
                        Some(game) => game::resume(game),
                        None => game::new(),
                    }
                } else {
                    break;
                }
            }
            None => break,
        }
        communication::print_colored("exit_game.txt", communication::Colors::Red);
        if communication::yesno() {
            return;
        }
    }
}
