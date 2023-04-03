pub fn convert_string_to_number(input_string: &str) -> (u32, char) {
    let parts: Vec<&str> = input_string.split(" - ").collect();

    let last_two_chars = parts[1][1..].to_string();
    let last_char = parts[1].chars().nth(0).unwrap();

    let number = last_two_chars.parse::<u32>().unwrap();

    (number, last_char)
}
