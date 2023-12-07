use regex::Regex;

fn main() {
    // Read input from file
    let input = std::fs::read_to_string("input.txt").unwrap();
    // itterate over lines, and use regex to return all numbers from the line
    let mut numbers: Vec<String> = vec![];
    for line in input.lines() {
        numbers.push(find_numbers(line));
    }
    // sum all numbers
    let sum: u32 = numbers
        .iter()
        .filter_map(|number| number.parse::<u32>().ok())
        .sum();

    for i in 0..numbers.len() {
        println!("{}: {}", i, numbers[i]);   
    }
    println!("Sum: {}", sum);
}

fn find_numbers(input: &str) -> String {
    let re = Regex::new(r"(\d)").unwrap();
    let mut numbers: Vec<String> = vec![];
    let fund_numbers = re.captures_iter(input);
    
    let matches: Vec<_> = fund_numbers.collect();
    if matches.len() == 0 {
        return "".to_string();
    } else if matches.len() == 1 {
        numbers.push(matches[0][0].to_string());
        numbers.push(matches[0][0].to_string());
    } else {
        numbers.push(matches[0][0].to_string());
        numbers.push(matches[matches.len()-1][0].to_string());
    }
    return numbers.join("");
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test happy case
    #[test]
    fn find_two_numbers() {
        let line_input = "123aoeu4aoeu56auoe789";
        let expected = "19";
        assert!(find_numbers(line_input) == expected);
    }
    // Test no numbers
    #[test]
    fn find_no_numbers() {
        let line_input = "aoeuaoeuaoeu";
        let expected = "";
        assert!(find_numbers(line_input) == expected);
    }
    // Test one number
    #[test]
    fn fined_one_number() {
        let line_input = "aoeuaoeuaoeu1aoeuaoeuaoeu";
        let expected = "11";
        assert!(find_numbers(line_input) == expected);
    }
}
