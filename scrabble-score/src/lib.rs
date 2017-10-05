pub fn score(word: &str) -> u32 {
    let mut score = 0;

    for c in word.chars() {
        score += match c.to_lowercase().to_string().as_ref() {
            "a" | "e" | "i" | "o" | "u" | "l" | "n" | "r" | "s" | "t" => 1,
            "d" | "g" => 2,
            "b" | "c" | "m" | "p" => 3,
            "f" | "h" | "v" | "w" | "y" => 4,
            "k" => 5,
            "j" | "x" => 8,
            "q" | "z" => 10,
            _ => 0
        }
    }

    score
}
