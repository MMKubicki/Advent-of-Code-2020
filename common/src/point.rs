use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Point<T>
where
    T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign,
{
    pub x: T,
    pub y: T,
}

impl<T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign> From<(T, T)>
    for Point<T>
{
    fn from(input: (T, T)) -> Self {
        Self {
            x: input.0,
            y: input.1,
        }
    }
}

impl<T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign> Add<(T, T)> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: (T, T)) -> Self::Output {
        self + Point::from(rhs)
    }
}

impl<T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign> Sub<(T, T)> for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: (T, T)) -> Self::Output {
        self - Point::from(rhs)
    }
}

impl Add<Point<isize>> for Point<usize> {
    type Output = Point<isize>;

    fn add(self, rhs: Point<isize>) -> Self::Output {
        Point {
            x: self.x as isize + rhs.x,
            y: self.y as isize + rhs.y,
        }
    }
}

impl Sub<Point<isize>> for Point<usize> {
    type Output = Point<isize>;

    fn sub(self, rhs: Point<isize>) -> Self::Output {
        Point {
            x: self.x as isize - rhs.x,
            y: self.y as isize - rhs.y,
        }
    }
}
