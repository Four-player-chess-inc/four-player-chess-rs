use crate::ident::Ident::{self, *};
use crate::piece::{Figure, Figure::*, Piece};
use crate::position::{Column, Line, Position, Row};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::default::Default;

pub struct Board {
    pieces: HashMap<Position, Piece>,
}

pub struct PiecePos<'a> {
    pub piece: &'a Piece,
    pub position: Position,
    board: &'a mut  Board
}

impl Board {
    pub fn new() -> Board {
        let figure_seq = [
            Figure::Rook,
            Figure::Knight,
            Figure::Bishop,
            Figure::Queen,
            Figure::King,
            Figure::Bishop,
            Figure::Knight,
            Figure::Rook,
        ];

        let mut figure_seq_rev = figure_seq;
        figure_seq_rev.reverse();

        let mut pieces = HashMap::new();

        (3..11_isize).zip(figure_seq).for_each(|i| {
            pieces.insert(
                Position::try_from((i.0, Row::R2.get_index())).unwrap(),
                Piece::new(Pawn, First, Line::Row(Row::R2)),
            );
            pieces.insert(
                Position::try_from((Column::b.get_index(), i.0)).unwrap(),
                Piece::new(Pawn, Second, Line::Column(Column::b)),
            );
            pieces.insert(
                Position::try_from((i.0, Row::R13.get_index())).unwrap(),
                Piece::new(Pawn, Third, Line::Row(Row::R13)),
            );
            pieces.insert(
                Position::try_from((Column::m.get_index(), i.0)).unwrap(),
                Piece::new(Pawn, Fourth, Line::Column(Column::m)),
            );

            pieces.insert(
                Position::try_from((i.0, Row::R1.get_index())).unwrap(),
                Piece::new(i.1, First, Line::Row(Row::R1)),
            );
            pieces.insert(
                Position::try_from((Column::a.get_index(), i.0)).unwrap(),
                Piece::new(i.1, Second, Line::Column(Column::a)),
            );
        });

        (3..11_isize).zip(figure_seq_rev).for_each(|i| {
            pieces.insert(
                Position::try_from((i.0, Row::R14.get_index())).unwrap(),
                Piece::new(i.1, Third, Line::Row(Row::R14)),
            );
            pieces.insert(
                Position::try_from((Column::n.get_index(), i.0)).unwrap(),
                Piece::new(i.1, Fourth, Line::Column(Column::n)),
            );
        });

        return Board { pieces };
    }
    pub fn get_piece(&self, pos: Position) -> Option<&Piece> {
        self.pieces.get(&pos)
    }
}
