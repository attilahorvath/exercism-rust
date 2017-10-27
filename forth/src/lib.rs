use std::collections::HashMap;
use std::rc::Rc;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;
type ForthFunction = Fn(&mut Forth) -> ForthResult;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Rc<ForthFunction>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn binary_operation<T>(f: T) -> Rc<ForthFunction>
where
    T: 'static + Fn(Value, Value) -> Result<Value, Error>,
{
    Rc::new(move |forth| {
        let b = forth.pop()?;
        let a = forth.pop()?;

        match f(a, b) {
            Ok(x) => forth.push(x),
            Err(e) => return Err(e),
        }

        Ok(())
    })
}

impl Forth {
    pub fn new() -> Self {
        let mut words: HashMap<String, Rc<ForthFunction>> = HashMap::new();

        words.insert("+".to_string(), binary_operation(|a, b| Ok(a + b)));
        words.insert("-".to_string(), binary_operation(|a, b| Ok(a - b)));
        words.insert("*".to_string(), binary_operation(|a, b| Ok(a * b)));
        words.insert(
            "/".to_string(),
            binary_operation(|a, b| match b {
                0 => Err(Error::DivisionByZero),
                _ => Ok(a / b),
            }),
        );

        words.insert("DUP".to_string(), Rc::new(Forth::dup));
        words.insert("DROP".to_string(), Rc::new(Forth::drop));
        words.insert("SWAP".to_string(), Rc::new(Forth::swap));
        words.insert("OVER".to_string(), Rc::new(Forth::over));

        Forth {
            stack: Vec::new(),
            words,
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let input = input.to_uppercase();

        let mut word_definition = false;
        let mut definition_tokens = Vec::new();

        for token in input
            .split(|c: char| c.is_whitespace() || c.is_control())
            .filter(|s| !s.is_empty())
        {
            if !word_definition && token == ":" {
                word_definition = true;
            } else if word_definition {
                if token != ";" {
                    definition_tokens.push(token);
                } else {
                    let mut iter = definition_tokens.clone().into_iter();

                    let word = match iter.next() {
                        Some(w) => w.to_string(),
                        None => return Err(Error::InvalidWord),
                    };

                    if word.chars().any(|c| c.is_numeric()) {
                        return Err(Error::InvalidWord);
                    }

                    let definition = iter.collect::<Vec<_>>().join(" ");

                    self.words.insert(
                        word,
                        Rc::new(move |forth| forth.eval(&definition)),
                    );

                    definition_tokens.clear();
                    word_definition = false;
                }
            } else {
                match self.words.get(token).cloned() {
                    Some(f) => f(self)?,
                    None => {
                        match token.parse() {
                            Ok(n) => self.push(n),
                            Err(_) => return Err(Error::UnknownWord),
                        }
                    }
                }
            }
        }

        if word_definition {
            return Err(Error::InvalidWord);
        }

        Ok(())
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Result<Value, Error> {
        match self.stack.pop() {
            Some(x) => Ok(x),
            None => Err(Error::StackUnderflow),
        }
    }

    fn dup(&mut self) -> ForthResult {
        let a = self.pop()?;

        self.push(a);
        self.push(a);

        Ok(())
    }

    fn drop(&mut self) -> ForthResult {
        self.pop()?;

        Ok(())
    }

    fn swap(&mut self) -> ForthResult {
        let b = self.pop()?;
        let a = self.pop()?;

        self.push(b);
        self.push(a);

        Ok(())
    }

    fn over(&mut self) -> ForthResult {
        let b = self.pop()?;
        let a = self.pop()?;

        self.push(a);
        self.push(b);
        self.push(a);

        Ok(())
    }
}
