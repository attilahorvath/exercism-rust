fn encode_char(c: char) -> char {
    if c.is_alphabetic() {
        return ('a' as u8 + ('z' as u8 - c as u8)) as char;
    }

    c
}

pub fn encode(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|&c| (c >= 'a' && c <= 'z') || c.is_numeric())
        .map(encode_char)
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn decode(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphanumeric())
        .map(encode_char)
        .collect()
}
