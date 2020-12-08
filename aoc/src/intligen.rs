use std::collections::{HashMap, HashSet};

use tracing::{error, info};

#[derive(Debug, Clone)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}
#[derive(Debug, Clone)]
pub struct Instruction(Operation, isize);

pub enum Next {
    Done(isize),
    Step,
    Seen(isize, isize, isize),
}
pub struct Machine {
    accumulator: isize,
    program_counter: isize,
    program: Vec<Instruction>,
    visited: HashSet<isize>,
}

impl Machine {
    pub fn fix_it(&mut self) -> isize {
        let mut visits: HashMap<isize, bool> = HashMap::default();
        loop {
            match self.step_toggle(&mut visits) {
                Next::Done(result) => {
                    info!("Done");
                    return result;
                }
                Next::Step => (),
                Next::Seen(old_pc, next_pc, old_acc) => {
                    info!(
                        "At {} before revisit at {}, the accumulator was {}",
                        old_pc, next_pc, old_acc
                    );
                    return old_acc;
                }
            }
        }
        // pc -> vec (count, acc)
    }

    fn step_toggle(&mut self, visits: &mut HashMap<isize, bool>) -> Next {
        self.visited.insert(self.program_counter);
        if let Some(next_op) = self.program.get(self.program_counter as usize).cloned() {
            let (next_pc, next_acc) = match next_op.0 {
                Operation::Acc => (self.program_counter + 1, self.accumulator + next_op.1),
                Operation::Jmp => (self.program_counter + next_op.1, self.accumulator),
                Operation::Nop => (self.program_counter + 1, self.accumulator),
            };
            if self.visited.contains(&next_pc) {
                Next::Seen(self.program_counter, next_pc, self.accumulator)
            } else {
                self.accumulator = next_acc;
                self.program_counter = next_pc;
                Next::Step
            }
        } else {
            info!("No operation at: {}", self.program_counter);
            Next::Done(self.accumulator)
        }
    }
    fn step(&mut self) -> Next {
        self.visited.insert(self.program_counter);
        if let Some(next_op) = self.program.get(self.program_counter as usize).cloned() {
            let (next_pc, next_acc) = match next_op.0 {
                Operation::Acc => (self.program_counter + 1, self.accumulator + next_op.1),
                Operation::Jmp => (self.program_counter + next_op.1, self.accumulator),
                Operation::Nop => (self.program_counter + 1, self.accumulator),
            };
            if self.visited.contains(&next_pc) {
                Next::Seen(self.program_counter, next_pc, self.accumulator)
            } else {
                self.accumulator = next_acc;
                self.program_counter = next_pc;
                Next::Step
            }
        } else {
            info!("No operation at: {}", self.program_counter);
            Next::Done(self.accumulator)
        }
    }
    pub fn run_to_first(&mut self) -> isize {
        loop {
            match self.step() {
                Next::Done(result) => {
                    info!("Done");
                    return result;
                }
                Next::Step => (),
                Next::Seen(old_pc, next_pc, old_acc) => {
                    info!(
                        "At {} before revisit at {}, the accumulator was {}",
                        old_pc, next_pc, old_acc
                    );
                    return old_acc;
                }
            }
        }
        // pc -> vec (count, acc)
    }
}

pub fn load_machine(input: &str) -> Machine {
    let program = match parse::get_instructions(input) {
        Ok((remainder, instructions)) => {
            if remainder.is_empty() {
                info!("Loaded {} instructions.", instructions.len());
                instructions
            } else {
                error!("{}", remainder);
                vec![]
            }
        }
        Err(err) => {
            error!("{}", err);
            vec![]
        }
    };
    Machine {
        accumulator: 0,
        program_counter: 0,
        visited: HashSet::default(),
        program,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn run_machine() {
        let mut machine = super::load_machine(PROGRAM);
        assert_eq!(machine.run_to_first(), 5);
    }

    const PROGRAM: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
    #[test]
    fn parse_it() {
        let (remainder, instructions) = super::parse::get_instructions(PROGRAM).expect("parse");
        assert_eq!(remainder, "");
        assert_eq!(instructions.len(), 9);
    }
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, newline},
        combinator::{map, map_res, opt},
        multi::separated_list0,
        sequence::separated_pair,
        IResult,
    };
    fn get_operation_acc(input: &str) -> IResult<&str, super::Operation> {
        map(tag("acc"), |_| super::Operation::Acc)(input)
    }
    fn get_operation_jmp(input: &str) -> IResult<&str, super::Operation> {
        map(tag("jmp"), |_| super::Operation::Jmp)(input)
    }
    fn get_operation_nop(input: &str) -> IResult<&str, super::Operation> {
        map(tag("nop"), |_| super::Operation::Nop)(input)
    }
    fn get_operation(input: &str) -> IResult<&str, super::Operation> {
        alt((get_operation_acc, get_operation_jmp, get_operation_nop))(input)
    }
    fn get_sign(input: &str) -> IResult<&str, isize> {
        map(
            opt(alt((map(tag("-"), |_| -1), map(tag("+"), |_| 1)))),
            |r| if let Some(s) = r { s } else { 1 },
        )(input)
    }
    fn get_isize(input: &str) -> IResult<&str, isize> {
        let (input, sign) = get_sign(input)?;
        let (input, result) = map_res(digit1, |s: &str| s.parse::<isize>())(input)?;
        Ok((input, sign * result))
    }
    fn get_instruction(input: &str) -> IResult<&str, super::Instruction> {
        map(
            separated_pair(get_operation, tag(" "), get_isize),
            |(op, par)| super::Instruction(op, par),
        )(input)
    }
    pub fn get_instructions(input: &str) -> IResult<&str, Vec<super::Instruction>> {
        separated_list0(newline, get_instruction)(input)
    }
}
