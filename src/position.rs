use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};
use crate::ident::Step;
use std::convert::TryFrom;



#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Row {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
}

impl Row {
    pub fn get_index(&self) -> isize {
        match self {
            Row::R1 => 0,
            Row::R2 => 1,
            Row::R3 => 2,
            Row::R4 => 3,
            Row::R5 => 4,
            Row::R6 => 5,
            Row::R7 => 6,
            Row::R8 => 7,
            Row::R9 => 8,
            Row::R10 => 9,
            Row::R11 => 10,
            Row::R12 => 11,
            Row::R13 => 12,
            Row::R14 => 13,
        }
    }
}

impl TryFrom<isize> for Row {
    type Error = ();
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::R1),
            1 => Ok(Self::R2),
            2 => Ok(Self::R3),
            3 => Ok(Self::R4),
            4 => Ok(Self::R5),
            5 => Ok(Self::R6),
            6 => Ok(Self::R7),
            7 => Ok(Self::R8),
            8 => Ok(Self::R9),
            9 => Ok(Self::R10),
            10 => Ok(Self::R11),
            11 => Ok(Self::R12),
            12 => Ok(Self::R13),
            13 => Ok(Self::R14),
            _ => Err(()),
        }
    }
}

/*impl Index<usize> for Row {
    type Output = Option<Row>;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            1 => &Some(Self::R1),
            2 => &Some(Self::R2),
            3 => &Some(Self::R3),
            4 => &Some(Self::R4),
            5 => &Some(Self::R5),
            6 => &Some(Self::R6),
            7 => &Some(Self::R7),
            8 => &Some(Self::R8),
            9 => &Some(Self::R9),
            10 => &Some(Self::R10),
            11 => &Some(Self::R11),
            12 => &Some(Self::R12),
            13 => &Some(Self::R13),
            14 => &Some(Self::R14),
            _ => &None
        }
    }
}*/

#[derive(PartialEq, Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum Column {
    a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    i,
    j,
    k,
    l,
    m,
    n,
}

impl Column {
    pub fn get_index(&self) -> isize {
        match self {
            Column::a => 0,
            Column::b => 1,
            Column::c => 2,
            Column::d => 3,
            Column::e => 4,
            Column::f => 5,
            Column::g => 6,
            Column::h => 7,
            Column::i => 8,
            Column::j => 9,
            Column::k => 10,
            Column::l => 11,
            Column::m => 12,
            Column::n => 13,
        }
    }
}

impl TryFrom<isize> for Column {
    type Error = ();
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::a),
            1 => Ok(Self::b),
            2 => Ok(Self::c),
            3 => Ok(Self::d),
            4 => Ok(Self::e),
            5 => Ok(Self::f),
            6 => Ok(Self::g),
            7 => Ok(Self::h),
            8 => Ok(Self::i),
            9 => Ok(Self::j),
            10 => Ok(Self::k),
            11 => Ok(Self::l),
            12 => Ok(Self::m),
            13 => Ok(Self::n),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, IntoEnumIterator, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Position {
    a4,
    a5,
    a6,
    a7,
    a8,
    a9,
    a10,
    a11,
    b4,
    b5,
    b6,
    b7,
    b8,
    b9,
    b10,
    b11,
    c4,
    c5,
    c6,
    c7,
    c8,
    c9,
    c10,
    c11,
    d1,
    d2,
    d3,
    d4,
    d5,
    d6,
    d7,
    d8,
    d9,
    d10,
    d11,
    d12,
    d13,
    d14,
    e1,
    e2,
    e3,
    e4,
    e5,
    e6,
    e7,
    e8,
    e9,
    e10,
    e11,
    e12,
    e13,
    e14,
    f1,
    f2,
    f3,
    f4,
    f5,
    f6,
    f7,
    f8,
    f9,
    f10,
    f11,
    f12,
    f13,
    f14,
    g1,
    g2,
    g3,
    g4,
    g5,
    g6,
    g7,
    g8,
    g9,
    g10,
    g11,
    g12,
    g13,
    g14,
    h1,
    h2,
    h3,
    h4,
    h5,
    h6,
    h7,
    h8,
    h9,
    h10,
    h11,
    h12,
    h13,
    h14,
    i1,
    i2,
    i3,
    i4,
    i5,
    i6,
    i7,
    i8,
    i9,
    i10,
    i11,
    i12,
    i13,
    i14,
    j1,
    j2,
    j3,
    j4,
    j5,
    j6,
    j7,
    j8,
    j9,
    j10,
    j11,
    j12,
    j13,
    j14,
    k1,
    k2,
    k3,
    k4,
    k5,
    k6,
    k7,
    k8,
    k9,
    k10,
    k11,
    k12,
    k13,
    k14,
    l4,
    l5,
    l6,
    l7,
    l8,
    l9,
    l10,
    l11,
    m4,
    m5,
    m6,
    m7,
    m8,
    m9,
    m10,
    m11,
    n4,
    n5,
    n6,
    n7,
    n8,
    n9,
    n10,
    n11,
}

impl Position {
    pub fn column(&self) -> Column {
        match self {
            Position::a4 => Column::a,
            Position::a5 => Column::a,
            Position::a6 => Column::a,
            Position::a7 => Column::a,
            Position::a8 => Column::a,
            Position::a9 => Column::a,
            Position::a10 => Column::a,
            Position::a11 => Column::a,
            Position::b4 => Column::b,
            Position::b5 => Column::b,
            Position::b6 => Column::b,
            Position::b7 => Column::b,
            Position::b8 => Column::b,
            Position::b9 => Column::b,
            Position::b10 => Column::b,
            Position::b11 => Column::b,
            Position::c4 => Column::c,
            Position::c5 => Column::c,
            Position::c6 => Column::c,
            Position::c7 => Column::c,
            Position::c8 => Column::c,
            Position::c9 => Column::c,
            Position::c10 => Column::c,
            Position::c11 => Column::c,
            Position::d1 => Column::d,
            Position::d2 => Column::d,
            Position::d3 => Column::d,
            Position::d4 => Column::d,
            Position::d5 => Column::d,
            Position::d6 => Column::d,
            Position::d7 => Column::d,
            Position::d8 => Column::d,
            Position::d9 => Column::d,
            Position::d10 => Column::d,
            Position::d11 => Column::d,
            Position::d12 => Column::d,
            Position::d13 => Column::d,
            Position::d14 => Column::d,
            Position::e1 => Column::e,
            Position::e2 => Column::e,
            Position::e3 => Column::e,
            Position::e4 => Column::e,
            Position::e5 => Column::e,
            Position::e6 => Column::e,
            Position::e7 => Column::e,
            Position::e8 => Column::e,
            Position::e9 => Column::e,
            Position::e10 => Column::e,
            Position::e11 => Column::e,
            Position::e12 => Column::e,
            Position::e13 => Column::e,
            Position::e14 => Column::e,
            Position::f1 => Column::f,
            Position::f2 => Column::f,
            Position::f3 => Column::f,
            Position::f4 => Column::f,
            Position::f5 => Column::f,
            Position::f6 => Column::f,
            Position::f7 => Column::f,
            Position::f8 => Column::f,
            Position::f9 => Column::f,
            Position::f10 => Column::f,
            Position::f11 => Column::f,
            Position::f12 => Column::f,
            Position::f13 => Column::f,
            Position::f14 => Column::f,
            Position::g1 => Column::g,
            Position::g2 => Column::g,
            Position::g3 => Column::g,
            Position::g4 => Column::g,
            Position::g5 => Column::g,
            Position::g6 => Column::g,
            Position::g7 => Column::g,
            Position::g8 => Column::g,
            Position::g9 => Column::g,
            Position::g10 => Column::g,
            Position::g11 => Column::g,
            Position::g12 => Column::g,
            Position::g13 => Column::g,
            Position::g14 => Column::g,
            Position::h1 => Column::h,
            Position::h2 => Column::h,
            Position::h3 => Column::h,
            Position::h4 => Column::h,
            Position::h5 => Column::h,
            Position::h6 => Column::h,
            Position::h7 => Column::h,
            Position::h8 => Column::h,
            Position::h9 => Column::h,
            Position::h10 => Column::h,
            Position::h11 => Column::h,
            Position::h12 => Column::h,
            Position::h13 => Column::h,
            Position::h14 => Column::h,
            Position::i1 => Column::i,
            Position::i2 => Column::i,
            Position::i3 => Column::i,
            Position::i4 => Column::i,
            Position::i5 => Column::i,
            Position::i6 => Column::i,
            Position::i7 => Column::i,
            Position::i8 => Column::i,
            Position::i9 => Column::i,
            Position::i10 => Column::i,
            Position::i11 => Column::i,
            Position::i12 => Column::i,
            Position::i13 => Column::i,
            Position::i14 => Column::i,
            Position::j1 => Column::j,
            Position::j2 => Column::j,
            Position::j3 => Column::j,
            Position::j4 => Column::j,
            Position::j5 => Column::j,
            Position::j6 => Column::j,
            Position::j7 => Column::j,
            Position::j8 => Column::j,
            Position::j9 => Column::j,
            Position::j10 => Column::j,
            Position::j11 => Column::j,
            Position::j12 => Column::j,
            Position::j13 => Column::j,
            Position::j14 => Column::j,
            Position::k1 => Column::k,
            Position::k2 => Column::k,
            Position::k3 => Column::k,
            Position::k4 => Column::k,
            Position::k5 => Column::k,
            Position::k6 => Column::k,
            Position::k7 => Column::k,
            Position::k8 => Column::k,
            Position::k9 => Column::k,
            Position::k10 => Column::k,
            Position::k11 => Column::k,
            Position::k12 => Column::k,
            Position::k13 => Column::k,
            Position::k14 => Column::k,
            Position::l4 => Column::l,
            Position::l5 => Column::l,
            Position::l6 => Column::l,
            Position::l7 => Column::l,
            Position::l8 => Column::l,
            Position::l9 => Column::l,
            Position::l10 => Column::l,
            Position::l11 => Column::l,
            Position::m4 => Column::m,
            Position::m5 => Column::m,
            Position::m6 => Column::m,
            Position::m7 => Column::m,
            Position::m8 => Column::m,
            Position::m9 => Column::m,
            Position::m10 => Column::m,
            Position::m11 => Column::m,
            Position::n4 => Column::n,
            Position::n5 => Column::n,
            Position::n6 => Column::n,
            Position::n7 => Column::n,
            Position::n8 => Column::n,
            Position::n9 => Column::n,
            Position::n10 => Column::n,
            Position::n11 => Column::n,
        }
    }
    pub fn row(&self) -> Row {
        match self {
            Position::a4 => Row::R4,
            Position::a5 => Row::R5,
            Position::a6 => Row::R6,
            Position::a7 => Row::R7,
            Position::a8 => Row::R8,
            Position::a9 => Row::R9,
            Position::a10 => Row::R10,
            Position::a11 => Row::R11,
            Position::b4 => Row::R4,
            Position::b5 => Row::R5,
            Position::b6 => Row::R6,
            Position::b7 => Row::R7,
            Position::b8 => Row::R8,
            Position::b9 => Row::R9,
            Position::b10 => Row::R10,
            Position::b11 => Row::R11,
            Position::c4 => Row::R4,
            Position::c5 => Row::R5,
            Position::c6 => Row::R6,
            Position::c7 => Row::R7,
            Position::c8 => Row::R8,
            Position::c9 => Row::R9,
            Position::c10 => Row::R10,
            Position::c11 => Row::R11,
            Position::d1 => Row::R1,
            Position::d2 => Row::R2,
            Position::d3 => Row::R3,
            Position::d4 => Row::R4,
            Position::d5 => Row::R5,
            Position::d6 => Row::R6,
            Position::d7 => Row::R7,
            Position::d8 => Row::R8,
            Position::d9 => Row::R9,
            Position::d10 => Row::R10,
            Position::d11 => Row::R11,
            Position::d12 => Row::R12,
            Position::d13 => Row::R13,
            Position::d14 => Row::R14,
            Position::e1 => Row::R1,
            Position::e2 => Row::R2,
            Position::e3 => Row::R3,
            Position::e4 => Row::R4,
            Position::e5 => Row::R5,
            Position::e6 => Row::R6,
            Position::e7 => Row::R7,
            Position::e8 => Row::R8,
            Position::e9 => Row::R9,
            Position::e10 => Row::R10,
            Position::e11 => Row::R11,
            Position::e12 => Row::R12,
            Position::e13 => Row::R13,
            Position::e14 => Row::R14,
            Position::f1 => Row::R1,
            Position::f2 => Row::R2,
            Position::f3 => Row::R3,
            Position::f4 => Row::R4,
            Position::f5 => Row::R5,
            Position::f6 => Row::R6,
            Position::f7 => Row::R7,
            Position::f8 => Row::R8,
            Position::f9 => Row::R9,
            Position::f10 => Row::R10,
            Position::f11 => Row::R11,
            Position::f12 => Row::R12,
            Position::f13 => Row::R13,
            Position::f14 => Row::R14,
            Position::g1 => Row::R1,
            Position::g2 => Row::R2,
            Position::g3 => Row::R3,
            Position::g4 => Row::R4,
            Position::g5 => Row::R5,
            Position::g6 => Row::R6,
            Position::g7 => Row::R7,
            Position::g8 => Row::R8,
            Position::g9 => Row::R9,
            Position::g10 => Row::R10,
            Position::g11 => Row::R11,
            Position::g12 => Row::R12,
            Position::g13 => Row::R13,
            Position::g14 => Row::R14,
            Position::h1 => Row::R1,
            Position::h2 => Row::R2,
            Position::h3 => Row::R3,
            Position::h4 => Row::R4,
            Position::h5 => Row::R5,
            Position::h6 => Row::R6,
            Position::h7 => Row::R7,
            Position::h8 => Row::R8,
            Position::h9 => Row::R9,
            Position::h10 => Row::R10,
            Position::h11 => Row::R11,
            Position::h12 => Row::R12,
            Position::h13 => Row::R13,
            Position::h14 => Row::R14,
            Position::i1 => Row::R1,
            Position::i2 => Row::R2,
            Position::i3 => Row::R3,
            Position::i4 => Row::R4,
            Position::i5 => Row::R5,
            Position::i6 => Row::R6,
            Position::i7 => Row::R7,
            Position::i8 => Row::R8,
            Position::i9 => Row::R9,
            Position::i10 => Row::R10,
            Position::i11 => Row::R11,
            Position::i12 => Row::R12,
            Position::i13 => Row::R13,
            Position::i14 => Row::R14,
            Position::j1 => Row::R1,
            Position::j2 => Row::R2,
            Position::j3 => Row::R3,
            Position::j4 => Row::R4,
            Position::j5 => Row::R5,
            Position::j6 => Row::R6,
            Position::j7 => Row::R7,
            Position::j8 => Row::R8,
            Position::j9 => Row::R9,
            Position::j10 => Row::R10,
            Position::j11 => Row::R11,
            Position::j12 => Row::R12,
            Position::j13 => Row::R13,
            Position::j14 => Row::R14,
            Position::k1 => Row::R1,
            Position::k2 => Row::R2,
            Position::k3 => Row::R3,
            Position::k4 => Row::R4,
            Position::k5 => Row::R5,
            Position::k6 => Row::R6,
            Position::k7 => Row::R7,
            Position::k8 => Row::R8,
            Position::k9 => Row::R9,
            Position::k10 => Row::R10,
            Position::k11 => Row::R11,
            Position::k12 => Row::R12,
            Position::k13 => Row::R13,
            Position::k14 => Row::R14,
            Position::l4 => Row::R4,
            Position::l5 => Row::R5,
            Position::l6 => Row::R6,
            Position::l7 => Row::R7,
            Position::l8 => Row::R8,
            Position::l9 => Row::R9,
            Position::l10 => Row::R10,
            Position::l11 => Row::R11,
            Position::m4 => Row::R4,
            Position::m5 => Row::R5,
            Position::m6 => Row::R6,
            Position::m7 => Row::R7,
            Position::m8 => Row::R8,
            Position::m9 => Row::R9,
            Position::m10 => Row::R10,
            Position::m11 => Row::R11,
            Position::n4 => Row::R4,
            Position::n5 => Row::R5,
            Position::n6 => Row::R6,
            Position::n7 => Row::R7,
            Position::n8 => Row::R8,
            Position::n9 => Row::R9,
            Position::n10 => Row::R10,
            Position::n11 => Row::R11,
        }
    }
    pub fn col_row(&self) -> (Column, Row) {
        (self.column(), self.row())
    }
    pub fn col_row_idx(&self) -> (isize, isize) {
        (self.column().get_index(), self.row().get_index())
    }
    pub fn line_between(pos_one: Position, pos_two: Position) -> Result<Vec<Position>, ()> {
        let (pos_one_col, pos_one_row) = pos_one.col_row_idx();
        let (pos_two_col, pos_two_row) = pos_two.col_row_idx();

        if pos_one_col == pos_two_col {
            return if pos_one_row < pos_two_row {
                Ok((pos_one_row..pos_two_row)
                    .skip(1)
                    .map(|r| Position::try_from((pos_one_col, r)).unwrap())
                    .collect::<Vec<_>>())
            } else {
                Ok((pos_two_row..pos_one_row)
                    .skip(1)
                    .map(|r| Position::try_from((pos_one_col, r)).unwrap())
                    .collect::<Vec<_>>())
            };
        } else if pos_one_row == pos_two_row {
            return if pos_one_col < pos_two_col {
                Ok((pos_one_col..pos_two_col)
                    .skip(1)
                    .map(|c| Position::try_from((c, pos_one_row)).unwrap())
                    .collect::<Vec<_>>())
            } else {
                Ok((pos_two_col..pos_one_col)
                    .skip(1)
                    .map(|c| Position::try_from((c, pos_one_row)).unwrap())
                    .collect::<Vec<_>>())
            };
        }
        Err(())
    }

    pub(crate) fn try_step(&self, step: Step) -> Result<Position, ()> {
        let (col_idx, row_idx) = self.col_row_idx();

        Position::try_from((col_idx + step.col_inc, row_idx + step.row_inc))
    }

    pub fn step(&self, direction: &Direction, distance: usize) -> Result<Position, ()> {
        let (mut col_idx, mut row_idx) = self.col_row_idx();
        col_idx += match direction.column {
            DecNoneInc::Inc => 1 * distance as isize,
            DecNoneInc::None => 0,
            DecNoneInc::Dec => -1 * distance as isize,
        };
        row_idx += match direction.row {
            DecNoneInc::Inc => 1 * distance as isize,
            DecNoneInc::None => 0,
            DecNoneInc::Dec => -1 * distance as isize,
        };
        Position::try_from((col_idx, row_idx))
    }
}

impl TryFrom<(isize, isize)> for Position {
    type Error = ();
    fn try_from(value: (isize, isize)) -> Result<Self, Self::Error> {
        if let Ok(col) = Column::try_from(value.0) {
            if let Ok(row) = Row::try_from(value.1) {
                return Self::try_from((col, row));
            }
        }
        Err(())
    }
}

impl TryFrom<(Column, Row)> for Position {
    type Error = ();
    fn try_from(value: (Column, Row)) -> Result<Self, Self::Error> {
        match value {
            (Column::a, Row::R4) => Ok(Position::a4),
            (Column::a, Row::R5) => Ok(Position::a5),
            (Column::a, Row::R6) => Ok(Position::a6),
            (Column::a, Row::R7) => Ok(Position::a7),
            (Column::a, Row::R8) => Ok(Position::a8),
            (Column::a, Row::R9) => Ok(Position::a9),
            (Column::a, Row::R10) => Ok(Position::a10),
            (Column::a, Row::R11) => Ok(Position::a11),
            (Column::b, Row::R4) => Ok(Position::b4),
            (Column::b, Row::R5) => Ok(Position::b5),
            (Column::b, Row::R6) => Ok(Position::b6),
            (Column::b, Row::R7) => Ok(Position::b7),
            (Column::b, Row::R8) => Ok(Position::b8),
            (Column::b, Row::R9) => Ok(Position::b9),
            (Column::b, Row::R10) => Ok(Position::b10),
            (Column::b, Row::R11) => Ok(Position::b11),
            (Column::c, Row::R4) => Ok(Position::c4),
            (Column::c, Row::R5) => Ok(Position::c5),
            (Column::c, Row::R6) => Ok(Position::c6),
            (Column::c, Row::R7) => Ok(Position::c7),
            (Column::c, Row::R8) => Ok(Position::c8),
            (Column::c, Row::R9) => Ok(Position::c9),
            (Column::c, Row::R10) => Ok(Position::c10),
            (Column::c, Row::R11) => Ok(Position::c11),
            (Column::d, Row::R1) => Ok(Position::d1),
            (Column::d, Row::R2) => Ok(Position::d2),
            (Column::d, Row::R3) => Ok(Position::d3),
            (Column::d, Row::R4) => Ok(Position::d4),
            (Column::d, Row::R5) => Ok(Position::d5),
            (Column::d, Row::R6) => Ok(Position::d6),
            (Column::d, Row::R7) => Ok(Position::d7),
            (Column::d, Row::R8) => Ok(Position::d8),
            (Column::d, Row::R9) => Ok(Position::d9),
            (Column::d, Row::R10) => Ok(Position::d10),
            (Column::d, Row::R11) => Ok(Position::d11),
            (Column::d, Row::R12) => Ok(Position::d12),
            (Column::d, Row::R13) => Ok(Position::d13),
            (Column::d, Row::R14) => Ok(Position::d14),
            (Column::e, Row::R1) => Ok(Position::e1),
            (Column::e, Row::R2) => Ok(Position::e2),
            (Column::e, Row::R3) => Ok(Position::e3),
            (Column::e, Row::R4) => Ok(Position::e4),
            (Column::e, Row::R5) => Ok(Position::e5),
            (Column::e, Row::R6) => Ok(Position::e6),
            (Column::e, Row::R7) => Ok(Position::e7),
            (Column::e, Row::R8) => Ok(Position::e8),
            (Column::e, Row::R9) => Ok(Position::e9),
            (Column::e, Row::R10) => Ok(Position::e10),
            (Column::e, Row::R11) => Ok(Position::e11),
            (Column::e, Row::R12) => Ok(Position::e12),
            (Column::e, Row::R13) => Ok(Position::e13),
            (Column::e, Row::R14) => Ok(Position::e14),
            (Column::f, Row::R1) => Ok(Position::f1),
            (Column::f, Row::R2) => Ok(Position::f2),
            (Column::f, Row::R3) => Ok(Position::f3),
            (Column::f, Row::R4) => Ok(Position::f4),
            (Column::f, Row::R5) => Ok(Position::f5),
            (Column::f, Row::R6) => Ok(Position::f6),
            (Column::f, Row::R7) => Ok(Position::f7),
            (Column::f, Row::R8) => Ok(Position::f8),
            (Column::f, Row::R9) => Ok(Position::f9),
            (Column::f, Row::R10) => Ok(Position::f10),
            (Column::f, Row::R11) => Ok(Position::f11),
            (Column::f, Row::R12) => Ok(Position::f12),
            (Column::f, Row::R13) => Ok(Position::f13),
            (Column::f, Row::R14) => Ok(Position::f14),
            (Column::g, Row::R1) => Ok(Position::g1),
            (Column::g, Row::R2) => Ok(Position::g2),
            (Column::g, Row::R3) => Ok(Position::g3),
            (Column::g, Row::R4) => Ok(Position::g4),
            (Column::g, Row::R5) => Ok(Position::g5),
            (Column::g, Row::R6) => Ok(Position::g6),
            (Column::g, Row::R7) => Ok(Position::g7),
            (Column::g, Row::R8) => Ok(Position::g8),
            (Column::g, Row::R9) => Ok(Position::g9),
            (Column::g, Row::R10) => Ok(Position::g10),
            (Column::g, Row::R11) => Ok(Position::g11),
            (Column::g, Row::R12) => Ok(Position::g12),
            (Column::g, Row::R13) => Ok(Position::g13),
            (Column::g, Row::R14) => Ok(Position::g14),
            (Column::h, Row::R1) => Ok(Position::h1),
            (Column::h, Row::R2) => Ok(Position::h2),
            (Column::h, Row::R3) => Ok(Position::h3),
            (Column::h, Row::R4) => Ok(Position::h4),
            (Column::h, Row::R5) => Ok(Position::h5),
            (Column::h, Row::R6) => Ok(Position::h6),
            (Column::h, Row::R7) => Ok(Position::h7),
            (Column::h, Row::R8) => Ok(Position::h8),
            (Column::h, Row::R9) => Ok(Position::h9),
            (Column::h, Row::R10) => Ok(Position::h10),
            (Column::h, Row::R11) => Ok(Position::h11),
            (Column::h, Row::R12) => Ok(Position::h12),
            (Column::h, Row::R13) => Ok(Position::h13),
            (Column::h, Row::R14) => Ok(Position::h14),
            (Column::i, Row::R1) => Ok(Position::i1),
            (Column::i, Row::R2) => Ok(Position::i2),
            (Column::i, Row::R3) => Ok(Position::i3),
            (Column::i, Row::R4) => Ok(Position::i4),
            (Column::i, Row::R5) => Ok(Position::i5),
            (Column::i, Row::R6) => Ok(Position::i6),
            (Column::i, Row::R7) => Ok(Position::i7),
            (Column::i, Row::R8) => Ok(Position::i8),
            (Column::i, Row::R9) => Ok(Position::i9),
            (Column::i, Row::R10) => Ok(Position::i10),
            (Column::i, Row::R11) => Ok(Position::i11),
            (Column::i, Row::R12) => Ok(Position::i12),
            (Column::i, Row::R13) => Ok(Position::i13),
            (Column::i, Row::R14) => Ok(Position::i14),
            (Column::j, Row::R1) => Ok(Position::j1),
            (Column::j, Row::R2) => Ok(Position::j2),
            (Column::j, Row::R3) => Ok(Position::j3),
            (Column::j, Row::R4) => Ok(Position::j4),
            (Column::j, Row::R5) => Ok(Position::j5),
            (Column::j, Row::R6) => Ok(Position::j6),
            (Column::j, Row::R7) => Ok(Position::j7),
            (Column::j, Row::R8) => Ok(Position::j8),
            (Column::j, Row::R9) => Ok(Position::j9),
            (Column::j, Row::R10) => Ok(Position::j10),
            (Column::j, Row::R11) => Ok(Position::j11),
            (Column::j, Row::R12) => Ok(Position::j12),
            (Column::j, Row::R13) => Ok(Position::j13),
            (Column::j, Row::R14) => Ok(Position::j14),
            (Column::k, Row::R1) => Ok(Position::k1),
            (Column::k, Row::R2) => Ok(Position::k2),
            (Column::k, Row::R3) => Ok(Position::k3),
            (Column::k, Row::R4) => Ok(Position::k4),
            (Column::k, Row::R5) => Ok(Position::k5),
            (Column::k, Row::R6) => Ok(Position::k6),
            (Column::k, Row::R7) => Ok(Position::k7),
            (Column::k, Row::R8) => Ok(Position::k8),
            (Column::k, Row::R9) => Ok(Position::k9),
            (Column::k, Row::R10) => Ok(Position::k10),
            (Column::k, Row::R11) => Ok(Position::k11),
            (Column::k, Row::R12) => Ok(Position::k12),
            (Column::k, Row::R13) => Ok(Position::k13),
            (Column::k, Row::R14) => Ok(Position::k14),
            (Column::l, Row::R4) => Ok(Position::l4),
            (Column::l, Row::R5) => Ok(Position::l5),
            (Column::l, Row::R6) => Ok(Position::l6),
            (Column::l, Row::R7) => Ok(Position::l7),
            (Column::l, Row::R8) => Ok(Position::l8),
            (Column::l, Row::R9) => Ok(Position::l9),
            (Column::l, Row::R10) => Ok(Position::l10),
            (Column::l, Row::R11) => Ok(Position::l11),
            (Column::m, Row::R4) => Ok(Position::m4),
            (Column::m, Row::R5) => Ok(Position::m5),
            (Column::m, Row::R6) => Ok(Position::m6),
            (Column::m, Row::R7) => Ok(Position::m7),
            (Column::m, Row::R8) => Ok(Position::m8),
            (Column::m, Row::R9) => Ok(Position::m9),
            (Column::m, Row::R10) => Ok(Position::m10),
            (Column::m, Row::R11) => Ok(Position::m11),
            (Column::n, Row::R4) => Ok(Position::n4),
            (Column::n, Row::R5) => Ok(Position::n5),
            (Column::n, Row::R6) => Ok(Position::n6),
            (Column::n, Row::R7) => Ok(Position::n7),
            (Column::n, Row::R8) => Ok(Position::n8),
            (Column::n, Row::R9) => Ok(Position::n9),
            (Column::n, Row::R10) => Ok(Position::n10),
            (Column::n, Row::R11) => Ok(Position::n11),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Line {
    Column(Column),
    Row(Row),
}

pub enum DecNoneInc {
    Dec,
    None,
    Inc,
}

pub struct Direction {
    pub column: DecNoneInc,
    pub row: DecNoneInc,
}

impl Direction {
    pub fn new(column: DecNoneInc, row: DecNoneInc) -> Direction {
        Direction { column, row }
    }
}

pub struct DirectionStar {
    pub forward: Direction,
    pub forward_right: Direction,
    pub right: Direction,
    pub backward_right: Direction,
    pub backward: Direction,
    pub backward_left: Direction,
    pub left: Direction,
    pub forward_left: Direction,
}

impl Direction {
    // return all directions
    pub fn try_all_from_home_line(home_line: Line) -> Result<DirectionStar, ()> {
        match home_line {
            Line::Row(Row::R2) | Line::Row(Row::R1) => Ok(DirectionStar {
                forward: Direction::new(DecNoneInc::None, DecNoneInc::Inc),
                forward_right: Direction::new(DecNoneInc::Inc, DecNoneInc::Inc),
                right: Direction::new(DecNoneInc::Inc, DecNoneInc::None),
                backward_right: Direction::new(DecNoneInc::Inc, DecNoneInc::Dec),
                backward: Direction::new(DecNoneInc::None, DecNoneInc::Dec),
                backward_left: Direction::new(DecNoneInc::Dec, DecNoneInc::Dec),
                left: Direction::new(DecNoneInc::Dec, DecNoneInc::None),
                forward_left: Direction::new(DecNoneInc::Dec, DecNoneInc::Inc),
            }),
            Line::Row(Row::R13) | Line::Row(Row::R14) => Ok(DirectionStar {
                forward: Direction::new(DecNoneInc::None, DecNoneInc::Dec),
                forward_right: Direction::new(DecNoneInc::Dec, DecNoneInc::Dec),
                right: Direction::new(DecNoneInc::Dec, DecNoneInc::None),
                backward_right: Direction::new(DecNoneInc::Dec, DecNoneInc::Inc),
                backward: Direction::new(DecNoneInc::None, DecNoneInc::Inc),
                backward_left: Direction::new(DecNoneInc::Inc, DecNoneInc::Inc),
                left: Direction::new(DecNoneInc::Inc, DecNoneInc::None),
                forward_left: Direction::new(DecNoneInc::Inc, DecNoneInc::Dec),
            }),
            Line::Column(Column::b) | Line::Column(Column::a) => Ok(DirectionStar {
                forward: Direction::new(DecNoneInc::Inc, DecNoneInc::None),
                forward_right: Direction::new(DecNoneInc::Inc, DecNoneInc::Dec),
                right: Direction::new(DecNoneInc::None, DecNoneInc::Dec),
                backward_right: Direction::new(DecNoneInc::Dec, DecNoneInc::Dec),
                backward: Direction::new(DecNoneInc::Dec, DecNoneInc::None),
                backward_left: Direction::new(DecNoneInc::Dec, DecNoneInc::Inc),
                left: Direction::new(DecNoneInc::None, DecNoneInc::Inc),
                forward_left: Direction::new(DecNoneInc::Inc, DecNoneInc::Inc),
            }),
            Line::Column(Column::m) | Line::Column(Column::n) => Ok(DirectionStar {
                forward: Direction::new(DecNoneInc::Dec, DecNoneInc::None),
                forward_right: Direction::new(DecNoneInc::Dec, DecNoneInc::Inc),
                right: Direction::new(DecNoneInc::None, DecNoneInc::Inc),
                backward_right: Direction::new(DecNoneInc::Inc, DecNoneInc::Inc),
                backward: Direction::new(DecNoneInc::Inc, DecNoneInc::None),
                backward_left: Direction::new(DecNoneInc::Inc, DecNoneInc::Dec),
                left: Direction::new(DecNoneInc::None, DecNoneInc::Dec),
                forward_left: Direction::new(DecNoneInc::Dec, DecNoneInc::Dec),
            }),
            _ => Err(()),
        }
    }
}

#[test]
fn position_try_step() {
    use crate::board::{Board, PieceBoardTrait};
    use crate::move_direction::MoveDirection;

    let board = Board::new();

    let pawn = board.piece_board(Position::e2).unwrap();
    let dt = pawn.piece.attrib().ident.direction_transformer();

    dbg!(dt(MoveDirection::forward(1)));
    assert_eq!(
        pawn.position
            .try_step(dt(MoveDirection::forward(1)))
            .unwrap(),
        Position::e3
    );
    assert_eq!(
        pawn.position
            .try_step(dt(MoveDirection::forward(5)))
            .unwrap(),
        Position::e7
    );
    assert_eq!(
        pawn.position.try_step(dt(MoveDirection::forward(15))),
        Err(())
    );
    assert_eq!(
        pawn.position
            .try_step(dt(MoveDirection::backward(1)))
            .unwrap(),
        Position::e1
    );
    assert_eq!(
        pawn.position.try_step(dt(MoveDirection::backward(2))),
        Err(())
    );

    let piece = board.piece_board(Position::b7).unwrap();
    let dt = piece.piece.attrib().ident.direction_transformer();
    assert_eq!(
        piece
            .position
            .try_step(dt(MoveDirection::forward(1)))
            .unwrap(),
        Position::c7
    );

    let piece = board.piece_board(Position::b7).unwrap();
    let dt = piece.piece.attrib().ident.direction_transformer();
    assert_eq!(
        piece
            .position
            .try_step(dt(MoveDirection::forward(5) + MoveDirection::left(2)))
            .unwrap(),
        Position::g9
    );

    let piece = board.piece_board(Position::h13).unwrap();
    let dt = piece.piece.attrib().ident.direction_transformer();
    assert_eq!(
        piece
            .position
            .try_step(dt(MoveDirection::forward(3) + MoveDirection::right(4)))
            .unwrap(),
        Position::d10
    );

    let piece = board.piece_board(Position::m9).unwrap();
    let dt = piece.piece.attrib().ident.direction_transformer();
    assert_eq!(
        piece
            .position
            .try_step(dt(MoveDirection::left(2) + MoveDirection::backward(1)))
            .unwrap(),
        Position::n7
    );
}
