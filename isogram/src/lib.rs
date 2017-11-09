pub fn check(phrase: &str) -> bool {
    let chars = phrase
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    let mut unique_chars = chars.clone();
    unique_chars.sort();
    unique_chars.dedup();

    chars.len() == unique_chars.len()
}
