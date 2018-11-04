extern crate rand;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.len() == 0 || !key.chars().all(char::is_lowercase) {
        return None;
    }

    let encoded = s
        .chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| {
            let offset = k as u8 - 'a' as u8;
            let mut shifted = c as u8 + offset;

            if shifted > 'z' as u8 {
                shifted -= 'z' as u8 - 'a' as u8 + 1;
            }

            shifted as char
        }).collect();

    Some(encoded)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.len() == 0 || !key.chars().all(char::is_lowercase) {
        return None;
    }

    let inverse_key = key
        .chars()
        .map(|k| {
            if k == 'a' {
                'a'
            } else {
                ('a' as u8 + ('z' as u8 - k as u8) + 1) as char
            }
        }).collect::<String>();

    encode(&inverse_key, s)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = thread_rng()
        .sample_iter(&Uniform::new_inclusive('a' as u8, 'z' as u8))
        .take(100)
        .map(|c| c as char)
        .collect::<String>();

    let encoded = encode(&key, s).unwrap();

    (key, encoded)
}
