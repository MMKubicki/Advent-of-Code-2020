use std::collections::{HashMap, HashSet};
use std::fmt;
use std::num::ParseIntError;
use std::ops::{Index, RangeInclusive};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Input {
    rules: TicketRules,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

impl Input {
    pub fn get_general_invalid_values_of_other_tickets(&self) -> Vec<usize> {
        self.other_tickets
            .iter()
            .map(|ticket| self.rules.get_general_invalid_values(ticket))
            .flatten()
            .collect()
    }

    pub fn cleanup_invalid_tickets(&mut self) {
        let rules = self.rules.clone();
        self.other_tickets.retain(|ticket| rules.is_valid(ticket));
    }

    pub fn try_determine_field(&self) -> Vec<String> {
        // Determine possibilities
        let mut possible_mappings = vec![self.rules.get_all_rule_indexes(); 20];
        for ticket in &self.other_tickets {
            for (idx, value) in ticket.values.iter().enumerate() {
                possible_mappings[idx] =
                    &possible_mappings[idx] & &self.rules.get_valid_rule_for(*value);
            }
        }

        // Respect own ticket?
        for (idx, value) in self.my_ticket.values.iter().enumerate() {
            possible_mappings[idx] =
                &possible_mappings[idx] & &self.rules.get_valid_rule_for(*value);
        }

        // Cleanup
        let mut checked_rules = HashSet::new();
        loop {
            // Get all rule identifier from possible_mappings where there is just one rule in the set
            // -> position is mapped
            let _ = possible_mappings
                .iter()
                .filter_map(|rule_set| {
                    if rule_set.len() == 1 {
                        Some(rule_set.iter().next().unwrap())
                    } else {
                        None
                    }
                })
                .map(|rule| {
                    checked_rules.insert(rule.to_owned());
                })
                .collect::<()>();

            // remove all rules that are final mapped from all other mappings that are bigger than 1
            let _ = possible_mappings
                .iter_mut()
                .filter(|rule_set| rule_set.len() > 1)
                .map(|rule_set| {
                    rule_set.retain(|elem| !checked_rules.contains(elem));
                })
                .collect::<()>();

            // if all is final mapped exit
            if possible_mappings.iter().all(|rule_set| rule_set.len() == 1) {
                break;
            }
        }

        // adjust return format
        possible_mappings
            .into_iter()
            .map(|v| v.into_iter().next().unwrap())
            .collect()
    }

    pub fn part_1_result(&self) -> usize {
        self.get_general_invalid_values_of_other_tickets()
            .iter()
            .sum()
    }

    pub fn part_2_result(&self) -> usize {
        let mappings = self.try_determine_field();
        mappings
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| {
                if v.contains("departure") {
                    Some(idx)
                } else {
                    None
                }
            })
            .map(|idx| self.my_ticket[idx])
            .product()
    }
}

impl FromStr for Input {
    type Err = error::ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use error::ParseInputError;

        let mut split_input = s.split("\n\n");

        let rules = split_input
            .next()
            .ok_or(ParseInputError::MalformedInput)?
            .parse()?;

        let my_ticket = split_input
            .next()
            .ok_or(ParseInputError::MalformedInput)?
            .strip_prefix("your ticket:\n")
            .ok_or(ParseInputError::MalformedInput)?
            .parse()?;

        let other_tickets = split_input
            .next()
            .ok_or(ParseInputError::MalformedInput)?
            .strip_prefix("nearby tickets:\n")
            .ok_or(ParseInputError::MalformedInput)?
            .split('\n')
            .filter(|v| !v.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Input {
            rules,
            my_ticket,
            other_tickets,
        })
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rules: {}, other tickets: {}, my ticket: {:?}",
            self.rules.rules.len(),
            self.other_tickets.len(),
            self.my_ticket
        )
    }
}

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct TicketRules {
    rules: HashMap<String, Vec<RangeInclusive<usize>>>,
}

impl TicketRules {
    pub fn is_valid(&self, ticket: &Ticket) -> bool {
        ticket.values.iter().all(|value| {
            self.rules
                .iter()
                .any(|(_, ranges)| ranges.iter().any(|range| range.contains(&value)))
        })
    }

    pub fn get_general_invalid_values(&self, ticket: &Ticket) -> Vec<usize> {
        let mut result = Vec::new();

        for value in ticket.values.iter() {
            let res = self
                .rules
                .values()
                .any(|rule| rule.iter().any(|range| range.contains(value)));

            if !res {
                result.push(*value);
            }
        }

        result
    }

    pub fn get_valid_rule_for(&self, value: usize) -> HashSet<String> {
        let mut result = HashSet::new();
        for (idx, ranges) in &self.rules {
            if ranges.iter().any(|range| range.contains(&value)) {
                result.insert(idx.to_owned());
            }
        }

        result
    }

    pub fn get_all_rule_indexes(&self) -> HashSet<String> {
        self.rules.keys().cloned().collect()
    }
}

impl FromStr for TicketRules {
    type Err = error::ParseTicketRulesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use error::ParseTicketRulesError;

        let mut rules = HashMap::new();

        for line in s.lines() {
            let mut split = line.split(": ");
            // Key
            let key = split
                .next()
                .ok_or(ParseTicketRulesError::MalformedInput)?
                .to_owned();

            // Ranges
            let ranges = split
                .collect::<String>()
                .split(" or ")
                .map(|range_str| {
                    let range = range_str.split('-').collect::<Vec<_>>();
                    if range.len() != 2 {
                        Err(ParseTicketRulesError::MalformedInput)
                    } else {
                        let values = range
                            .into_iter()
                            .map(str::parse::<usize>)
                            .collect::<Result<Vec<usize>, ParseIntError>>()?;

                        if values.len() != 2 {
                            Err(ParseTicketRulesError::MalformedInput)
                        } else {
                            Ok(values[0]..=values[1])
                        }
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;

            rules.insert(key, ranges);
        }

        Ok(TicketRules { rules })
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Hash)]
pub struct Ticket {
    values: Vec<usize>,
}

impl FromStr for Ticket {
    type Err = error::ParseTicketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use error::ParseTicketError;

        let values = s
            .split(',')
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()?;

        if values.len() != 20 {
            Err(ParseTicketError::NotEnoughValues {
                expected: 20,
                is: values.len(),
            })
        } else {
            Ok(Ticket { values })
        }
    }
}

impl Index<usize> for Ticket {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        self.values.index(index)
    }
}

pub mod error {
    use std::num::ParseIntError;
    use thiserror::Error;

    #[derive(Debug, Error, Clone, Eq, PartialEq)]
    pub enum ParseInputError {
        #[error("malformed input")]
        MalformedInput,
        #[error("error parsing ticket rules: {error}")]
        ParseTicketRulesError {
            #[from]
            error: ParseTicketRulesError,
        },
        #[error("error parsing ticket: {error}")]
        ParseTicketError {
            #[from]
            error: ParseTicketError,
        },
    }

    #[derive(Debug, Error, Clone, Eq, PartialEq)]
    pub enum ParseTicketRulesError {
        #[error("malformed input")]
        MalformedInput,
        #[error("error parsing integer: {error}")]
        ParseIntError {
            #[from]
            error: ParseIntError,
        },
    }

    #[derive(Debug, Error, Clone, Eq, PartialEq)]
    pub enum ParseTicketError {
        #[error("input not long enough. expected {expected} values. found: {is} values")]
        NotEnoughValues { expected: usize, is: usize },
        #[error("error parsing integer: {error}")]
        ParseIntError {
            #[from]
            error: ParseIntError,
        },
    }
}
