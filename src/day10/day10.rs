use std::fs;

fn count_differences(mut jolts: Vec<i32>) -> (i32, i32, i32) {
    jolts.push(0);
    jolts.sort();

    let mut ret = (0, 0, 0);

    jolts[..jolts.len() - 1]
        .iter()
        .zip(jolts[1..].iter())
        .for_each(|(x, y)| match y - x {
            c if c == 1 => ret.0 += 1,
            c if c == 2 => ret.1 += 1,
            c if c == 3 => ret.2 += 1,
            _ => panic!("Invalid diff encountered"),
        });

    return ret;
}

fn file_to_numbers(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn solve_01(filename: &str) -> i32 {
    let cnts = count_differences(file_to_numbers(filename));
    return cnts.0 * (cnts.2 + 1);
}

fn solve_02(filename: &str) -> u128 {
    let mut jolts = file_to_numbers(filename);
    jolts.push(0);
    jolts.sort();
    jolts.push(jolts[jolts.len() - 1] + 3);
    let mut accumulator: Vec<u128> = vec![0; jolts.len()];
    accumulator[0] = 1;

    for i in 1..jolts.len() {
        for j in 1..4 {
            if i >= j && jolts[i] - jolts[i - j] <= 3 {
                accumulator[i] += accumulator[i - j];
            }
        }
    }
    return accumulator[jolts.len() - 1];
}

fn main() {
    println!("Solution 1: {}", solve_01("src/day10/input.txt"));
    println!("Solution 2: {}", solve_02("src/day10/input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_difference_count() {
        let cnts = count_differences(file_to_numbers("src/day10/input_test_01.txt"));
        assert_eq!(cnts.0, 7);
        assert_eq!(cnts.1, 0);
        assert_eq!(cnts.2, 4);
    }

    #[test]
    fn test_solve_01() {
        assert_eq!(solve_01("src/day10/input_test_01.txt"), 35);
        assert_eq!(solve_01("src/day10/input_test_02.txt"), 220);
    }

    #[test]
    fn test_solve_02(){
        assert_eq!(solve_02("src/day10/input_test_01.txt"), 8);
        assert_eq!(solve_02("src/day10/input_test_02.txt"), 19208);
    }
}
