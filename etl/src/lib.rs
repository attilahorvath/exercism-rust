use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut output = BTreeMap::new();

    for (&score, chars) in input {
        for char in chars.iter().collect::<String>().to_lowercase().chars() {
            output.insert(char, score);
        }
    }

    output
}
