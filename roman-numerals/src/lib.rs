use std::fmt;

const NUMERALS: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub struct Roman(u32);

impl From<u32> for Roman {
    fn from(number: u32) -> Self {
        Roman(number)
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut number = self.0;

        while number > 0 {
            let &(n, c) = NUMERALS.iter().find(|&&(i, _)| number >= i).unwrap();
            write!(f, "{}", c)?;
            number -= n;
        }

        Ok(())
    }
}
