pub fn hello(name: Option<&str>) -> String {
    match name {
        None    => "Hello, World!".to_string(),
        Some(s) => format!("Hello, {}!", s).to_string(),
    }
}
