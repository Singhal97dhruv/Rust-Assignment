//Question 6: Implement a function that finds the longest common prefix of a given set of strings.

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let mut prefix = String::new();
    let mut chars_iterators: Vec<_> = strings.iter().map(|s| s.chars()).collect();
    
    'outer: loop {
        let mut curr_char: Option<char> = None;
        for chars_iter in &mut chars_iterators {
            if let Some(c) = chars_iter.next() {
                if let Some(prev_char) = curr_char {
                    if c != prev_char {
                        break 'outer;
                    }
                } else {
                    curr_char = Some(c);
                }
            } else {
                break 'outer;
            }
        }
        if let Some(c) = curr_char {
            prefix.push(c);
        } else {
            break 'outer;
        }
    }
    
    prefix
}

fn main() {
    let strings = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    
    println!("Longest common prefix of strings: {}", longest_common_prefix(&strings));
}
