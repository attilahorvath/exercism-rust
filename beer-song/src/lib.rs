use std::fmt::Display;

trait StringExtensions
where
    Self: Display,
{
    fn capitalize(&self) -> String {
        let string = self.to_string();
        let mut chars = string.chars();

        if let Some(first) = chars.next() {
            first.to_uppercase().chain(chars).collect()
        } else {
            String::new()
        }
    }

    fn pluralize(&self, n: u32) -> String {
        if n == 0 {
            format!("no more {}s", self)
        } else if n == 1 {
            format!("1 {}", self)
        } else {
            format!("{} {}s", n, self)
        }
    }
}

impl<T> StringExtensions for T where T: Display {}

fn first_line(n: u32) -> String {
    format!(
        "{bottles} of beer on the wall, {bottles} of beer.",
        bottles = "bottle".pluralize(n)
    ).capitalize()
}

fn first_part_of_second_line(n: u32) -> String {
    if n == 0 {
        "Go to the store and buy some more".to_string()
    } else {
        format!(
            "Take {} down and pass it around",
            if n == 1 { "it" } else { "one" }
        )
    }
}

fn second_line(n: u32) -> String {
    let next = if n == 0 { 99 } else { n - 1 };

    format!(
        "{}, {} of beer on the wall.",
        first_part_of_second_line(n),
        "bottle".pluralize(next)
    )
}

pub fn verse(n: u32) -> String {
    format!("{}\n{}\n", first_line(n), second_line(n))
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
