fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = vec![];
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    if i < arr1.len() {
        merged.extend_from_slice(&arr1[i..]);
    }

    if j < arr2.len() {
        merged.extend_from_slice(&arr2[j..]);
    }

    merged
}

fn main() {
    let arr1 = vec![1, 3, 5];
    let arr2 = vec![2, 4, 6];
    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);
}
