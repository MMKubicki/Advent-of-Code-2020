use super::{HorDir, MovementDirection, MovementDirectionKind, PositionPart, VerDir};
use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Part2Position {
    horizontal: PositionPart<HorDir>,
    vertical: PositionPart<VerDir>,
    waypoint: Waypoint,
}

impl Part2Position {
    pub fn manhatten_distance(&self) -> usize {
        self.horizontal.value + self.vertical.value
    }

    pub fn move_by(&mut self, dir: &MovementDirection) {
        match dir.kind {
            MovementDirectionKind::North => self.move_north(dir.value),
            MovementDirectionKind::South => self.move_south(dir.value),
            MovementDirectionKind::East => self.move_east(dir.value),
            MovementDirectionKind::West => self.move_west(dir.value),
            MovementDirectionKind::Left => self.turn_left(dir.value),
            MovementDirectionKind::Right => self.turn_right(dir.value),
            MovementDirectionKind::Forward => self.move_forward(dir.value),
        }
    }

    fn move_horizontal(&mut self, to: HorDir, value: usize) {
        match self.waypoint.horizontal.deconstruct() {
            (dir, _) if dir == to => self.waypoint.horizontal.value += value,
            (_, x) if x >= value => self.waypoint.horizontal.value -= value,
            (_, x) if x < value => {
                self.waypoint.horizontal.direction = to;
                self.waypoint.horizontal.value = value - x;
            }
            _ => unreachable!(),
        }
    }

    fn move_east(&mut self, value: usize) {
        self.move_horizontal(HorDir::East, value);
    }

    fn move_west(&mut self, value: usize) {
        self.move_horizontal(HorDir::West, value);
    }

    fn move_vertical(&mut self, to: VerDir, value: usize) {
        match self.waypoint.vertical.deconstruct() {
            (dir, _) if dir == to => self.waypoint.vertical.value += value,
            (_, x) if x >= value => self.waypoint.vertical.value -= value,
            (_, x) if x < value => {
                self.waypoint.vertical.direction = to;
                self.waypoint.vertical.value = value - x;
            }
            _ => unreachable!(),
        }
    }

    fn move_north(&mut self, value: usize) {
        self.move_vertical(VerDir::North, value);
    }

    fn move_south(&mut self, value: usize) {
        self.move_vertical(VerDir::South, value);
    }

    fn move_forward(&mut self, value: usize) {
        for _ in 0..value {
            //horizontal
            let (dir_hor, pos_hor) = self.horizontal.deconstruct();
            match self.waypoint.horizontal.deconstruct() {
                (dir, x) if dir == dir_hor => self.horizontal.value += x,
                (_, x) if x < pos_hor => self.horizontal.value -= x,
                (dir, x) if x >= pos_hor => {
                    self.horizontal.direction = dir;
                    self.horizontal.value = x - pos_hor;
                }
                _ => unreachable!(),
            }

            //vertical
            let (dir_ver, pos_ver) = self.vertical.deconstruct();
            match self.waypoint.vertical.deconstruct() {
                (dir, x) if dir == dir_ver => self.vertical.value += x,
                (_, x) if x < pos_ver => self.vertical.value -= x,
                (dir, x) if x >= pos_ver => {
                    self.vertical.direction = dir;
                    self.vertical.value = x - pos_ver;
                }
                _ => unreachable!(),
            }
        }
    }

    fn turn_left(&mut self, value: usize) {
        assert_eq!(value % 90, 0);

        let mut temp = value;
        while temp > 0 {
            temp -= 90;

            let new_hor = self.waypoint.vertical.turn_left();
            let new_ver = self.waypoint.horizontal.turn_left();
            self.waypoint.horizontal = new_hor;
            self.waypoint.vertical = new_ver;
        }
    }

    fn turn_right(&mut self, value: usize) {
        assert_eq!(value % 90, 0);

        let mut temp = value;
        while temp > 0 {
            temp -= 90;

            let new_hor = self.waypoint.vertical.turn_right();
            let new_ver = self.waypoint.horizontal.turn_right();
            self.waypoint.horizontal = new_hor;
            self.waypoint.vertical = new_ver;
        }
    }
}

impl AddAssign<&MovementDirection> for Part2Position {
    fn add_assign(&mut self, rhs: &MovementDirection) {
        self.move_by(rhs);
    }
}

impl<'a, T: Iterator<Item = &'a MovementDirection>> AddAssign<T> for Part2Position {
    fn add_assign(&mut self, rhs: T) {
        for dir in rhs {
            self.move_by(dir)
        }
    }
}

impl Default for Part2Position {
    fn default() -> Self {
        Self {
            horizontal: PositionPart {
                direction: HorDir::East,
                value: 0,
            },
            vertical: PositionPart {
                direction: VerDir::North,
                value: 0,
            },
            waypoint: Waypoint {
                horizontal: PositionPart {
                    direction: HorDir::East,
                    value: 10,
                },
                vertical: PositionPart {
                    direction: VerDir::North,
                    value: 1,
                },
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Waypoint {
    horizontal: PositionPart<HorDir>,
    vertical: PositionPart<VerDir>,
}
