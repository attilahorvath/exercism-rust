pub fn encrypt(text: &str) -> String {
    let text = text.to_lowercase()
        .chars()
        .filter(|c| c.is_lowercase())
        .collect::<Vec<_>>();

    if text.is_empty() {
        return String::new();
    }

    let rows = (text.len() as f64).sqrt() as usize;
    let cols = (text.len() as f64 / rows as f64).ceil() as usize;

    let mut encrypted = String::with_capacity(rows * cols);

    for r in 0..cols {
        for c in 0..cols {
            let index = c * cols + r;

            if index < text.len() {
                encrypted.push(text[index]);
            }
        }

        if r < cols - 1 {
            encrypted.push(' ');
        }
    }

    encrypted
}
