use std::collections::HashMap;
use std::fs;
use substring::Substring;

struct Passport {
    data: HashMap<String, String>,
}

impl Passport {
    fn from_string(s: &str) -> Passport {
        let mut map = HashMap::<String, String>::new();
        let tokens: Vec<&str> = s.split(|c| c == ' ' || c == '\n').collect();
        for t in tokens {
            let key_value: Vec<&str> = t.split(":").collect();

            if key_value.len() != 2 {
                panic!("Key-Value pair should only have two elements");
            }

            map.insert(String::from(key_value[0]), String::from(key_value[1]));
        }

        Passport { data: map }
    }

    fn is_val_in_range(s: &String, min: i32, max: i32) -> bool {
        let val = match s.parse::<i32>() {
            Ok(v) => v,
            Err(_) => return false,
        };

        if val < min || val > max {
            return false;
        }

        return true;
    }

    fn is_valid(&self) -> bool {
        let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        for &f in &required_fields {
            if !self.data.contains_key(f) {
                return false;
            }
        }
        return true;
    }

    fn is_valid_extensive(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        if !Passport::is_val_in_range(self.data.get("byr").unwrap(), 1920, 2002) {
            return false;
        }

        if !Passport::is_val_in_range(self.data.get("iyr").unwrap(), 2010, 2020) {
            return false;
        }

        if !Passport::is_val_in_range(self.data.get("eyr").unwrap(), 2020, 2030) {
            return false;
        }

        let hgt = self.data.get("hgt").unwrap();
        let hgt_number = String::from(hgt.substring(0, hgt.len() - 2));
        let hgt_unit = String::from(hgt.substring(hgt.len() - 2, hgt.len()));

        if hgt_unit == "cm" && !Passport::is_val_in_range(&hgt_number, 150, 193) {
            return false;
        }

        if hgt_unit == "in" && !Passport::is_val_in_range(&hgt_number, 59, 76) {
            return false;
        }

        if hgt_unit != "cm" && hgt_unit != "in" {
            return false;
        }

        let hcl: Vec<char> = self.data.get("hcl").unwrap().chars().collect();
        if hcl.len() != 7 || hcl[0] != '#' {
            return false;
        }

        for c in &hcl[1..hcl.len()] {
            if !"0123456789abcdef".chars().any(|x| x == *c) {
                return false;
            }
        }

        let ecl = self.data.get("ecl").unwrap();
        if !vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .any(|x| x == ecl)
        {
            return false;
        }

        let pid: Vec<char> = self.data.get("pid").unwrap().chars().collect();
        if pid.len() != 9 {
            return false;
        }

        for c in pid {
            if !"0123456789".chars().any(|x| x == c) {
                return false;
            }
        }

        return true;
    }
}

fn main() {
    let input = fs::read_to_string("src/day04/input.txt").unwrap();
    let data: Vec<&str> = input.split("\n\n").collect();

    let mut cnt_valid_simple: i32 = 0;
    let mut cnt_valid_extensive: i32 = 0;

    for d in data {
        let p = Passport::from_string(d);
        if p.is_valid() {
            cnt_valid_simple += 1;
        }
        if p.is_valid_extensive() {
            cnt_valid_extensive += 1;
        }
    }

    println!("Number of simple valid passports: {}", cnt_valid_simple);
    println!(
        "Number of extensive valid passports: {}",
        cnt_valid_extensive
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        let input = fs::read_to_string("src/day04/input_test_01.txt").unwrap();
        let data: Vec<&str> = input.split("\n\n").collect();

        let mut passports = Vec::<Passport>::new();

        for d in data {
            passports.push(Passport::from_string(d));
        }

        assert_eq!(passports[0].is_valid(), true);
        assert_eq!(passports[1].is_valid(), false);
        assert_eq!(passports[2].is_valid(), true);
        assert_eq!(passports[3].is_valid(), false);
    }

    #[test]
    fn test_is_valid_extensive() {
        let input = fs::read_to_string("src/day04/input_test_02_valid.txt").unwrap();
        let data: Vec<&str> = input.split("\n\n").collect();

        let mut i = 0;
        for d in data {
            println!("Test: {}", i);
            i += 1;
            assert_eq!(Passport::from_string(d).is_valid_extensive(), true);
        }
    }

    #[test]
    fn test_is_invalid_extensive() {
        let input = fs::read_to_string("src/day04/input_test_02_invalid.txt").unwrap();
        let data: Vec<&str> = input.split("\n\n").collect();

        for d in data {
            assert_eq!(Passport::from_string(d).is_valid_extensive(), false);
        }
    }
}
