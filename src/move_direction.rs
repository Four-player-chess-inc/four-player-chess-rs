use std::ops::Add;

pub(crate) struct MoveDirection {
    pub(crate) vertical: isize,
    pub(crate) horizontal: isize,
}

impl MoveDirection {
    pub(crate) fn forward(steps: usize) -> MoveDirection {
        MoveDirection {
            vertical: steps as isize,
            horizontal: 0,
        }
    }
    pub(crate) fn backward(steps: usize) -> MoveDirection {
        MoveDirection {
            vertical: -(steps as isize),
            horizontal: 0,
        }
    }
    pub(crate) fn right(steps: usize) -> MoveDirection {
        MoveDirection {
            vertical: 0,
            horizontal: steps as isize,
        }
    }
    pub(crate) fn left(steps: usize) -> MoveDirection {
        MoveDirection {
            vertical: 0,
            horizontal: -(steps as isize),
        }
    }
}

impl Add for MoveDirection {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            vertical: self.vertical + other.vertical,
            horizontal: self.horizontal + other.horizontal,
        }
    }
}
