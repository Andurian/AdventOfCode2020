fn main() {

    let numbers = common::parse_file_linewise_as::<i32>("src/day01/input.txt");

    'outer1: for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];

            if a + b == 2020 {
                println!("{} * {} = {}", a, b, a * b);
                break 'outer1;
            }
        }
    }

    'outer2: for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            for k in j + 1..numbers.len() {
                let a = &numbers[i];
                let b = &numbers[j];
                let c = &numbers[k];

                if a + b + c == 2020 {
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    break 'outer2;
                }
            }
        }
    }
}
