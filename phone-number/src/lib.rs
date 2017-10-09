pub fn number(phone_number: &str) -> Option<String> {
    let mut digits = phone_number
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();

    if digits.starts_with('1') && digits.len() == 11 {
        digits.remove(0);
    }

    if digits.len() != 10 {
        return None;
    }

    Some(digits)
}

pub fn area_code(phone_number: &str) -> Option<String> {
    number(phone_number).map(|n| n.chars().take(3).collect())
}

pub fn pretty_print(phone_number: &str) -> String {
    match number(phone_number) {
        Some(n) => {
            format!(
                "({}) {}-{}",
                n.get(0..3).unwrap(),
                n.get(3..6).unwrap(),
                n.get(6..10).unwrap()
            )
        }
        None => "invalid".to_string(),
    }
}
