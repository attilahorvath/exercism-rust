pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb = list
        .windows(2)
        .map(|pair| format!("For want of a {} the {} was lost.", pair[0], pair[1]))
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(first) = list.get(0) {
        if !proverb.is_empty() {
            proverb.push('\n');
        }

        proverb.push_str(&format!("And all for the want of a {}.", first));
    }

    proverb
}
