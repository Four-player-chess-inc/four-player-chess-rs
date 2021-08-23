mod piece_board;
mod recover;
mod test;

pub(crate) use crate::board::piece_board::{PieceBoard, PieceBoardTrait};
use crate::board::recover::{Recover, RecoverablePieceMove, SquarePos};
use crate::ident::Ident::{self, *};
use crate::piece::Figure::Pawn;
use crate::piece::{Figure, Piece};
use crate::position::{Column, Line, Position, Row};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::default::Default;

pub struct Board {
    pieces: HashMap<Position, Piece>,
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

    pub(crate) fn piece(&self, pos: Position) -> Option<&Piece> {
        self.pieces.get(&pos)
    }

    pub(crate) fn piece_move(&mut self, from: Position, to: Position) -> Option<Piece> {
        if let Some(mut piece) = self.pieces.remove(&from) {
            piece.attrib_mut().have_not_move_yet = false;
            return self.pieces.insert(to, piece);
        }
        None
    }

    pub(crate) fn recoverable_piece_move(
        &mut self,
        from: Position,
        to: Position,
    ) -> RecoverablePieceMove {
        RecoverablePieceMove {
            recover: Recover {
                from: SquarePos {
                    square: match self.pieces.get(&from) {
                        Some(piece) => Some(piece.clone()),
                        None => None,
                    },
                    position: from,
                },
                to: SquarePos {
                    square: match self.pieces.get(&to) {
                        Some(piece) => Some(piece.clone()),
                        None => None,
                    },
                    position: to,
                },
            },
            captured: self.piece_move(from, to),
        }
    }

    pub(crate) fn recover_move(&mut self, recover: Recover) {
        let from = &recover.from;
        match &from.square {
            Some(piece) => {
                self.pieces.insert(from.position, piece.clone());
            }
            None => {
                self.pieces.remove(&from.position);
            }
        }

        let to = &recover.to;
        match &to.square {
            Some(piece) => {
                self.pieces.insert(to.position, piece.clone());
            }
            None => {
                self.pieces.remove(&to.position);
            }
        }
    }
}
