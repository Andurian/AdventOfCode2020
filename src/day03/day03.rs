use std::fs;

struct Field {
    field: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Field {
    fn from_string(s: &String) -> Field {
        let lines: Vec<&str> = s.split("\n").collect();
        let height = lines.len();
        let width = lines[0].len();

        let mut field = Vec::<Vec<char>>::new();

        for line in lines {
            if line.len() != width {
                panic!("Lines are not equal length.");
            }
            field.push(line.chars().collect());
        }

        return Field {
            field: field,
            width: width,
            height: height,
        };
    }

    fn print(&self) {
        for line in &self.field {
            for c in line {
                print!("{}", c)
            }
            print!("\n")
        }
    }

    fn at(&self, x: usize, y: usize) -> char {
        if y >= self.height {
            panic!("Out of range");
        }

        return self.field[y][x % (self.width)];
    }
}

fn count_trees(field: &Field, dx: usize, dy: usize) -> i32 {
    let mut cnt: i32 = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;

    while y < field.height - 1 {
        x += dx;
        y += dy;
        if field.at(x, y) == '#' {
            cnt += 1;
        }
    }

    return cnt;
}

fn count_multiple(field: &Field, directions: &[(usize, usize)]) -> i64 {
    let mut cnt: i64 = 1;

    for d in directions {
        cnt *= count_trees(&field, d.0, d.1) as i64;
    }

    return cnt;
}

fn main() {
    let input = fs::read_to_string("src/day03/input.txt").unwrap();
    let field = Field::from_string(&input);
    println!("Num trees encountered: {}", count_trees(&field, 3, 1));
    println!("Num multiplied trees: {}", count_multiple(&field, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_trees() {
        let input = fs::read_to_string("src/day03/input_test.txt").unwrap();
        let field = Field::from_string(&input);
        assert_eq!(count_trees(&field, 3, 1), 7);
    }

    #[test]
    fn test_count_multiple() {
        let input = fs::read_to_string("src/day03/input_test.txt").unwrap();
        let field = Field::from_string(&input);
        assert_eq!(
            count_multiple(&field, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]),
            336
        );
    }
}
