pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.to_uppercase()
        .chars()
        .filter(|&c| c.is_numeric() || c == 'X')
        .collect::<String>();

    let x_index = isbn.find('X');

    if isbn.len() != 10 || (x_index != None && x_index != Some(9)) {
        return false;
    }

    let sum = isbn.chars()
        .rev()
        .enumerate()
        .map(|(index, char)| {
            char.to_digit(10).unwrap_or(10) * (index + 1) as u32
        })
        .sum::<u32>();

    sum % 11 == 0
}
