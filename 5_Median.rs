// Question 5 Given a sorted array of integers, implement a function that returns the median of the array. 

fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] as f64 + arr[mid] as f64) / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6];
    
    println!("Median of arr: {}", find_median(&arr));
}
