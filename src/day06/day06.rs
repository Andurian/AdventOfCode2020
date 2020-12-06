use std::collections::HashSet;
use std::fs;

fn group_count_or(answers: &str) -> i32 {
    return answers
        .split("\n")
        .flat_map(|line| line.chars())
        .collect::<HashSet<char>>()
        .len() as i32;
}

fn group_count_and(answers: &str) -> i32 {
    // Can be shortened by using unstable feature fold_first insteads of fold with a dummy set
    return answers
        .split("\n")
        .map(|s| s.chars().collect::<HashSet<char>>())
        .fold(
            "abcdefghijklmnopqrstuvwxyz"
                .chars()
                .collect::<HashSet<char>>(),
            |a, b| a.intersection(&b).cloned().collect(),
        )
        .len() as i32;
}

fn total_count(groups: &Vec<&str>, f: &dyn Fn(&str) -> i32) -> i32 {
    return groups.iter().map(|s| f(s)).fold(0, |a, b| a + b);
}

fn main() {
    let input = fs::read_to_string("src/day06/input.txt").unwrap();
    let groups: Vec<&str> = input.split("\n\n").collect();

    println!(
        "Total count \"anyone\":   {}",
        total_count(&groups, &group_count_or)
    );
    println!(
        "Total count \"everyone\": {}",
        total_count(&groups, &group_count_and)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total_count_or() {
        let input = fs::read_to_string("src/day06/input_test.txt").unwrap();
        assert_eq!(
            total_count(&input.split("\n\n").collect(), &group_count_or),
            11
        );
    }

    #[test]
    fn test_total_count_and() {
        let input = fs::read_to_string("src/day06/input_test.txt").unwrap();
        assert_eq!(
            total_count(&input.split("\n\n").collect(), &group_count_and),
            6
        );
    }
}
