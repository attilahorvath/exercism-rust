use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input
        .iter()
        .flat_map(|(&score, chars)| {
            chars.iter().flat_map(|&c| c.to_lowercase()).map(move |c| {
                (c, score)
            })
        })
        .collect()
}
