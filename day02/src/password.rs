use std::convert::TryFrom;
use std::str::FromStr;

pub use error::Location;
pub use error::RequirementAndPasswordParseError;
pub use error::RequirementParseError;

/// Wrapper for password requirements
#[derive(Default, Debug)]
pub struct Requirement {
    min: usize,
    max: usize,
    char: char,
}

impl Requirement {
    pub fn new(min: usize, max: usize, char: char) -> Self {
        Self { min, max, char }
    }

    /// Check password with rules of part 1.
    /// The given char should only be present between min and max times in the password
    pub fn check_password_part1<T: AsRef<str>>(&self, password: T) -> bool {
        // count all chars that equal saved char
        let count = password
            .as_ref()
            .chars()
            .map(|c| c.eq(&self.char))
            .filter(|b| *b)
            .count();

        // check that count is between min and max
        self.min <= count && count <= self.max
    }

    /// Check password with rules of part 2.
    /// The given char should be at either min position or max position, but not both
    pub fn check_password_part2<T: AsRef<str>>(&self, password: T) -> bool {
        let pwd = password.as_ref();

        assert!(self.min <= self.max);

        // passwords 1 indexed (not 0) -> compensate
        let min = self.min - 1;
        let max = self.max - 1;
        let diff = max - min - 1;

        // check length before access
        if pwd.len() <= min || pwd.len() <= max {
            return false;
        }

        // get chars from password
        let mut pwd_iter = pwd.chars();
        let min_char = pwd_iter.nth(min).unwrap();
        let max_char = pwd_iter.nth(diff).unwrap();

        // xor check results, because just one is allowed to be equal to check char
        (min_char == self.char) ^ (max_char == self.char)
    }
}

/// Comfort wrapper for Requirement::check_password_part1
pub fn check_password_part1(input: &(Requirement, Password)) -> bool {
    input.0.check_password_part1(&input.1)
}

/// Comfort wrapper for Requirement::check_password_part2
pub fn check_password_part2(input: &(Requirement, Password)) -> bool {
    input.0.check_password_part2(&input.1)
}

/// Parse Requirement from string
impl FromStr for Requirement {
    type Err = error::RequirementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Requirement format:
        // MIN-MAX CHAR

        // MIN & MAX are Integer
        // CHAR is Character

        // split by space between MAX and CHAR
        // check for correctness
        // MIN-MAX should be in index 0
        // CHAR should be in index 1
        let space_split = s.split(' ').collect::<Vec<_>>();
        if space_split.len() != 2 {
            return Err(RequirementParseError::Malformed(
                Location::SpaceSplitBetweenNumeralsAndChar,
            ));
        }

        // MIN-MAX by hyphen and check
        // MIN should be in index 0
        // MAX should be in index 1
        let hyphen_split = space_split[0].split('-').collect::<Vec<_>>();
        if hyphen_split.len() != 2 {
            return Err(RequirementParseError::Malformed(
                Location::HyphenSplitBetweenNumerals,
            ));
        }

        // Parse MIN
        let min = match hyphen_split[0].parse::<usize>() {
            Ok(value) => value,
            Err(e) => return Err(RequirementParseError::MinParseError(e)),
        };

        // Parse MAX
        let max = match hyphen_split[1].parse::<usize>() {
            Ok(value) => value,
            Err(e) => return Err(RequirementParseError::MaxParseError(e)),
        };

        // Parse CHAR (check that just one char is submitted)
        let char_list = space_split[1].chars().collect::<Vec<_>>();
        if char_list.len() != 1 {
            return Err(RequirementParseError::Malformed(Location::MoreThanOneChar));
        }
        let char = char_list[0];

        Ok(Requirement::new(min, max, char))
    }
}

/// Just a wrapper for a borrowed string
#[derive(Default, Debug)]
pub struct Password<'a> {
    password: &'a str,
}

impl<'a> Password<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { password: input }
    }
}

impl<'a> AsRef<str> for Password<'a> {
    fn as_ref(&self) -> &str {
        self.password
    }
}

impl<'a> From<&'a str> for Password<'a> {
    fn from(s: &'a str) -> Self {
        Password::new(s)
    }
}

#[derive(Default, Debug)]
pub struct RequirementAndPassword<'a> {
    requirement: Requirement,
    password: Password<'a>,
}

impl<'a> RequirementAndPassword<'a> {
    pub fn new(requirement: Requirement, password: Password<'a>) -> Self {
        Self {
            requirement,
            password,
        }
    }

    pub fn deconstruct(self) -> (Requirement, Password<'a>) {
        (self.requirement, self.password)
    }
}

/// Parse requirement and password while using part of supplied string for password
impl<'a> TryFrom<&'a str> for RequirementAndPassword<'a> {
    type Error = RequirementAndPasswordParseError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        // Requirement and Password format:
        // REQUIREMENT: PASSWORD

        // REQUIREMENT is Requirement
        // PASSWORD is Password

        // split by ": " and check
        let split = s.split(": ").collect::<Vec<&'a str>>();
        if split.len() != 2 {
            return Err(RequirementAndPasswordParseError::Malformed(
                Location::SplitBetweenRequirementAndPassword,
            ));
        }

        // parse requirement
        let requirement = split[0].parse::<Requirement>()?;
        // take password
        let password = split[1];

        Ok(RequirementAndPassword::new(requirement, password.into()))
    }
}

pub mod error {
    use std::fmt;
    use std::num;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum RequirementParseError {
        #[error("input string malformed: {0}")]
        Malformed(Location),

        #[error("failed to parse min requirement: {0}")]
        MinParseError(num::ParseIntError),

        #[error("failed to parse max requirement: {0}")]
        MaxParseError(num::ParseIntError),
    }

    #[derive(Debug)]
    pub enum Location {
        SpaceSplitBetweenNumeralsAndChar,
        HyphenSplitBetweenNumerals,
        MoreThanOneChar,
        SplitBetweenRequirementAndPassword,
    }

    impl fmt::Display for Location {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Location::SpaceSplitBetweenNumeralsAndChar => {
                    f.write_str("space split between numerals and char")
                }
                Location::HyphenSplitBetweenNumerals => {
                    f.write_str("hyphen split between numerals")
                }
                Location::MoreThanOneChar => f.write_str("more than one char given as requirement"),
                Location::SplitBetweenRequirementAndPassword => {
                    f.write_str("\": \" split between requirement and password")
                }
            }
        }
    }

    #[derive(Error, Debug)]
    pub enum RequirementAndPasswordParseError {
        #[error("input string malformed: {0}")]
        Malformed(Location),

        #[error("failed parsing requirement: {0}")]
        RequirementParseError(#[from] RequirementParseError),
    }
}
