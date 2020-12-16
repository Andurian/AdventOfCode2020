#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

struct Limits {
    range: HashMap<String, Vec<(i32, i32)>>,
}

impl Limits {
    fn from_string(s: &str) -> Limits {
        lazy_static! {
            static ref RE: Regex = Regex::new("^([^:]*): (\\d*)-(\\d*) or (\\d*)-(\\d*)").unwrap();
        }

        let mut map = HashMap::new();

        for l in s.split("\n").map(|l| l.trim()) {
            let matches = RE.captures(l.trim()).unwrap();
            let v = (2..matches.len())
                .step_by(2)
                .map(|i| {
                    (
                        matches[i].parse::<i32>().unwrap(),
                        matches[i + 1].parse::<i32>().unwrap(),
                    )
                })
                .collect::<Vec<(i32, i32)>>();
            map.insert(String::from(&matches[1]), v);
        }

        Limits { range: map }
    }

    fn matches(&self, v: i32) -> HashSet<String> {
        let mut ret = HashSet::new();

        for (k, entry) in &self.range {
            for pair in entry {
                if v >= pair.0 && v <= pair.1 {
                    ret.insert(k.clone());
                }
            }
        }

        ret
    }

    fn matches_ticket(&self, ticket: &Vec<i32>) -> bool {
        for v in ticket {
            if self.matches(*v).is_empty() {
                return false;
            }
        }
        return true;
    }
}

fn solve_1(filename: &str) -> i32 {
    let groups = common::read_grouped_file(filename);

    let l = Limits::from_string(&groups[0]);

    groups[2]
        .split("\n")
        .map(|l| l.trim())
        .skip(1)
        .fold(0, |acc, line| {
            let mut cnt = 0;
            for x in line.split(",").map(|x| x.parse::<i32>().unwrap()) {
                if l.matches(x).is_empty() {
                    cnt += x
                }
            }
            acc + cnt
        })
}

fn solve_2(filename: &str) -> u64 {
    let groups = common::read_grouped_file(filename);

    let l = Limits::from_string(&groups[0]);

    let valid_tickets = groups[2]
        .split("\n")
        .skip(1)
        .map(|l| {
            l.trim()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|ticket| l.matches_ticket(ticket))
        .collect::<Vec<Vec<i32>>>();

    let possible_values = l
        .range
        .iter()
        .map(|(k, _v)| k.clone())
        .collect::<HashSet<String>>();
    let mut possible_values = (0..valid_tickets.first().unwrap().len())
        .map(|_| possible_values.clone())
        .collect::<Vec<HashSet<String>>>();

    for ticket in valid_tickets {
        for (i, v) in ticket.iter().enumerate() {
            let matches = l.matches(*v);
            possible_values[i] = possible_values[i].intersection(&matches).cloned().collect();
        }
    }

    let len = possible_values.len();
    while possible_values.iter().any(|entry| entry.len() > 1) {
        for i in 0..len {
            if possible_values[i].len() == 1 {
                let value_to_remove = possible_values[i].iter().next().unwrap().clone();
                for j in 0..len {
                    if i == j {
                        continue;
                    }
                    possible_values[j].remove(&value_to_remove);
                }
            }
        }
    }

    let my_ticket = groups[1]
        .split("\n")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    possible_values.iter().enumerate().fold(1u64, |acc, (i, v)| {
        if v.iter().next().unwrap().clone().starts_with("departure") {
            return acc * (my_ticket[i] as u64);
        } else {
            return acc;
        }
    })
}

fn main() {
    println!("Solution 1: {}", solve_1("src/day16/input.txt"));
    println!("Solution 2: {}", solve_2("src/day16/input.txt"));
}
