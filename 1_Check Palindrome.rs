// Question1 Implement a function that checks whether a given string is a palindrome or not.



fn isPalindrome(s: &str)->bool{
    let chars: Vec<char>=s.chars().collect();
    let len=chars.len();
    for i in 0..len/2{
        if chars[i]!=chars[len-i-1]{
            return false;
        }
    }
    true
}

fn main(){
    let examples= vec!["naman","chaman","rotor","hello"];
    for case in examples{
        println!("Is '{}' a palindrome? {}",case,isPalindrome(case));
    }
}