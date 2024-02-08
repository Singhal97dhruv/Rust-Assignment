// Question 4 Implement a function that checks whether a given number is prime or not.

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let test_cases = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &num in test_cases.iter() {
        println!("Is {} prime? {}", num, is_prime(num));
    }
}
