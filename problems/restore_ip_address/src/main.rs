use std::cmp::min;

fn restore_ip_addresses(s: String) -> Vec<String> {
    // Discard strings not large enough to form a valid IP address
    if s.len() < 4 {
        return vec![];
    }

    // Generate all possible IP address permutations from the given string
    let mut valid_permutations: Vec<String> = Vec::new();
    for i in 0..3 {
        for j in i + 1..min(6, s.len() - 2) {
            for k in j + 1..s.len() - 1 {
                let ip_address = format!("{}.{}.{}.{}", &s[0..=i], &s[i+1..=j], &s[j+1..=k], &s[k+1..]);
                if validate_ip_address(&ip_address) {
                    valid_permutations.push(ip_address);
                }
            }
        }
    }

    valid_permutations
}

// Validates an IP address
fn validate_ip_address(address: &str) -> bool {
    !address.split('.').any(|s| s.parse::<u32>().unwrap_or(256) > 255 || (s.chars().nth(0).unwrap_or('a') == '0' && s.len() > 1))
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::restore_ip_addresses;

    #[test]
    fn test_restore_ip_address_1() {
        let s = String::from("25525511135");
        let result = vec![
            "255.255.11.135",
            "255.255.111.35",
        ];
        assert_eq!(restore_ip_addresses(s), result);
    }

    #[test]
    fn test_restore_ip_address_2() {
        let s = String::from("0000");
        let result = vec!["0.0.0.0"];
        assert_eq!(restore_ip_addresses(s), result);
    }

    #[test]
    fn test_restore_ip_address_3() {
        let s = String::from("101023");
        let result = vec![
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3",
        ];
        assert_eq!(restore_ip_addresses(s), result);
    }

    #[test]
    fn test_restore_ip_address4() {
        let s = String::from("101@023");
        let result: Vec<String> = vec![];
        assert_eq!(restore_ip_addresses(s), result);
    }
}