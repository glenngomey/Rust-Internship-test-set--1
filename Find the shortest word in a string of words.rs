fn shortest_word(s: &str) -> &str {
    s.split_whitespace().min_by_key(|word| word.len()).unwrap_or("")
}

fn main() {
    let s = "apple orange banana";
    let shortest = shortest_word(s);
    println!("Shortest word: {}", shortest);
}
