use std::collections::HashSet;

enum InstructionType {
    NoOperation,
    Accumulate,
    Jump,
}

trait Instruction {
    fn apply(&self, state: &mut State);
    fn instruction_type(&self) -> InstructionType;
    fn argument(&self) -> i32;
}

struct State {
    accumulator: i32,
    instruction_pointer: i32,
}

impl State {
    fn new() -> State {
        State {
            accumulator: 0,
            instruction_pointer: 0,
        }
    }
}

struct Program {
    state: State,
    code: Vec<Box<dyn Instruction>>,
}

struct NoOperation {
    argument: i32,
}

impl Instruction for NoOperation {
    fn apply(&self, state: &mut State) {
        state.instruction_pointer += 1;
    }

    fn instruction_type(&self) -> InstructionType {
        InstructionType::NoOperation
    }

    fn argument(&self) -> i32 {
        self.argument
    }
}

struct Accumulate {
    argument: i32,
}

impl Instruction for Accumulate {
    fn apply(&self, state: &mut State) {
        state.accumulator += self.argument;
        state.instruction_pointer += 1;
    }

    fn instruction_type(&self) -> InstructionType {
        InstructionType::Accumulate
    }

    fn argument(&self) -> i32 {
        self.argument
    }
}

struct Jump {
    argument: i32,
}

impl Instruction for Jump {
    fn apply(&self, state: &mut State) {
        state.instruction_pointer += self.argument;
    }

    fn instruction_type(&self) -> InstructionType {
        InstructionType::Jump
    }

    fn argument(&self) -> i32 {
        self.argument
    }
}

impl Program {
    fn from_vec(lines: &Vec<String>) -> Program {
        Program {
            state: State::new(),
            code: lines
                .iter()
                .map(|line| {
                    let tokens: Vec<&str> = line.split(" ").collect();
                    let argument = tokens[1].parse::<i32>().unwrap();

                    match tokens[0] {
                        s if s == "nop" => {
                            Box::new(NoOperation { argument }) as Box<dyn Instruction>
                        }
                        s if s == "acc" => Box::new(Accumulate { argument }),
                        s if s == "jmp" => Box::new(Jump { argument }),
                        _ => panic!("Encountered unknown instruction"),
                    }
                })
                .collect(),
        }
    }

    fn try_run(&mut self) -> bool {
        let mut executed_instructions = HashSet::<i32>::new();
        self.state = State::new();

        while !executed_instructions.contains(&self.state.instruction_pointer)
            && self.state.instruction_pointer < self.code.len() as i32
        {
            executed_instructions.insert(self.state.instruction_pointer);
            let instruction = &self.code[self.state.instruction_pointer as usize];

            instruction.apply(&mut self.state);
        }

        return self.state.instruction_pointer == self.code.len() as i32;
    }

    fn try_fix_code(&mut self) -> bool {
        for fix_idx in 0..self.code.len() - 1 {
            match self.code[fix_idx].instruction_type() {
                InstructionType::NoOperation | InstructionType::Jump => {
                    self.swap_nop_for_jmp(fix_idx);
                }
                _ => continue,
            }

            if self.try_run() {
                return true;
            }

            self.swap_nop_for_jmp(fix_idx);
        }

        return false;
    }

    fn swap_nop_for_jmp(&mut self, i: usize) {
        let arg = self.code[i].argument();

        self.code[i] = match self.code[i].instruction_type() {
            InstructionType::Jump => {
                Box::new(NoOperation { argument: arg }) as Box<dyn Instruction>
            }
            InstructionType::NoOperation => Box::new(Jump { argument: arg }),
            _ => panic!("Encountered unswitchable type"),
        }
    }
}

fn main() {
    let mut code = Program::from_vec(&common::read_file_linewise("src/day08/input.txt"));
    code.try_run();
    println!("Accumulator before first loop: {}", code.state.accumulator);

    if code.try_fix_code() {
        println!("Fixed. Final accumulator: {}", code.state.accumulator);
    } else {
        println!("Could not fix code");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_acc_before_first_loop() {
        let mut code = Program::from_vec(&common::read_file_linewise("src/day08/input_test.txt"));
        code.try_run();
        assert_eq!(code.state.accumulator, 5);
    }

    #[test]
    fn test_fix_code() {
        let mut code = Program::from_vec(&common::read_file_linewise("src/day08/input_test.txt"));
        assert_eq!(code.try_fix_code(), true);
        assert_eq!(code.state.accumulator, 8);
    }
}
