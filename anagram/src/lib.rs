fn lowercased_and_sorted(word: &str) -> (Vec<char>, Vec<char>) {
    let lowercased = word.to_lowercase().chars().collect::<Vec<_>>();
    let mut sorted = lowercased.clone();
    sorted.sort();
    (lowercased, sorted)
}

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    let (word, sorted_word) = lowercased_and_sorted(word);

    candidates
        .iter()
        .filter(|c| {
            let (candidate, sorted_candidate) = lowercased_and_sorted(c);
            sorted_candidate == sorted_word && candidate != word
        })
        .map(|c| *c)
        .collect()
}
