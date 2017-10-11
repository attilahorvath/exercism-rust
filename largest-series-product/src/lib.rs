pub fn lsp(digits: &str, length: u32) -> Result<u32, &'static str> {
    if digits.chars().any(|c| !c.is_numeric()) {
        return Err("Invalid input");
    }

    if length == 0 {
        return Ok(1);
    }

    let max = digits
        .chars()
        .collect::<Vec<_>>()
        .windows(length as usize)
        .map(|w| w.iter().fold(1, |acc, i| acc * i.to_digit(10).unwrap()))
        .max();

    match max {
        Some(m) => Ok(m),
        None => Err("Invalid length"),
    }
}
