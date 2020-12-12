fn is_valid(preamble: &[u128], x: u128) -> bool {
    for i in 0..preamble.len() {
        for j in i + 1..preamble.len() {
            if preamble[i] + preamble[j] == x {
                return true;
            }
        }
    }
    return false;
}

fn check_sequence(numbers: &Vec<u128>, preamble_length: usize) -> (bool, u128) {
    for i in 0..numbers.len() - preamble_length {
        let x = numbers[i + preamble_length];
        if !is_valid(&numbers[i..i + preamble_length], x) {
            return (false, x);
        }
    }
    return (true, 0);
}

fn find_encryption(numbers: &Vec<u128>, x: u128) -> u128 {
    for i in 0..numbers.len() {
        let mut y = 0;
        for j in i..numbers.len() {
            y += numbers[j];
            if y > x {
                break;
            } else if y == x {
                let slice = &numbers[i..j + 1];
                return slice.iter().min().unwrap() + slice.iter().max().unwrap();
            }
        }
    }
    panic!("Could not break encryption");
}

fn main() {
    let numbers = common::parse_file_linewise_as::<u128>("src/day09/input.txt");
    let (_, x) = check_sequence(&numbers, 25);
    println!("Fist unbuildable number: {}", x);
    println!("Encryption weakness: {}", find_encryption(&numbers, x));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        let preamble: Vec<u128> = (1u128..26u128).collect();
        assert_eq!(is_valid(&preamble, 26), true);
        assert_eq!(is_valid(&preamble, 49), true);
        assert_eq!(is_valid(&preamble, 100), false);
        assert_eq!(is_valid(&preamble, 50), false);
    }

    #[test]
    fn test_check_sequence() {
        let numbers = common::parse_file_linewise_as::<u128>("src/day09/input_test.txt");
        let (valid, x) = check_sequence(&numbers, 5);
        assert_eq!(valid, false);
        assert_eq!(x, 127);
    }

    #[test]
    fn test_find_encryption() {
        let numbers = common::parse_file_linewise_as::<u128>("src/day09/input_test.txt");
        assert_eq!(find_encryption(&numbers, 127), 62);
    }
}
