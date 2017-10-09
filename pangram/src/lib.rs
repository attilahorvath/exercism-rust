pub fn is_pangram(sentence: &str) -> bool {
    let mut chars = sentence
        .to_lowercase()
        .chars()
        .filter(|&c| c >= 'a' && c <= 'z')
        .collect::<Vec<_>>();

    chars.sort();
    chars.dedup();

    chars.len() == ('z' as usize - 'a' as usize + 1)
}
