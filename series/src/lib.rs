pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut series = Vec::new();

    let limit = if len > digits.len() {
        0
    } else {
        digits.len() - len + 1
    };

    for i in 0..limit {
        series.push(digits[i..(i + len)].to_string());
    }

    series
}
