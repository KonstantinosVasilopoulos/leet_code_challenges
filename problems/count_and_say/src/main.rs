fn count_and_say(n: i32) -> String {
    if n == 1 {
        return String::from("1");
    }

    let previous_rle = count_and_say(n - 1);
    let mut rle = String::new();
    let mut previous_char: Option<char> = None;
    let mut i: u16 = 1;
    for c in previous_rle.chars() {
        if previous_char.is_none() {
            previous_char = Some(c);
            continue;
        }

        if previous_char.unwrap() == c {
            i += 1;
        } else {
            rle.push_str(&i.to_string());
            rle.push_str(&previous_char.unwrap().to_string());
            i = 1;
            previous_char = Some(c);
        }

    }

    rle.push_str(&i.to_string());
    rle.push_str(&previous_char.unwrap().to_string());

    rle
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_1() {
        assert_eq!(count_and_say(1), String::from("1"));
    }

    #[test]
    fn test_count_and_say_4() {
        assert_eq!(count_and_say(4), String::from("1211"));
    }

    #[test]
    fn test_count_and_say_10() {
        assert_eq!(count_and_say(10), String::from("13211311123113112211"));
    }
}
