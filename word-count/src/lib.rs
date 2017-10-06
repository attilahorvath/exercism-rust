use std::collections::HashMap;

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    phrase.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|w| w.to_string())
        .fold(HashMap::new(), |mut acc, w| {
            *acc.entry(w).or_insert(0) += 1;
            acc
        })
}
