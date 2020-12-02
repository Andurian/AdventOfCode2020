use std::fs;

struct Policy {
    letter: char,
    min: i32,
    max: i32,
}

impl Policy {
    fn from_string(s: &str) -> Policy {
        let tokens: Vec<&str> = s.split(|c| c == ' ' || c == '-').collect();

        Policy {
            letter: tokens[2].chars().next().unwrap(),
            min: tokens[0].parse().unwrap(),
            max: tokens[1].parse().unwrap(),
        }
    }

    fn matches(&self, password: &str) -> bool {
        let mut cnt: i32 = 0;

        for c in password.chars(){
            if c == self.letter{
                cnt += 1;
            }
        }

        return cnt >= self.min && cnt <= self.max;
    }

    fn matches_new(&self, password: &str) -> bool {
        let mut cnt: i32 = 0;

        let i1 = (self.min - 1) as usize;
        let i2 = (self.max - 1) as usize;

        let chars: Vec<char> = password.chars().collect();

        if chars[i1] == self.letter{
            cnt += 1;
        }

        if chars[i2] == self.letter{
            cnt += 1;
        }

        return cnt == 1;
    }
}

fn main() {
    let input = fs::read_to_string("src/day02/input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();
    
    let mut valid_passwords_old: i32 = 0;
    let mut valid_passwords_new: i32 = 0;

    for line in lines {
        let tokens: Vec<&str> = line.split(": ").collect();
        let policy = Policy::from_string(&tokens[0]);

        if policy.matches(&tokens[1]) {
            valid_passwords_old += 1;
        }

        if policy.matches_new(&tokens[1]) {
            valid_passwords_new += 1;
        }
    }

    println!("Valid Passwords (old): {}", valid_passwords_old);
    println!("Valid Passwords (new): {}", valid_passwords_new);
}
