fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 1 {
        arr[len / 2] as f64
    } else {
        (arr[len / 2 - 1] as f64 + arr[len / 2] as f64) / 2.0
    }
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let median = find_median(&arr);
    println!("Median: {}", median);
}
