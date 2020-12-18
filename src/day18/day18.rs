fn extract_parenthesis_substr(s: &str) -> &str {
    let mut cnt = 1;

    if s.chars().next().unwrap() != '(' {
        panic!("string does not start with parenthesis");
    }

    for (i, c) in s.chars().enumerate().skip(1) {
        match c {
            x if x == '(' => cnt += 1,
            x if x == ')' => cnt -= 1,
            _ => (),
        }

        if cnt == 0 {
            return &s[1..i];
        }
    }

    panic!("could not find matching closing paren")
}

fn action(op: char, a: u64, b: u64) -> u64 {
    match op {
        x if x == '+' => a + b,
        x if x == '*' => a * b,
        _ => panic!("invalid op"),
    }
}

fn eval_greedy(s: &str) -> u64 {
    let mut ret = 0;
    let mut op = '+';

    let mut iter = s.chars().enumerate();
    while let Some(val) = iter.next() {
        match val.1 {
            x if x == ' ' => (),
            x if x == '*' => op = x,
            x if x == '+' => op = x,
            x if x == '(' => {
                let substr = extract_parenthesis_substr(&s[val.0..]);
                ret = action(op, ret, eval_greedy(substr));
                for _ in 0..(substr.len() + 1) {
                    iter.next();
                }
            }
            x => ret = action(op, ret, String::from(x).parse::<u64>().unwrap()),
        }
    }
    ret
}

fn eval_addition_precedence(s: &str) -> u64 {
    let mut ret = 1;
    let mut cur = 0;

    let mut iter = s.chars().enumerate();
    while let Some(val) = iter.next() {
        match val.1 {
            x if x == ' ' => (),
            x if x == '*' => {
                ret *= cur;
                cur = 0
            }
            x if x == '+' => (),
            x if x == '(' => {
                let substr = extract_parenthesis_substr(&s[val.0..]);
                cur += eval_addition_precedence(substr);
                for _ in 0..(substr.len() + 1) {
                    iter.next();
                }
            }
            x => cur += String::from(x).parse::<u64>().unwrap(),
        }
    }
    ret * cur
}

fn solve_1(filename: &str) -> u64 {
    common::read_file_linewise(filename)
        .iter()
        .fold(0u64, |acc, eq| acc + eval_greedy(eq))
}

fn solve_2(filename: &str) -> u64 {
    common::read_file_linewise(filename)
        .iter()
        .fold(0u64, |acc, eq| acc + eval_addition_precedence(eq))
}

fn main() {
    println!("Solution 1: {}", solve_1("src/day18/input.txt"));
    println!("Solution 1: {}", solve_2("src/day18/input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eval_greedy() {
        let lines = common::read_file_linewise("src/day18/input_test.txt");
        let expected = vec![71u64, 51, 26, 437, 12240, 13632];

        for i in 0..lines.len() {
            assert_eq!(eval_greedy(&lines[i]), expected[i]);
        }
    }

    #[test]
    fn test_eval_addition_precedence() {
        let lines = common::read_file_linewise("src/day18/input_test.txt");
        let expected = vec![231u64, 51, 46, 1445, 669060, 23340];

        for i in 0..lines.len() {
            assert_eq!(eval_addition_precedence(&lines[i]), expected[i]);
        }
    }
}
