use super::{
    FaceDirection, HorDir, MovementDirection, MovementDirectionKind, PositionPart, VerDir,
};
use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Part1Position {
    horizontal: PositionPart<HorDir>,
    vertical: PositionPart<VerDir>,
    facing: FaceDirection,
}

impl Part1Position {
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
        match self.horizontal.deconstruct() {
            (dir, _) if dir == to => self.horizontal.value += value,
            (_, x) if x >= value => self.horizontal.value -= value,
            (_, x) if x < value => {
                self.horizontal.direction = to;
                self.horizontal.value = value - x;
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
        match self.vertical.deconstruct() {
            (dir, _) if dir == to => self.vertical.value += value,
            (_, x) if x >= value => self.vertical.value -= value,
            (_, x) if x < value => {
                self.vertical.direction = to;
                self.vertical.value = value - x;
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
        match self.facing {
            FaceDirection::North => self.move_north(value),
            FaceDirection::South => self.move_south(value),
            FaceDirection::East => self.move_east(value),
            FaceDirection::West => self.move_west(value),
        }
    }

    fn turn_left(&mut self, value: usize) {
        assert_eq!(value % 90, 0);

        let mut temp = value;
        while temp > 0 {
            temp -= 90;

            self.facing = match self.facing {
                FaceDirection::North => FaceDirection::West,
                FaceDirection::South => FaceDirection::East,
                FaceDirection::East => FaceDirection::North,
                FaceDirection::West => FaceDirection::South,
            };
        }
    }

    fn turn_right(&mut self, value: usize) {
        assert_eq!(value % 90, 0);

        let mut temp = value;
        while temp > 0 {
            temp -= 90;

            self.facing = match self.facing {
                FaceDirection::North => FaceDirection::East,
                FaceDirection::South => FaceDirection::West,
                FaceDirection::East => FaceDirection::South,
                FaceDirection::West => FaceDirection::North,
            }
        }
    }
}

impl AddAssign<&MovementDirection> for Part1Position {
    fn add_assign(&mut self, rhs: &MovementDirection) {
        self.move_by(rhs);
    }
}

impl<'a, T: Iterator<Item = &'a MovementDirection>> AddAssign<T> for Part1Position {
    fn add_assign(&mut self, rhs: T) {
        for dir in rhs {
            self.move_by(dir)
        }
    }
}

impl Default for Part1Position {
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
            facing: FaceDirection::East,
        }
    }
}
