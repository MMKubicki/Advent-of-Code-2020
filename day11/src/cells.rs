use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fmt::Write;
use std::str::FromStr;

use common::Point;

use error::ParseCellStateError;

#[derive(Eq, PartialEq, Clone, Default, Debug, Hash)]
pub struct CellMap {
    width: usize,
    height: usize,
    content: Vec<Vec<CellState>>,
}

impl CellMap {
    /// Take a step with the rules of Part 1
    /// For each Cell do:
    ///     look at value of cell and do:
    ///         Floor -> directly copy it to new map
    ///         Invalid -> skip
    ///         EmptySeat | OccupiedSeat ->
    ///             Count surrounding OccupiedSeats
    ///             if count = 0 -> write OccupiedSeat to new map
    ///             if count >= 4 -> write EmptySeat to new map
    ///             else copy cell value to new map
    ///             go to next cell
    pub fn step_part_1(&self) -> CellMap {
        let mut result = self.clone();

        for y in 0..(self.height as isize) {
            for x in 0..(self.width as isize) {
                if matches!(self.get_at(x, y), CellState::Floor | CellState::Invalid) {
                    continue;
                }

                let mut counter = 0;
                for d_y in (y - 1)..=(y + 1) {
                    for d_x in (x - 1)..=(x + 1) {
                        if d_x == x && d_y == y {
                            continue;
                        }

                        if self.get_at(d_x, d_y) == CellState::OccupiedSeat {
                            counter += 1;
                        }
                    }
                }

                let y_checked = y as usize;
                let x_checked = x as usize;

                match (self.get_at(x, y), counter) {
                    (CellState::EmptySeat, 0) => {
                        result.set_at(x_checked, y_checked, CellState::OccupiedSeat)
                    }
                    (CellState::OccupiedSeat, adjacent) if adjacent >= 4 => {
                        result.set_at(x_checked, y_checked, CellState::EmptySeat)
                    }
                    _ => {}
                }
            }
        }

        result
    }

    /// Take a step with the rules of Part 2
    /// Basically the same as Part 1 but with looking at first seat in
    /// possible direction instead of just surrounding and
    /// it takes 5 to write EmptySeat
    ///
    /// For each Cell do:
    ///     look at value of cell and do:
    ///         Floor -> directly copy it to new map
    ///         Invalid -> skip
    ///         EmptySeat | OccupiedSeat ->
    ///             Count (first seat while looking in all 8 possible directions) == OccupiedSeat
    ///             if count = 0 -> write OccupiedSeat to new map
    ///             if count >= 5 -> write EmptySeat to new map
    ///             else copy cell value to new map
    ///             go to next cell
    pub fn step_part_2(&self) -> CellMap {
        let mut result = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                if matches!(
                    self.get_at(x as isize, y as isize),
                    CellState::Floor | CellState::Invalid
                ) {
                    continue;
                }

                let mut counter = 0;
                for d_y in -1..=1 {
                    for d_x in -1..=1 {
                        if d_x == 0 && d_y == 0 {
                            continue;
                        }

                        if self.look_at(x, y, d_x, d_y) == CellState::OccupiedSeat {
                            counter += 1;
                        }
                    }
                }

                match (self.get_at(x as isize, y as isize), counter) {
                    (CellState::EmptySeat, 0) => result.set_at(x, y, CellState::OccupiedSeat),
                    (CellState::OccupiedSeat, adjacent) if adjacent >= 5 => {
                        result.set_at(x, y, CellState::EmptySeat)
                    }
                    _ => {}
                }
            }
        }

        result
    }

    /// Count all occurrences of state in CellMap
    pub fn count_in_state(&self, state: CellState) -> usize {
        self.content
            .iter()
            .map(|l| l.iter().filter(|item| **item == state))
            .flatten()
            .count()
    }

    /// Looks in the direction defined by (dir_x, dir_y) starting from (pos_x, pos_y)
    /// Floor cells get skipped until the first hit of an Invalid cell, EmptySeat or OccupiedSeat
    /// On Invalid Cell return Floor
    /// On EmptySeat or OccupiedSeat return its value
    pub fn look_at(&self, pos_x: usize, pos_y: usize, dir_x: isize, dir_y: isize) -> CellState {
        let delta: Point<_> = (dir_x, dir_y).into();
        let mut current_pos: Point<_> = (pos_x as isize, pos_y as isize).into();

        loop {
            current_pos += delta;
            let res = self.get_at(current_pos.x, current_pos.y);
            match res {
                CellState::Floor => continue,
                CellState::Invalid => return CellState::Floor,
                x if matches!(x, CellState::EmptySeat | CellState::OccupiedSeat) => return x,
                _ => unreachable!(),
            }
        }
    }

    /// Check if x and y are valid
    /// if not return Invalid -> signals position outside of Map
    /// if valid:
    /// return CellState at position (x,y)
    pub fn get_at(&self, x: isize, y: isize) -> CellState {
        if x < 0 || y < 0 {
            return CellState::Invalid;
        }

        let x_neg_checked = x as usize;
        let y_neg_checked = y as usize;

        if x_neg_checked >= self.width || y_neg_checked >= self.height {
            return CellState::Invalid;
        }

        let x_checked = x_neg_checked;
        let y_checked = y_neg_checked;

        self.content[y_checked][x_checked]
    }

    /// Sets CellState at position (x,y) to state
    pub fn set_at(&mut self, x: usize, y: usize, state: CellState) {
        self.content[y][x] = state
    }
}

// Parsing Input into CellMap
// split by lines
// parse each char in a line to CellState
impl FromStr for CellMap {
    type Err = error::ParseCellMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let content = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(char::try_into)
                    .collect::<Result<Vec<CellState>, ParseCellStateError>>()
            })
            .collect::<Result<Vec<Vec<CellState>>, ParseCellStateError>>()?;

        let height = content.len();
        let width = content[0].len();

        Ok(Self {
            width,
            height,
            content,
        })
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum CellState {
    Floor,
    EmptySeat,
    OccupiedSeat,
    Invalid,
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Floor
    }
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellState::Floor => f.write_char('.'),
            CellState::EmptySeat => f.write_char('L'),
            CellState::OccupiedSeat => f.write_char('#'),
            CellState::Invalid => f.write_char('I'),
        }
    }
}

impl TryFrom<char> for CellState {
    type Error = error::ParseCellStateError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(CellState::Floor),
            'L' => Ok(CellState::EmptySeat),
            '#' => Ok(CellState::OccupiedSeat),
            x => Err(error::ParseCellStateError::UnknownChar(x)),
        }
    }
}

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum ParseCellStateError {
        #[error("unknown char: {0}")]
        UnknownChar(char),
    }

    #[derive(Debug, Error)]
    pub enum ParseCellMapError {
        #[error("unknown char: {0}")]
        CellStateError(#[from] ParseCellStateError),
    }
}
