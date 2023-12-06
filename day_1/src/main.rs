use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Test of date result: {}", re.is_match("2020-01-01"));
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test happy case
    #[test]
    fn test_regex() {
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        assert!(re.is_match("2020-01-01"));
    }

    // Test sad case
    #[test]
    fn test_regex_fail() {
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        assert_eq!(re.is_match("2020-01-01-01"), false);
    }
}
