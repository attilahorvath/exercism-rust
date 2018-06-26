enum SentenceType {
    Unknown,
    Simple,
    Question,
}

enum Tone {
    Unknown,
    Normal,
    Shout,
}

pub fn reply(message: &str) -> &str {
    let mut message_format = (SentenceType::Unknown, Tone::Unknown);

    for c in message.chars().filter(|&c| !c.is_whitespace()) {
        message_format = (
            (match message_format.0 {
                SentenceType::Unknown => SentenceType::Simple,
                SentenceType::Simple if c == '?' => SentenceType::Question,
                SentenceType::Question if c != '?' => SentenceType::Simple,
                _ => message_format.0,
            }),
            (match message_format.1 {
                Tone::Unknown if c.is_uppercase() => Tone::Shout,
                _ if c.is_lowercase() => Tone::Normal,
                _ => message_format.1,
            }),
        );
    }

    match message_format {
        (SentenceType::Simple, Tone::Shout) => "Whoa, chill out!",
        (SentenceType::Simple, _) => "Whatever.",
        (SentenceType::Question, Tone::Shout) => "Calm down, I know what I'm doing!",
        (SentenceType::Question, _) => "Sure.",
        _ => "Fine. Be that way!",
    }
}
