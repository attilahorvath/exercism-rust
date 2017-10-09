pub fn hamming_distance(first: &str, second: &str) -> Result<usize, &'static str> {
    if first.len() != second.len() {
        return Err("Sequences must be of equal length");
    }

    Ok(
        first
            .chars()
            .zip(second.chars())
            .filter(|&(a, b)| a != b)
            .count(),
    )
}
