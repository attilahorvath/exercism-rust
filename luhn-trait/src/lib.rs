pub trait ValidLuhn: ToString {
    fn valid_luhn(&self) -> bool {
        let filtered = self.to_string()
            .chars()
            .filter(|&c| c != ' ')
            .collect::<String>();

        if filtered.len() <= 1 || filtered.chars().any(|c| !c.is_numeric()) {
            return false;
        }

        filtered
            .chars()
            .rev()
            .enumerate()
            .map(|(index, c)| {
                let digit = c.to_digit(10).unwrap();

                if index % 2 == 0 {
                    digit
                } else {
                    let doubled = digit * 2;

                    doubled - (if doubled > 9 { 9 } else { 0 })
                }
            })
            .sum::<u32>() % 10 == 0
    }
}

impl<T> ValidLuhn for T
where
    T: ToString,
{
}
