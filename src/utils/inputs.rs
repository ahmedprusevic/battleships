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

pub fn parse_input(player_input: String) -> Option<(String, i8)> {
    let mut chars = player_input.chars();
    let letter_part = chars.next()?;
    let first_number_part = chars.next()?.to_digit(10)?;
    let second_number_part = chars.next().and_then(|c| c.to_digit(10)).unwrap_or(100);
    let player_selected_number = if second_number_part == 100 {
        first_number_part as i8
    } else {
        (first_number_part * 10 + second_number_part) as i8
    };
    Some((letter_part.to_string(), player_selected_number))
}
