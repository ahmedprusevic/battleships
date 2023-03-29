use std::io;
use std::io::BufRead;

pub fn read_player_input() -> String {
    let mut player_input = String::new();
    io::stdin()
        .lock()
        .read_line(&mut player_input)
        .expect("Failed to read line!");
    player_input.trim().to_string()
}
