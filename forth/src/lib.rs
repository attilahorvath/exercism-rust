use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;
type ForthFunction = Fn(&mut Vec<Value>) -> ForthResult;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Box<ForthFunction>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn pop(stack: &mut Vec<Value>) -> Result<Value, Error> {
    match stack.pop() {
        Some(x) => Ok(x),
        None => Err(Error::StackUnderflow),
    }
}

fn binary_operation<T>(f: T) -> Box<ForthFunction>
    where T: 'static + Fn(Value, Value) -> Result<Value, Error> {
    Box::new(move |stack| {
        let b = pop(stack)?;
        let a = pop(stack)?;

        match f(a, b) {
            Ok(x) => stack.push(x),
            Err(e) => return Err(e),
        }

        Ok(())
    })
}

fn dup(stack: &mut Vec<Value>) -> ForthResult {
    let a = pop(stack)?;

    stack.push(a);
    stack.push(a);

    Ok(())
}

fn drop(stack: &mut Vec<Value>) -> ForthResult {
    pop(stack)?;

    Ok(())
}

fn swap(stack: &mut Vec<Value>) -> ForthResult {
    let b = pop(stack)?;
    let a = pop(stack)?;

    stack.push(b);
    stack.push(a);

    Ok(())
}

fn over(stack: &mut Vec<Value>) -> ForthResult {
    let b = pop(stack)?;
    let a = pop(stack)?;

    stack.push(a);
    stack.push(b);
    stack.push(a);

    Ok(())
}

impl Forth {
    pub fn new() -> Self {
        let mut words: HashMap<String, Box<ForthFunction>> = HashMap::new();

        words.insert("+".to_string(), binary_operation(|a, b| Ok(a + b)));
        words.insert("-".to_string(), binary_operation(|a, b| Ok(a - b)));
        words.insert("*".to_string(), binary_operation(|a, b| Ok(a * b)));
        words.insert("/".to_string(), binary_operation(|a, b| {
            match b {
                0 => Err(Error::DivisionByZero),
                _ => Ok(a / b),
            }
        }));

        words.insert("DUP".to_string(), Box::new(dup));
        words.insert("DROP".to_string(), Box::new(drop));
        words.insert("SWAP".to_string(), Box::new(swap));
        words.insert("OVER".to_string(), Box::new(over));

        Forth {
            stack: Vec::new(),
            words,
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        for s in input.to_uppercase().split(|c: char| c.is_whitespace() || c.is_control()) {
            match self.words.get(s) {
                Some(f) => {
                    if let Err(e) = f(&mut self.stack) {
                        return Err(e);
                    }
                }
                None => self.stack.push(s.parse().unwrap()),
            }
        }

        Ok(())
    }
}
