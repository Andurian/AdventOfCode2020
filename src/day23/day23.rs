use std::collections::HashMap;
use std::fmt::Debug;

struct Ring {
    connections: HashMap<i32, i32>,
    current: i32,
}

impl Ring {
    fn new(values: &Vec<i32>) -> Ring {
        Ring {
            connections: values
                .iter()
                .zip(itertools::chain(values[1..].iter(), values[..1].iter()))
                .map(|(i, j)| (*i, *j))
                .collect::<HashMap<i32, i32>>(),
            current: values[0],
        }
    }

    fn next(&self, v: i32) -> i32 {
        *self.connections.get(&v).unwrap()
    }

    fn make_move(&mut self) {
        let start = self.next(self.current);
        let mid = self.next(start);
        let end = self.next(mid);

        let mut insert_1 = self.current - 1;
        if insert_1 <= 0 {
            insert_1 = self.connections.len() as i32;
        }
        while start == insert_1 || mid == insert_1 || end == insert_1 {
            insert_1 -= 1;
            if insert_1 <= 0 {
                insert_1 = self.connections.len() as i32;
            }
        }

        let insert_2 = self.next(insert_1);

        self.connections.insert(self.current, self.next(end));

        self.connections.insert(insert_1, start);
        self.connections.insert(end, insert_2);

        self.current = self.next(self.current);
    }

    fn solutions_str(&self) -> String {
        let mut ret = String::new();

        let mut x = self.next(1);
        while x != 1 {
            ret.push_str(&format!("{}", x));
            x = self.next(x);
        }

        ret
    }
}

impl Debug for Ring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("({})", self.current));
        let mut x = self.next(self.current);
        while x != self.current {
            s.push_str(&format!(", {}", x));
            x = self.next(x);
        }
        f.write_str(&s)
    }
}

fn solve_1(values: Vec<i32>, num_moves: i32) -> String {
    let mut r = Ring::new(&values);
    for _ in 0..num_moves {
        r.make_move();
    }
    r.solutions_str()
}

fn solve_2(values: Vec<i32>) -> i128 {
    let mut v = (1..1_000_001).collect::<Vec<i32>>();
    for i in 0..values.len() {
        v[i] = values[i];
    }

    let mut r = Ring::new(&v);
    for _ in 0..10_000_000 {
        r.make_move();
    }

    let a = r.next(1);
    let b = r.next(a) as i128;

    (a as i128) * b
}

fn main() {
    println!(
        "Solution 1: {}",
        solve_1(vec![5, 6, 2, 8, 9, 3, 1, 4, 7], 100)
    );
    println!("Solution 2: {}", solve_2(vec![5, 6, 2, 8, 9, 3, 1, 4, 7]));
}
