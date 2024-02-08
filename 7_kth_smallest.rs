// Question 7 Implement a function that returns the kth smallest element in a given array.


fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;
    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

fn quickselect(arr: &mut [i32], low: usize, high: usize, k: usize) -> i32 {
    if k > 0 && k <= high - low + 1 {
        let pi = partition(arr, low, high);
        if pi - low == k - 1 {
            return arr[pi];
        } else if pi - low > k - 1 {
            return quickselect(arr, low, pi - 1, k);
        } else {
            return quickselect(arr, pi + 1, high, k - (pi - low + 1));
        }
    }
    -1 
}

fn kth_smallest(arr: &mut [i32], k: usize) -> i32 {
    let n = arr.len();
    quickselect(arr, 0, n - 1, k)
}

fn main() {
    let mut arr = vec![7, 10, 4, 3, 20, 15];
    let k = 3;
    let kth_smallest_element = kth_smallest(&mut arr, k);
    if kth_smallest_element != -1 {
        println!("The {}th smallest element is: {}", k, kth_smallest_element);
    } else {
        println!("Invalid value of k.");
    }
}
