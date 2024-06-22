fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == i32::MIN && divisor == -1 {
        return i32::MAX;
    } else if dividend == i32::MAX && divisor == -1 {
        return i32::MIN + 1;
    }

    dividend / divisor
}

fn main() {
}

#[cfg(test)]
mod test {
    use crate::divide;

    #[test]
    fn test_simple() {
        assert_eq!(divide(10, 3), 3);
    }

    #[test]
    fn test_negative() {
        assert_eq!(divide(7, -3), -2);
    }

    #[test]
    fn test_extreme() {
        assert_eq!(divide(i32::MAX, -1), i32::MAX)
    }
}
