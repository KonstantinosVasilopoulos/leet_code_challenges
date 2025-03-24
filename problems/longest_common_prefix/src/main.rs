fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    // Iterate over the strings in the vector, one character at a time
    let mut common_prefix = String::new();
    let mut i = 0usize;
    loop {
        let mut current_char: Option<char> = None;

        // Check the i-th character of each string
        for str in &strs {
            if let Some(c) = str.chars().nth(i) {
                if current_char.is_none() {
                    current_char = Some(c);
                } else if current_char.unwrap() != c {
                    return common_prefix;
                }
            } else {
                return common_prefix;
            }
        }

        common_prefix.push(current_char.unwrap());
        i += 1;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_prefix() {
        let strs = vec![
            String::from("flowers"),
            String::from("flow"),
            String::from("flight"),
        ];
        assert_eq!(longest_common_prefix(strs), String::from("fl"));
    }

    #[test]
    fn test_no_common_prefix() {
        let strs = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];
        assert_eq!(longest_common_prefix(strs), String::from(""));
    }
}
