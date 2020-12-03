use std::convert::TryFrom;
use std::str::FromStr;

use reduce::Reduce;

pub use error::FieldParseError;
pub use error::FieldStateParseError;

// Field
//##################

#[derive(Default, Debug, Clone)]
pub struct Field(Vec<Vec<FieldState>>);

impl Field {
    fn new(content: Vec<Vec<FieldState>>) -> Self {
        Self(content)
    }

    pub fn get_field(&self, x: usize, y: usize) -> FieldState {
        // vertical
        assert!(y < self.0.len());
        let line = &self.0[y];

        // horizontal
        // endlessly repeating
        let read_x = x % line.len();
        line[read_x]
    }

    pub fn is_tree_at(&self, x: usize, y: usize) -> bool {
        self.get_field(x, y) == FieldState::Tree
    }

    /// The distance from top to bottom
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for Field {
    type Err = FieldParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Convert input into Vec<Vec<FieldState>>
        // Two-dimensional field of states
        let values = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(FieldState::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Check that every line equal length
        // Count length, wrap in option, and enumerate them
        // Check if length is different from line before,
        //  if yes, propagate None and line number to end
        //  if no, continue with comparison
        let result = values
            .iter()
            .map(|v| Option::Some(v.len()))
            .enumerate()
            .reduce(|old, new| match (old, new) {
                // propagate none
                ((x, None), _) => (x, None),
                // last one is none. should never happen?
                ((x, _), (_, None)) => (x, None),
                // new one has different length than old -> pass the line number of error
                ((_, Some(old)), (x, Some(new))) if old != new => (x, None),
                // length should be equal -> pass new one
                (_, (x, y)) => (x, y),
            });
        // return error from length check if applicable
        if let Some((line, None)) = result {
            return Err(FieldParseError::LineDifferentLength(line));
        }

        Ok(Self::new(values))
    }
}

//##################

// FieldState
//##################
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FieldState {
    Empty,
    Tree,
}

impl TryFrom<char> for FieldState {
    type Error = FieldStateParseError;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '#' => Ok(FieldState::Tree),
            '.' => Ok(FieldState::Empty),
            x => Err(FieldStateParseError::UnknownParameter(x)),
        }
    }
}

//##################

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum FieldParseError {
        #[error("failed to parse state: {0}")]
        FailedParsingFieldState(#[from] FieldStateParseError),
        #[error("line {0} other length than the ones before")]
        LineDifferentLength(usize),
    }

    #[derive(Error, Debug)]
    pub enum FieldStateParseError {
        #[error("char not recognized: {0}")]
        UnknownParameter(char),
    }
}
