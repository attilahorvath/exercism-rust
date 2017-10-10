const ALPHABET_LENGTH: u8 = 'z' as u8 - 'a' as u8 + 1;

fn rotate_char(c: char, base: u8, key: u8) -> char {
    (base + (c as u8 - base + key) % ALPHABET_LENGTH) as char
}

pub fn rotate(text: &str, key: u8) -> String {
    text.chars()
        .map(|c| if c.is_lowercase() {
            rotate_char(c, 'a' as u8, key)
        } else if c.is_uppercase() {
            rotate_char(c, 'A' as u8, key)
        } else {
            c
        })
        .collect()
}
