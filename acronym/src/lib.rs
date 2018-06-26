pub fn abbreviate(phrase: &str) -> String {
    let mut abbreviation = String::new();

    let mut prev_alpha = false;
    let mut prev_upper = false;

    for c in phrase.chars() {
        let alpha = c.is_alphabetic();
        let upper = c.is_uppercase();

        if (!prev_alpha && alpha) || (prev_alpha && !prev_upper && upper) {
            for i in c.to_uppercase() {
                abbreviation.push(i);
            }
        }

        prev_alpha = alpha;
        prev_upper = upper;
    }

    abbreviation
}
