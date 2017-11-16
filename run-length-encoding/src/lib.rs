use std::iter;
use std::str;

fn peek_while<F>(iter: &mut iter::Peekable<str::Chars>, f: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut s = String::new();

    while let Some(&c) = iter.peek() {
        if f(c) {
            s.push(c);
            iter.next();
        } else {
            break;
        }
    }

    s
}

pub fn encode(s: &str) -> String {
    let mut iter = s.chars().peekable();
    let mut encoded = String::new();

    while let Some(&c) = iter.peek() {
        let i = peek_while(&mut iter, |a| a == c).len();

        if i > 1 {
            encoded.push_str(&i.to_string());
        }

        encoded.push(c);
    }

    encoded
}

pub fn decode(s: &str) -> String {
    let mut iter = s.chars().peekable();
    let mut decoded = String::new();

    let mut i = 1;

    while let Some(&c) = iter.peek() {
        if c.is_numeric() {
            i = peek_while(&mut iter, |a| a.is_numeric()).parse().unwrap();
        } else {
            decoded.push_str(&c.to_string().repeat(i));
            iter.next();
            i = 1;
        }
    }

    decoded
}
