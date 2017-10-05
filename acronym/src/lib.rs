pub fn abbreviate(phrase: &str) -> String {
    let mut abbreviation = String::new();

    let mut last_alpha = false;
    let mut last_upper = false;

    for c in phrase.chars() {
        let alpha = c.is_alphabetic();
        let upper = c.is_uppercase();

        if (!last_alpha && alpha) || (last_alpha && !last_upper && upper) {
            abbreviation.push_str(&c.to_uppercase().to_string());
        }

        last_alpha = alpha;
        last_upper = upper;
    }

    abbreviation
}
