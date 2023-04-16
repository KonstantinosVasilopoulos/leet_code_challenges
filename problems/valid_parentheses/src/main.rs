fn is_valid(s: String) -> bool {
    // Create a stack to store the parentheses
    let mut stack: Vec<char> = Vec::with_capacity(s.len());

    // Iterate over the input string while checking the each character
    let check_stack = |stack: &mut Vec<char>, expect: char| stack.pop().unwrap_or('0') == expect;
    for parentheses in s.chars() {
        let status: bool = match parentheses {
            // Push opening parentheses into the stack
            '(' | '[' | '{' => {
                stack.push(parentheses);
                true
            },

            // Check that closing parentheses are after their opening counterpart
            ')' => check_stack(&mut stack, '('),
            ']' => check_stack(&mut stack, '['),
            '}' => check_stack(&mut stack, '{'),
            _ => false,
        };

        if !status {
            return false;
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::is_valid;

    fn test_input(input: String, expect: bool) -> bool {
        is_valid(input) == expect
    }

    #[test]
    fn parentheses_only() {
        assert!(test_input(String::from("()"), true))
    }

    #[test]
    fn all_parentheses() {
        assert!(test_input(String::from("()[]{}"), true))
    }

    #[test]
    fn invalid_closing_parentheses() {
        assert!(test_input(String::from("(]"), false))
    }
}

fn main() {
    println!("{:?}", is_valid(String::from("()[]{}")));
}
