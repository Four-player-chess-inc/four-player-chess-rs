use crate::move_direction::MoveDirection;
use crate::position::Position;
use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
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

    // TODO: ugly
    pub(crate) fn how_far_from_the_beginning(&self, pos: Position) -> usize {
        match self {
            Self::First => pos.row().get_index() as usize + 1,
            Self::Second => pos.column().get_index() as usize + 1,
            Self::Third => (Position::d14.row().get_index() - pos.row().get_index()) as usize + 1,
            Self::Fourth => {
                (Position::n11.column().get_index() - pos.column().get_index()) as usize + 1
            }
        }
    }
}
