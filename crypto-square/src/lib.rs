pub fn encrypt(text: &str) -> String {
    let text = text.to_lowercase()
        .chars()
        .filter(|c| c.is_lowercase())
        .collect::<Vec<_>>();

    let size = (text.len() as f64).sqrt().ceil() as usize;

    (0..size)
        .map(|r| {
            (0..size)
                .filter_map(|c| text.get(c * size + r))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
