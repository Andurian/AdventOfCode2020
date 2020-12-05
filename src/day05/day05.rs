use std::fs;

fn str_to_number(s: &str, c0: char, c1: char) -> u8 {
    let binary_str: String = s
        .chars()
        .map(|x| match x {
            c if c == c0 => '0',
            c if c == c1 => '1',
            _ => panic!("invalid char encountered"),
        })
        .collect();

    return u8::from_str_radix(&binary_str, 2).unwrap();
}

fn get_seat_id(s: &str) -> i32 {
    let row = str_to_number(&s[..7], 'F', 'B');
    let col = str_to_number(&s[7..], 'L', 'R');

    return (row as i32) * 8 + (col as i32);
}

fn main() {
    let input = fs::read_to_string("src/day05/input.txt").unwrap();

    let mut ids: Vec<i32> = input.split("\n").map(get_seat_id).collect();
    ids.sort();

    println!("Max seat ID: {}", ids[ids.len() - 1]);
    for (i, &id) in ids[1..].iter().enumerate() {
        if id - ids[i] == 2 {
            println!("My seat ID: {}", id - 1);
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
    }
}
