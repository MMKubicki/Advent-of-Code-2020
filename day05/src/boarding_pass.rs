use std::convert::TryFrom;
use std::str::FromStr;

pub use error::SeatParseError;

// Seat
//##################

#[derive(Debug, Eq, PartialEq)]
pub struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn get_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

impl FromStr for Seat {
    type Err = error::SeatParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // expected Format: RRRRRRRCCC
        // with R : Row -> F (Front), B (Back)
        // and C: Column -> L (Left), R (Right)
        // eg. FBFBBFBLLR

        use error::SeatParseError::*;

        // there has to be 10 Chars
        if s.chars().count() != 10 {
            return Err(MalformedInput);
        }

        // take the first 7 chars, parse them to RowSelect (F or B), pull unknown chars out
        // and report that error if it happens
        let row_info = s
            .chars()
            .take(7)
            .map(RowSelect::try_from)
            .collect::<Result<Vec<_>, _>>();
        let row_info = match row_info {
            Ok(values) => values,
            Err(e) => return Err(UnknownRowChar(e.get_char())),
        };

        // use gained row info to binary search row between 0 and 127
        let row = binary_search(&row_info, 0, 127);

        // skip first 7 chars, take the next 3, parse them to ColumnSelect (L or R), pull unknown chars out
        // and report that error if it happens
        let column_info = s
            .chars()
            .skip(7)
            .take(3)
            .map(ColumnSelect::try_from)
            .collect::<Result<Vec<_>, _>>();
        let column_info = match column_info {
            Ok(values) => values,
            Err(e) => return Err(UnknownColumnChar(e.get_char())),
        };

        // use gained row info to binary search row between 0 and 7
        let column = binary_search(&column_info, 0, 7);

        Ok(Seat::new(row, column))
    }
}

//##################

fn binary_search<C: BinarySearchHelper>(directives: &[C], min: usize, max: usize) -> usize {
    let mut min = min;
    let mut max = max;

    for directive in directives {
        let half = (min + max + 1) / 2;

        if directive.is_lower() {
            // min = min
            max = half - 1;
        } else {
            min = half;
            // max = max
        }
    }

    assert_eq!(min, max);

    min
}

// RowSelect
//##################

#[derive(Debug, Eq, PartialEq)]
pub enum RowSelect {
    Front,
    Back,
}

impl TryFrom<char> for RowSelect {
    type Error = error::IdentifierParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use error::IdentifierParseError;
        use RowSelect::*;

        match c {
            'F' => Ok(Front),
            'B' => Ok(Back),
            x => Err(IdentifierParseError::new(x)),
        }
    }
}

impl BinarySearchHelper for RowSelect {
    fn is_lower(&self) -> bool {
        *self == RowSelect::Front
    }
}

//##################

// ColumnSelect
//##################

#[derive(Debug, Eq, PartialEq)]
pub enum ColumnSelect {
    Left,
    Right,
}

impl TryFrom<char> for ColumnSelect {
    type Error = error::IdentifierParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use error::IdentifierParseError;
        use ColumnSelect::*;

        match c {
            'L' => Ok(Left),
            'R' => Ok(Right),
            x => Err(IdentifierParseError::new(x)),
        }
    }
}

impl BinarySearchHelper for ColumnSelect {
    fn is_lower(&self) -> bool {
        *self == ColumnSelect::Left
    }
}

//##################

pub trait BinarySearchHelper {
    fn is_lower(&self) -> bool;

    fn is_upper(&self) -> bool {
        !self.is_lower()
    }
}

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("unknown char '{unknown_char}'")]
    pub struct IdentifierParseError {
        unknown_char: char,
    }

    impl IdentifierParseError {
        pub fn new(unknown_char: char) -> Self {
            Self { unknown_char }
        }

        pub fn get_char(&self) -> char {
            self.unknown_char
        }
    }

    #[derive(Error, Debug)]
    pub enum SeatParseError {
        #[error("unknown row char '{0}'")]
        UnknownRowChar(char),

        #[error("unknown column char '{0}'")]
        UnknownColumnChar(char),

        #[error("malformed input")]
        MalformedInput,
    }
}
