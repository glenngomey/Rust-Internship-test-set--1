fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    s.chars().eq(s.chars().rev())
}

fn main() {
    let s = "racecar";
    if is_palindrome(s) {
        println!("{} is a palindrome.", s);
    } else {
        println!("{} is not a palindrome.", s);
    }
}
