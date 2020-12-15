#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;

trait Program {
    fn mask_instruction(&mut self, mask_str: &str);
    fn mem_instruction(&mut self, address: u64, value: u64);
    fn memory_sum(&self) -> u64;
}

struct ProgramV1 {
    memory: HashMap<u64, u64>,
    and_mask: u64,
    or_mask: u64,
}

impl ProgramV1 {
    fn new() -> ProgramV1 {
        ProgramV1 {
            memory: HashMap::<u64, u64>::new(),
            and_mask: !0u64,
            or_mask: 0u64,
        }
    }
}

impl Program for ProgramV1 {
    fn mask_instruction(&mut self, mask_str: &str) {
        self.and_mask = !0u64;
        self.or_mask = 0u64;
        for (i, c) in mask_str.chars().rev().enumerate() {
            match c {
                x if x == 'X' => continue,
                x if x == '1' => self.or_mask |= 1u64 << i,
                x if x == '0' => self.and_mask &= !(1u64 << i),
                _ => panic!("Unknown mask char encountered"),
            }
        }
    }

    fn mem_instruction(&mut self, address: u64, value: u64) {
        let value = (value & self.and_mask) | self.or_mask;
        self.memory.insert(address, value);
    }

    fn memory_sum(&self) -> u64 {
        self.memory.iter().fold(0, |acc, (_k, v)| acc + v)
    }
}

struct ProgramV2 {
    memory: HashMap<u64, u64>,
    masks: Vec<(u64, u64)>,
}

impl ProgramV2 {
    fn new() -> ProgramV2 {
        ProgramV2 {
            memory: HashMap::<u64, u64>::new(),
            masks: Vec::<(u64, u64)>::new(),
        }
    }
}

impl Program for ProgramV2 {
    fn mask_instruction(&mut self, mask: &str) {
        self.masks.clear();
        let mut const_or_mask = 0u64;
        let mut pos_x = Vec::<usize>::new();
        for (i, c) in mask.chars().rev().enumerate() {
            match c {
                x if x == '1' => const_or_mask |= 1u64 << i,
                x if x == 'X' => pos_x.push(i),
                _ => (),
            }
        }
        for i in 0u64..(1u64 << pos_x.len()) {
            let mut and_mask = !0u64;
            let mut or_mask = const_or_mask;
            for j in 0u64..pos_x.len() as u64 {
                let offset = pos_x[j as usize];
                if i & 1u64 << j != 0 {
                    or_mask |= 1u64 << offset;
                } else {
                    and_mask &= !(1u64 << offset);
                }
            }
            self.masks.push((and_mask, or_mask));
        }
    }
    fn mem_instruction(&mut self, address: u64, value: u64) {
        for (and_mask, or_mask) in &self.masks {
            let addr = (address & and_mask) | or_mask;
            self.memory.insert(addr, value);
        }
    }
    fn memory_sum(&self) -> u64 {
        self.memory.iter().fold(0, |acc, (_k, v)| acc + v)
    }
}

fn run_program(filename: &str, p: &mut dyn Program) -> u64 {
    lazy_static! {
        static ref RE_MASK: Regex = Regex::new("^mask = (.*)").unwrap();
        static ref RE_MEM: Regex = Regex::new("^mem\\[(\\d+)\\] = (\\d+)").unwrap();
    }

    for line in &common::read_file_linewise(filename) {
        if RE_MASK.is_match(line) {
            let matches = RE_MASK.captures(line).unwrap();
            p.mask_instruction(&matches[1]);
        } else if RE_MEM.is_match(line) {
            let matches = RE_MEM.captures(line).unwrap();
            let address = matches[1].parse::<u64>().unwrap();
            let value = matches[2].parse::<u64>().unwrap();
            p.mem_instruction(address, value);
        }
    }

    p.memory_sum()
}

fn main() {
    println!(
        "Solution 1: {}",
        run_program("src/day14/input.txt", &mut ProgramV1::new())
    );
    println!(
        "Solution 2: {}",
        run_program("src/day14/input.txt", &mut ProgramV2::new())
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_v1() {
        assert_eq!(
            run_program("src/day14/input_test_01.txt", &mut ProgramV1::new()),
            165
        );
    }

    #[test]
    fn test_v2() {
        assert_eq!(
            run_program("src/day14/input_test_02.txt", &mut ProgramV2::new()),
            208
        );
    }
}
