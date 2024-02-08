// Question 9: Reverse a string in Rust

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}


fn main() {
    let original = "Shinning Star";
    
    let reversed = reverse_string(original);
    println!("Reversed string: {}", reversed);
    
}
