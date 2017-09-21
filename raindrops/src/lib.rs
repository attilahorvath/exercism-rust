pub fn raindrops(n: usize) -> String {
    if n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
        format!("{}{}{}", if n % 3 == 0 { "Pling" } else { "" },
                          if n % 5 == 0 { "Plang" } else { "" },
                          if n % 7 == 0 { "Plong" } else { "" })
    } else {
        n.to_string()
    }
}
