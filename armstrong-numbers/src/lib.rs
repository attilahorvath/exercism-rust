pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    digits
        .iter()
        .map(|digit| digit.pow(digits.len() as u32))
        .sum::<u32>()
        == num
}
