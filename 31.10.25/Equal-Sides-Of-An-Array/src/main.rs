/*
You are going to be given an array of integers. Your job is to take that array and find an index 
N where the sum of the integers to the left of N is equal to the sum of the integers to the right of N.

If there is no index that would make this happen, return None.

Input

An integer array of length 0 < arr < 1000. The numbers in the array can be any integer positive or negative.
Output

The lowest index N where the side to the left of N is equal to the side to the right of N. 
If you do not find an index that fits these rules, then you will return -1.
Note

If you are given an array with multiple answers, return the lowest correct index. */



fn find_even_index(arr: &[i32]) -> Option<usize> {
    let (mut l_sum, mut r_sum) = (0i32, arr.iter().sum());

    for i in 0..arr.len() {
        r_sum -= arr[i];
        if l_sum == r_sum {
            return Some(i);
        }
        l_sum += arr[i];
    }
    None
}

fn main() {
    println!("Result: {}", find_even_index(&[1, 2, 3, 4, 3, 2, 1]).unwrap_or_else(|| usize::MAX));
    println!("Result: {}", find_even_index(&[1, 100, 50, -51, 1, 1]).unwrap_or_else(|| usize::MAX));
    println!("Result: {}", find_even_index(&[1, 2, 3, 4, 5, 6]).unwrap_or_else(|| usize::MAX));
    println!("Result: {}", find_even_index(&[20, 10, 30, 10, 10, 15, 35]).unwrap_or_else(|| usize::MAX));
    println!("Result: {}", find_even_index(&(1..100).collect::<Vec<_>>()).unwrap_or_else(|| usize::MAX));
}
