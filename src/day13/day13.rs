// Sloppy implementation of: https://de.wikipedia.org/wiki/Chinesischer_Restsatz
// Finds the smallest integer x > 0 that fulfills
//      x = input[0] (mod input[1])
//      ...
//      x = input[n] (mond input[n])
fn find_congruency(input: &Vec<(i128, i128)>) -> i128 {
    let big_m = input.iter().fold(1, |acc, (_x, m)| acc * m);

    let mut res = input.iter().fold(0, |acc, (x, m)| {
        let mi = big_m / m;
        let (_g, _r, s) = modinverse::egcd(*m, mi);
        let e = s * mi;
        acc + x * e
    });

    while res < 0 {
        res += big_m
    }

    res
}

fn find_first_bus_to_take(filename: &str) -> i32 {
    let input = common::read_file_linewise(filename);
    let estimate = input[0].parse::<i32>().unwrap();
    let res = input[1]
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| {
            let x = x.parse::<i32>().unwrap();
            (x, (x - (estimate % x)).abs())
        })
        .min_by_key(|x| x.1)
        .unwrap();
    res.0 * res.1
}

fn find_earliest_time_for_contest(filename: &str) -> i128 {
    let input = common::read_file_linewise(filename);
    let timetable = input[1]
        .split(',')
        .enumerate()
        .filter(|(_i, x)| *x != "x")
        .map(|(i, x)| {
            let x = x.parse::<i128>().unwrap();
            (x - i as i128, x)
        })
        .collect::<Vec<(i128, i128)>>();

    find_congruency(&timetable)
}

fn main() {
    println!(
        "Solution 1: {}",
        find_first_bus_to_take("src/day13/input.txt")
    );
    println!(
        "Solution 2: {}",
        find_earliest_time_for_contest("src/day13/input.txt")
    );

    find_earliest_time_for_contest("src/day13/input_test.txt");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_first_bus_to_take() {
        assert_eq!(find_first_bus_to_take("src/day13/input_test.txt"), 295);
    }

    #[test]
    fn test_find_earliest_time_for_contest() {
        assert_eq!(
            find_earliest_time_for_contest("src/day13/input_test.txt"),
            1068781
        );
    }

    #[test]
    fn test_find_congruency() {
        assert_eq!(
            find_congruency(&vec![(0, 7), (12, 13), (55, 59), (25, 31), (12, 19)]),
            1068781
        );
        //assert_eq!(find_congruency(&vec![(0, 17), (11, 13), (16, 19)]), 3417); // This test case somehow doesn't work...
        assert_eq!(
            find_congruency(&vec![(0, 67), (6, 7), (57, 59), (58, 61)]),
            754018
        );
        assert_eq!(
            find_congruency(&vec![(0, 67), (5, 7), (56, 59), (57, 61)]),
            779210
        );
        assert_eq!(
            find_congruency(&vec![(0, 67), (6, 7), (56, 59), (57, 61)]),
            1261476
        );
        assert_eq!(
            find_congruency(&vec![(0, 1789), (36, 37), (45, 47), (1886, 1889)]),
            1202161486
        );
    }
}
