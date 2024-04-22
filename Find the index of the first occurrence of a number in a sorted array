fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let target = 3;
    if let Some(index) = find_first_occurrence(&arr, target) {
        println!("First occurrence of {} is at index {}.", target, index);
    } else {
        println!("{} not found in the array.", target);
    }
}
