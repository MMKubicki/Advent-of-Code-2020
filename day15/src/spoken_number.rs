use std::collections::{HashMap, VecDeque};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SpokenNumber {
    init: VecDeque<usize>,
    count: usize,
    last: usize,
    memory: Memory,
}

impl SpokenNumber {
    pub fn new(init: Vec<usize>) -> Self {
        Self {
            init: VecDeque::from(init),
            count: 0,
            last: 0,
            memory: Memory::default(),
        }
    }
}

impl Iterator for SpokenNumber {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if !self.init.is_empty() {
            self.init.pop_front().unwrap()
        } else {
            match self.memory.get(self.last) {
                None => 0,
                Some(list) => list[0] - list[1],
            }
        };

        self.last = next;
        self.count += 1;
        self.memory.append(next, self.count);
        Some(next)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
struct Memory {
    mem: HashMap<usize, VecDeque<usize>>,
}

impl Memory {
    pub fn append(&mut self, key: usize, pos: usize) {
        match self.mem.get_mut(&key) {
            None => {
                self.mem.insert(key, VecDeque::from(vec![pos]));
            }
            Some(dq) => {
                dq.push_front(pos);
                while dq.len() > 2 {
                    dq.pop_back();
                }
            }
        }
    }

    pub fn get(&self, key: usize) -> Option<Vec<usize>> {
        match self.mem.get(&key) {
            None => None,
            Some(v) if v.len() > 2 => None,
            Some(v) if v.len() < 2 => None,
            Some(v) => Some(v.iter().copied().collect()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut iter = SpokenNumber::new(vec![0, 3, 6]);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(0));
    }

    #[test]
    fn part1_example() {
        let input = vec![
            (vec![0, 3, 6], 436),
            (vec![1, 3, 2], 1),
            (vec![2, 1, 3], 10),
            (vec![1, 2, 3], 27),
            (vec![2, 3, 1], 78),
            (vec![3, 2, 1], 438),
            (vec![3, 1, 2], 1836),
        ];
        for (start, result) in input.into_iter() {
            assert_eq!(apply_x(start, 2020), Some(result));
        }
    }

    #[test]
    fn part2_1() {
        assert_eq!(apply_x(vec![0, 3, 6], 30000000), Some(175594))
    }

    #[test]
    fn part2_2() {
        assert_eq!(apply_x(vec![1, 3, 2], 30000000), Some(2578))
    }

    #[test]
    fn part2_3() {
        assert_eq!(apply_x(vec![2, 1, 3], 30000000), Some(3544142))
    }

    #[test]
    fn part2_4() {
        assert_eq!(apply_x(vec![1, 2, 3], 30000000), Some(261214))
    }

    #[test]
    fn part2_5() {
        assert_eq!(apply_x(vec![2, 3, 1], 30000000), Some(6895259))
    }

    #[test]
    fn part2_6() {
        assert_eq!(apply_x(vec![3, 2, 1], 30000000), Some(18))
    }

    #[test]
    fn part2_7() {
        assert_eq!(apply_x(vec![3, 1, 2], 30000000), Some(362))
    }

    fn apply_x(start: Vec<usize>, x: usize) -> Option<usize> {
        SpokenNumber::new(start).nth(x - 1)
    }
}
