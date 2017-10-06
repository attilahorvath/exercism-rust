use std::collections::HashMap;

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let phrase = phrase.to_lowercase();
    let words = phrase.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|w| w.to_string())
        .collect::<Vec<_>>();

    let mut word_counts = HashMap::new();

    for word in &words {
        if word_counts.contains_key(word) {
            continue;
        }

        let count = words.iter().filter(|&w| { w == word }).count() as u32;

        word_counts.insert(word.clone(), count);
    }

    word_counts
}
