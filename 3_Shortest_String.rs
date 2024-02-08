// Question 3: Given a string of words, implement a function that returns the shortest word in the string.


fn shortest_word(sentence: &str) -> Option<&str> {
    sentence.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let sentence = "Sam is naugtiest among his friends";
    match shortest_word(sentence) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("No words found in the sentence"),
    }
}
