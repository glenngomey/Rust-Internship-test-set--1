fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = strings[0].clone();
    for s in &strings[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
}

fn main() {
    let strings = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", prefix);
}
