use std::ops::Add;

pub(crate) struct MoveDirection {
    pub(crate) vertical: isize,
    pub(crate) horizontal: isize,
}

impl MoveDirection {
    pub(crate) fn Forward(steps: usize) -> MoveDirection {
        MoveDirection {
            vertical: steps as isize,
            horizontal: 0,
        }
    }
    pub(crate) fn Backward(steps: usize) -> MoveDirection {
        MoveDirection {
            vertical: -(steps as isize),
            horizontal: 0,
        }
    }
    pub(crate) fn Right(steps: usize) -> MoveDirection {
        MoveDirection {
            vertical: 0,
            horizontal: steps as isize,
        }
    }
    pub(crate) fn Left(steps: usize) -> MoveDirection {
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
