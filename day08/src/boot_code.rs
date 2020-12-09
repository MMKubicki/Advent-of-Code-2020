use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use error::{ParseInstructionError, ParseInstructionListError};

pub fn parse_instruction_list(
    content: &str,
) -> Result<Vec<Instruction>, ParseInstructionListError> {
    content
        .lines()
        .map(str::parse::<Instruction>)
        .enumerate()
        .map(|(c, res)| res.map_err(|err| (c, err).into()))
        .collect::<Result<Vec<_>, _>>()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TermReason {
    Loop(isize),
    End(isize),
}

// Instruction
//##################

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Instruction {
    Nop(Sign, usize),
    Acc(isize),
    Jmp(Sign, usize),
}

impl Instruction {
    pub fn get_name(&self) -> &'static str {
        match self {
            Instruction::Nop(_, _) => "nop",
            Instruction::Acc(_) => "acc",
            Instruction::Jmp(_, _) => "jmp",
        }
    }

    pub fn get_value(&self) -> isize {
        match self {
            Instruction::Nop(sign, value) => sign_value_to_isize(sign, value),
            Instruction::Acc(value) => *value,
            Instruction::Jmp(sign, value) => sign_value_to_isize(sign, value),
        }
    }
}

fn sign_value_to_isize(sign: &Sign, value: &usize) -> isize {
    match sign {
        Sign::Plus => *value as isize,
        Sign::Minus => -(*value as isize),
    }
}

impl FromStr for Instruction {
    type Err = error::ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        let instruction_str = match split.next() {
            Some(value) => value,
            None => return Err(ParseInstructionError::MalformedInputError(s.to_owned())),
        };

        let instruction_data = match split.next() {
            Some(value) => value.parse::<isize>()?,
            None => return Err(ParseInstructionError::MalformedInputError(s.to_owned())),
        };

        let (sign, value) = if instruction_data < 0 {
            (Sign::Minus, (-instruction_data) as usize)
        } else {
            (Sign::Plus, instruction_data as usize)
        };

        match instruction_str {
            "nop" => Ok(Instruction::Nop(sign, value)),
            "acc" => Ok(Instruction::Acc(instruction_data)),
            "jmp" => Ok(Instruction::Jmp(sign, value)),
            x => Err(ParseInstructionError::UnknownInstructionError(
                x.to_owned(),
                s.to_owned(),
            )),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:+}", self.get_name(), self.get_value())
    }
}

impl AsRef<Instruction> for Instruction {
    fn as_ref(&self) -> &Instruction {
        &self
    }
}

//##################

// Machine
//##################

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Machine {
    pc: usize,
    acc: isize,
}

impl Machine {
    pub fn run_instruction<T: AsRef<Instruction>>(&mut self, instruction: T) {
        let instruction = instruction.as_ref();

        match instruction {
            Instruction::Nop(_, _) => {
                self.pc += 1;
            }
            Instruction::Acc(value) => {
                self.acc += value;
                self.pc += 1;
            }
            Instruction::Jmp(sign, value) => match sign {
                Sign::Plus => self.pc += value,
                Sign::Minus => self.pc -= value,
            },
        }
    }

    pub fn run_till_term<T: AsRef<Instruction>>(&mut self, instructions: &[T]) -> TermReason {
        let mut ran_instruction = HashSet::new();

        loop {
            if ran_instruction.contains(&self.pc) {
                return TermReason::Loop(self.acc);
            }
            ran_instruction.insert(self.pc);

            let instruction = match instructions.get(self.pc) {
                None => return TermReason::End(self.acc),

                Some(value) => value,
            };

            self.run_instruction(instruction);
        }
    }
}

//##################

pub mod error {
    use std::num::ParseIntError;
    use thiserror::Error;

    #[derive(Error, Debug, Eq, PartialEq, Clone)]
    pub enum ParseInstructionError {
        #[error("can not use input: {0}")]
        MalformedInputError(String),

        #[error("unknown instruction \"{0}\" found in input: {1}")]
        UnknownInstructionError(String, String),

        #[error("error parsing instruction data: {0}")]
        ParseIntError(#[from] ParseIntError),
    }

    #[derive(Error, Debug, Eq, PartialEq, Clone)]
    #[error("error on line {line}: {error}")]
    pub struct ParseInstructionListError {
        line: usize,
        error: ParseInstructionError,
    }

    impl From<(usize, ParseInstructionError)> for ParseInstructionListError {
        fn from((line, error): (usize, ParseInstructionError)) -> Self {
            Self { line, error }
        }
    }
}
