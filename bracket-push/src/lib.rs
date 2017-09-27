pub struct Brackets(String);

impl<'a> From<&'a str> for Brackets {
    fn from(string: &str) -> Self {
        Brackets(string.into())
    }
}

impl Brackets {
    pub fn are_balanced(&self) -> bool {
        let mut stack = Vec::new();

        for c in self.0.chars() {
            match c {
                '[' | '{' | '(' => {
                    stack.push(c);
                },
                ']' | '}' | ')' => {
                    match stack.pop() {
                        Some(x) => {
                            if c == ']' && x != '[' ||
                               c == '}' && x != '{' ||
                               c == ')' && x != '(' {
                                return false
                            }
                        },
                        None => return false
                    }
                },
                _ => ()
            }
        }

        stack.len() == 0
    }
}
