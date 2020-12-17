use std::convert::TryFrom;
use std::str::FromStr;

pub fn parse_instruction_list(
    input: &str,
) -> Result<Vec<Instruction>, error::ParseInstructionError> {
    input.lines().map(str::parse).collect()
}

// General Parsing of Instruction
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Instruction {
    Mask { mask: Mask },
    Write { write: Write },
}

impl FromStr for Instruction {
    type Err = error::ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use error::ParseInstructionError;

        if s.starts_with("mask") {
            return Ok(Instruction::Mask { mask: s.parse()? });
        }
        if s.starts_with("mem") {
            return Ok(Instruction::Write { write: s.parse()? });
        }

        Err(ParseInstructionError::MalformedInputError(s.to_owned()))
    }
}

// Parsing of Write Instruction
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Write {
    pub to: usize,
    pub value: usize,
}

impl FromStr for Write {
    type Err = error::ParseWriteError;

    #[allow(clippy::or_fun_call)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use error::ParseWriteError;

        let mut iter = s.split(" = ");

        let pre_to = iter
            .next()
            .ok_or(ParseWriteError::MalformedInput(s.to_owned()))?
            .strip_prefix("mem[")
            .ok_or(ParseWriteError::MalformedInput(s.to_owned()))?
            .strip_suffix("]")
            .ok_or(ParseWriteError::MalformedInput(s.to_owned()))?
            .parse::<usize>();

        let to = match pre_to {
            Ok(val) => val,
            Err(error) => return Err(ParseWriteError::ParseDestinationError { error }),
        };

        let value = iter
            .next()
            .ok_or(ParseWriteError::MalformedInput(s.to_owned()))?
            .parse::<usize>()?;

        Ok(Write { to, value })
    }
}

// Mask Instruction -> Parsing, Storage and Applying of Mask
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Mask {
    mask: [MaskValue; 36],
}

impl Mask {
    pub fn apply_v1(&self, value: usize) -> usize {
        let bin = format!("{:036b}", value);

        let mask_applied: String = bin
            .chars()
            .zip(self.mask.iter())
            .map(|(c, m)| match m {
                MaskValue::Zero => '0',
                MaskValue::One => '1',
                MaskValue::DontCare => c,
            })
            .collect();

        usize::from_str_radix(&mask_applied, 2).expect("mask applying should keep it parsable")
    }

    pub fn apply_v2(&self, value: usize) -> Vec<usize> {
        let bin = format!("{:036b}", value);

        let applied_static = bin
            .chars()
            .zip(self.mask.iter())
            .map(|(c, m)| match m {
                MaskValue::Zero => c,
                MaskValue::One => '1',
                MaskValue::DontCare => 'X',
            })
            .collect::<String>();

        let mut chunk_iter = applied_static.split('X');

        let mut result = Vec::new();
        result.push(chunk_iter.next().unwrap().to_owned());

        for part in chunk_iter {
            let temp = result;
            result = Vec::new();

            for pref in temp {
                result.push(format!("{}0{}", pref, part));
                result.push(format!("{}1{}", pref, part));
            }
        }

        result
            .iter()
            .map(|s| usize::from_str_radix(s, 2).expect("Bitflipping should still be parsable"))
            .collect()
    }
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            mask: [MaskValue::DontCare; 36],
        }
    }
}

impl FromStr for Mask {
    type Err = error::ParseMaskError;

    #[allow(clippy::or_fun_call)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use error::ParseMaskError;

        let mut mask = [MaskValue::default(); 36];

        let pre_mask = s
            .split(" = ")
            .nth(1)
            .ok_or(ParseMaskError::MalformedInput(s.to_owned()))?
            .chars()
            .map(MaskValue::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        if pre_mask.len() != mask.len() {
            return Err(ParseMaskError::MaskTooShort {
                expected: mask.len(),
                is: pre_mask.len(),
            });
        }

        let mask_len = mask.len();
        mask.clone_from_slice(&pre_mask[..mask_len]);

        Ok(Mask { mask })
    }
}

// MaskValue -> Individual digits of Mask -> Parsing
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum MaskValue {
    Zero,
    One,
    DontCare,
}

impl Default for MaskValue {
    fn default() -> Self {
        MaskValue::DontCare
    }
}

impl TryFrom<char> for MaskValue {
    type Error = error::ParseMaskValueError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(MaskValue::Zero),
            '1' => Ok(MaskValue::One),
            'X' => Ok(MaskValue::DontCare),
            x => Err(error::ParseMaskValueError::UnknownCharError(x)),
        }
    }
}

// Error handling
pub mod error {
    use std::num::ParseIntError;

    use thiserror::Error;

    #[derive(Error, Debug, Clone, Eq, PartialEq)]
    pub enum ParseInstructionError {
        #[error("error parsing mask: {error}")]
        ParseMaskError {
            #[from]
            error: ParseMaskError,
        },
        #[error("error parsing write instruction: {error}")]
        ParseWriteError {
            #[from]
            error: ParseWriteError,
        },
        #[error("malformed input: {0}")]
        MalformedInputError(String),
    }

    #[derive(Error, Debug, Clone, Eq, PartialEq)]
    pub enum ParseWriteError {
        #[error("input malformed: {0}")]
        MalformedInput(String),
        #[error("error parsing destination integer: {error}")]
        ParseDestinationError { error: ParseIntError },
        #[error("error parsing value integer: {error}")]
        ParseValueError {
            #[from]
            error: ParseIntError,
        },
    }

    #[derive(Error, Debug, Clone, Eq, PartialEq, Hash)]
    pub enum ParseMaskError {
        #[error("error parsing mask value: {error}")]
        ParseMaskValueError {
            #[from]
            error: ParseMaskValueError,
        },
        #[error("input malformed: {0}")]
        MalformedInput(String),
        #[error("mask too short. expected: {expected}, is: {is}")]
        MaskTooShort { expected: usize, is: usize },
    }

    #[derive(Error, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ParseMaskValueError {
        #[error("unknown char: {0}")]
        UnknownCharError(char),
    }
}
