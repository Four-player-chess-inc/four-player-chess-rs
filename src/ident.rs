use crate::move_direction::MoveDirection;
use std::iter::Iterator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ident {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug)]
pub(crate) struct Step {
    pub(crate) col_inc: isize,
    pub(crate) row_inc: isize,
}

impl Ident {
    pub fn spin(&self) -> IdentSpin {
        IdentSpin { ident: *self }
    }

    pub(crate) fn direction_transformer(&self) -> fn(MoveDirection) -> Step {
        match self {
            Self::First => |md| Step {
                col_inc: md.horizontal,
                row_inc: md.vertical,
            },
            Self::Second => |md| Step {
                col_inc: md.vertical,
                row_inc: -md.horizontal,
            },
            Self::Third => |md| Step {
                col_inc: -md.horizontal,
                row_inc: -md.vertical,
            },
            Self::Fourth => |md| Step {
                col_inc: -md.vertical,
                row_inc: md.horizontal,
            },
        }
    }
}

pub struct IdentSpin {
    ident: Ident,
}

impl Iterator for IdentSpin {
    type Item = Ident;
    fn next(&mut self) -> Option<Self::Item> {
        self.ident = match self.ident {
            Ident::First => Ident::Second,
            Ident::Second => Ident::Third,
            Ident::Third => Ident::Fourth,
            Ident::Fourth => Ident::First,
        };
        Some(self.ident)
    }
}
