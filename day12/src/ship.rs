mod part1;
mod part2;

use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

pub use part1::Part1Position;
pub use part2::Part2Position;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum HorDir {
    East,
    West,
}

impl HorDir {
    pub fn turn_left(self) -> VerDir {
        match self {
            HorDir::East => VerDir::North,
            HorDir::West => VerDir::South,
        }
    }
    
    pub fn turn_right(self) -> VerDir {
        match self {
            HorDir::East => VerDir::South,
            HorDir::West => VerDir::North,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum VerDir {
    North,
    South,
}

impl VerDir {
    pub fn turn_left(self) -> HorDir {
        match self {
            VerDir::North => HorDir::West,
            VerDir::South => HorDir::East,
        }
    }
    
    pub fn turn_right(self) -> HorDir {
        match self {
            VerDir::North => HorDir::East,
            VerDir::South => HorDir::West,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct PositionPart<T> {
    direction: T,
    value: usize,
}

impl<T> PositionPart<T> {
    fn deconstruct(self) -> (T, usize) {
        (self.direction, self.value)
    }
}

impl PositionPart<HorDir> {
    pub fn turn_left(self) -> PositionPart<VerDir> {
        PositionPart {
            direction: self.direction.turn_left(),
            value: self.value,
        }
    }
    
    pub fn turn_right(self) -> PositionPart<VerDir> {
        PositionPart {
            direction: self.direction.turn_right(),
            value: self.value,
        }
    }
}

impl PositionPart<VerDir> {
    pub fn turn_left(self) -> PositionPart<HorDir> {
        PositionPart {
            direction: self.direction.turn_left(),
            value: self.value,
        }
    }

    pub fn turn_right(self) -> PositionPart<HorDir> {
        PositionPart {
            direction: self.direction.turn_right(),
            value: self.value,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum FaceDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MovementDirection {
    kind: MovementDirectionKind,
    value: usize,
}

impl FromStr for MovementDirection {
    type Err = error::ParseMovementDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use error::ParseMovementDirectionError;

        let kind = s[0..1]
            .chars()
            .next()
            .ok_or(ParseMovementDirectionError::GeneralInputError)?
            .try_into()?;
        let value = s[1..].parse()?;

        Ok(Self { kind, value })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum MovementDirectionKind {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl TryFrom<char> for MovementDirectionKind {
    type Error = error::ParseMovementDirectionKindError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'N' => Ok(MovementDirectionKind::North),
            'S' => Ok(MovementDirectionKind::South),
            'E' => Ok(MovementDirectionKind::East),
            'W' => Ok(MovementDirectionKind::West),
            'L' => Ok(MovementDirectionKind::Left),
            'R' => Ok(MovementDirectionKind::Right),
            'F' => Ok(MovementDirectionKind::Forward),
            x => Err(error::ParseMovementDirectionKindError::UnknownCharError(x)),
        }
    }
}

pub mod error {
    use std::num::ParseIntError;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum ParseMovementDirectionError {
        #[error("general error with formatting of input")]
        GeneralInputError,
        #[error("error parsing direction kind: {0}")]
        ParseMovementDirectionKindError(#[from] ParseMovementDirectionKindError),
        #[error("error parsing integer value")]
        ParseIntegerError(#[from] ParseIntError),
    }

    #[derive(Debug, Error, Copy, Clone)]
    pub enum ParseMovementDirectionKindError {
        #[error("unknown char: {0}")]
        UnknownCharError(char),
    }
}
