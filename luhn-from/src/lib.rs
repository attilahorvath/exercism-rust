pub struct Luhn(String);

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(number: T) -> Self {
        Luhn(number.to_string())
    }
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let filtered = self.0.chars().filter(|&c| c != ' ').collect::<String>();

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
