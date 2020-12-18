use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Add;

use itertools::Itertools;

pub use pos_3d::Position3D;
pub use pos_4d::Position4D;
pub use pos_5d::Position5D;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Collection<T>
where
    T: Position,
{
    active_cells: HashSet<T>,
}

impl<T: Position> Collection<T> {
    pub fn multi_step(&self, count: usize) -> Collection<T> {
        assert_ne!(count, 0);

        let mut new = self.step();

        let mut counter = count - 1;
        while counter > 0 {
            counter -= 1;
            new = new.step();
        }

        new
    }

    pub fn step(&self) -> Collection<T> {
        // List for mapping current active cells -> number of active neighbors
        let mut mapping_from_active = HashMap::<T, usize>::new();
        // Collected neighbors of current active cells to check if should activate
        let mut neighbors_to_check = HashSet::<T>::new();

        // Get offsets that are neighbors for easy addition to get neighbor pos
        let neighbor_offsets = (0..T::DIM)
            .map(|_| (-1..=1))
            .multi_cartesian_product()
            .map(T::from)
            .filter(|pos| *pos != T::from((0, 0)))
            .collect::<Vec<_>>();

        // For all currently active cells
        //   add all possible neighbor_offsets
        //   partition by if the neighbors are active or not
        //   count the actives and put them in the mapping_from_active
        //   put the inactive into neighbors_to_check to check later if we should activate them
        for active in &self.active_cells {
            let (active_neighbors, inactive_neighbors): (Vec<T>, Vec<T>) = neighbor_offsets
                .iter()
                .map(|off| *off + *active)
                .partition(|pos| self.active_cells.contains(pos));

            neighbors_to_check.extend(inactive_neighbors.into_iter());
            mapping_from_active.insert(*active, active_neighbors.len());
        }

        // Iterate over mapping_from_active to see which should stay active
        // those with 2 or 3 active neighbors
        let new_active_from_old_active =
            mapping_from_active.into_iter().filter_map(|(key, value)| {
                if matches!(value, 2 | 3) {
                    Some(key)
                } else {
                    None
                }
            });

        // List for mapping current inactive neighbor cells of active cells -> number of active neighbors
        let mut mapping_from_inactive = HashMap::new();

        // For all inactive neighbor cells of active cells
        //   add all possible neighbor_offsets
        //   count the active neighbor cells and put them in mapping_from_inactive
        for position in neighbors_to_check {
            let count = neighbor_offsets
                .iter()
                .map(|off| *off + position)
                .filter(|pos| self.active_cells.contains(pos))
                .count();
            mapping_from_inactive.insert(position, count);
        }

        // Iterate over mapping_from_inactive to see which should be activated in next frame
        // those with 3 active neighbors
        let new_active_from_old_inactive =
            mapping_from_inactive
                .into_iter()
                .filter_map(
                    |(key, value)| {
                        if matches!(value, 3) {
                            Some(key)
                        } else {
                            None
                        }
                    },
                );

        // take all still active and newly activated cells together
        let new_active = new_active_from_old_active
            .chain(new_active_from_old_inactive)
            .collect();

        Collection {
            active_cells: new_active,
        }
    }

    pub fn count_active(&self) -> usize {
        self.active_cells.len()
    }
}

impl<S: AsRef<str>, T: Position> From<S> for Collection<T> {
    fn from(s: S) -> Self {
        let s = s.as_ref();

        // parsing input to Position
        // take lines -> numerate them => y position
        //   for each line
        //   take char and numerate them => x position
        //   and discard any that are not '#'
        // collect remaining x and y combinations
        let active_cells = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| if c == '#' { Some(x) } else { None })
                    .map(|x| (x as isize, y as isize))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .map(T::from)
            .collect::<HashSet<_>>();

        Collection { active_cells }
    }
}

/// Trait that returns the dimension
pub trait Dim {
    const DIM: usize;

    fn get_dim() -> usize {
        Self::DIM
    }
}

/// Required Traits for usage as position data in a Collection
pub trait Position:
    Add<Output = Self> + Copy + Dim + Eq + From<(isize, isize)> + From<Vec<isize>> + Hash + PartialEq
{
}

/// Implementation of 3D Position Data
pub mod pos_3d {
    use std::ops::Add;

    use super::Dim;
    use crate::cube::Position;

    #[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
    pub struct Position3D {
        pub x: isize,
        pub y: isize,
        pub z: isize,
    }

    impl Dim for Position3D {
        const DIM: usize = 3;
    }

    impl Add for Position3D {
        type Output = Position3D;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl From<(isize, isize)> for Position3D {
        fn from((x, y): (isize, isize)) -> Self {
            Position3D { x, y, z: 0 }
        }
    }

    impl From<Vec<isize>> for Position3D {
        fn from(vec: Vec<isize>) -> Self {
            assert_eq!(vec.len(), Self::DIM);

            Self {
                x: vec[0],
                y: vec[1],
                z: vec[2],
            }
        }
    }

    impl Position for Position3D {}
}

/// Implementation of 4D Position Data
pub mod pos_4d {
    use std::ops::Add;

    use super::{Dim, Position};

    #[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
    pub struct Position4D {
        pub x: isize,
        pub y: isize,
        pub z: isize,
        pub w: isize,
    }

    impl Dim for Position4D {
        const DIM: usize = 4;
    }

    impl Add for Position4D {
        type Output = Position4D;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
            }
        }
    }

    impl From<(isize, isize)> for Position4D {
        fn from((x, y): (isize, isize)) -> Self {
            Self { x, y, z: 0, w: 0 }
        }
    }

    impl From<Vec<isize>> for Position4D {
        fn from(vec: Vec<isize>) -> Self {
            assert_eq!(vec.len(), Self::DIM);

            Self {
                x: vec[0],
                y: vec[1],
                z: vec[2],
                w: vec[3],
            }
        }
    }

    impl Position for Position4D {}
}

/// EXTRA: Implementation of 5D Position Data
pub mod pos_5d {
    use std::ops::Add;

    use super::{Dim, Position};

    #[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
    pub struct Position5D {
        pub x: isize,
        pub y: isize,
        pub z: isize,
        pub w: isize,
        pub v: isize,
    }

    impl Dim for Position5D {
        const DIM: usize = 5;
    }

    impl Add for Position5D {
        type Output = Position5D;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
                v: self.v + rhs.v,
            }
        }
    }

    impl From<(isize, isize)> for Position5D {
        fn from((x, y): (isize, isize)) -> Self {
            Self {
                x,
                y,
                z: 0,
                w: 0,
                v: 0,
            }
        }
    }

    impl From<Vec<isize>> for Position5D {
        fn from(vec: Vec<isize>) -> Self {
            assert_eq!(vec.len(), Self::DIM);

            Self {
                x: vec[0],
                y: vec[1],
                z: vec[2],
                w: vec[3],
                v: vec[4],
            }
        }
    }

    impl Position for Position5D {}
}
