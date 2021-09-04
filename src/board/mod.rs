mod piece_board;
mod recover;
mod test;

pub(crate) use crate::board::piece_board::{PieceBoard, PieceBoardTrait};
use crate::board::recover::{Recover, RecoverablePieceMove, SquarePos};
use crate::ident::Ident::{self, *};
use crate::piece::Figure::Pawn;
use crate::piece::{Figure, Piece};
use crate::position::{Column, Position, Row};
use std::collections::HashMap;
use std::convert::TryFrom;

pub struct Board {
    pub(crate) pieces: HashMap<Position, Piece>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum CheckMate {
    No,
    Check,
    Checkmate,
    Stalemate,
}

impl Board {
    // TODO: ugly
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
                Piece::new(Pawn, First),
            );
            pieces.insert(
                Position::try_from((Column::b.get_index(), i.0)).unwrap(),
                Piece::new(Pawn, Second),
            );
            pieces.insert(
                Position::try_from((i.0, Row::R13.get_index())).unwrap(),
                Piece::new(Pawn, Third),
            );
            pieces.insert(
                Position::try_from((Column::m.get_index(), i.0)).unwrap(),
                Piece::new(Pawn, Fourth),
            );

            pieces.insert(
                Position::try_from((i.0, Row::R1.get_index())).unwrap(),
                Piece::new(i.1, First),
            );
            pieces.insert(
                Position::try_from((Column::a.get_index(), i.0)).unwrap(),
                Piece::new(i.1, Second),
            );
        });

        (3..11_isize).zip(figure_seq_rev).for_each(|i| {
            pieces.insert(
                Position::try_from((i.0, Row::R14.get_index())).unwrap(),
                Piece::new(i.1, Third),
            );
            pieces.insert(
                Position::try_from((Column::n.get_index(), i.0)).unwrap(),
                Piece::new(i.1, Fourth),
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

    pub(crate) fn pieces_board(&self) -> Vec<PieceBoard> {
        self.pieces
            .iter()
            .map(|(k, v)| PieceBoard {
                piece: v,
                position: *k,
                board: self,
            })
            .collect::<Vec<_>>()
    }

    pub(crate) fn recoverable_piece_move(
        &mut self,
        from: Position,
        to: Position,
    ) -> RecoverablePieceMove {
        RecoverablePieceMove {
            recover: Recover {
                from: SquarePos {
                    square: self.pieces.get(&from).cloned(),
                    position: from,
                },
                to: SquarePos {
                    square: self.pieces.get(&to).cloned(),
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

    // TODO: ugly
    pub(crate) fn check_checkmate_on_board(&mut self, ident: Ident) -> Result<CheckMate, ()> {
        let (king_was_under_attack, king_pos) = match self.piece_board((Figure::King, ident)) {
            Some(king) => (king.under_attack_any().is_some(), king.position),
            None => return Err(()),
        };

        let our_pieces_pos = self
            .pieces_board()
            .iter()
            .filter(|p| p.piece.attrib().ident == ident)
            .map(|p| p.position)
            .collect::<Vec<_>>();

        for piece_pos in our_pieces_pos {
            for mv in self.piece_board(piece_pos).unwrap().move_variants() {
                let recover = self.recoverable_piece_move(piece_pos, mv);
                let cur_king_pos = if king_pos == piece_pos { mv } else { king_pos };
                if self
                    .piece_board(cur_king_pos)
                    .unwrap()
                    .under_attack_any()
                    .is_none()
                {
                    self.recover_move(recover.recover);
                    return match king_was_under_attack {
                        true => Ok(CheckMate::Check),
                        false => Ok(CheckMate::No),
                    };
                }
                self.recover_move(recover.recover);
            }
        }
        if king_was_under_attack {
            Ok(CheckMate::Checkmate)
        } else {
            Ok(CheckMate::Stalemate)
        }
    }

    pub(crate) fn under_attack_any(&self, target_pos: Position, ours: Ident) -> Option<Position> {
        let opponets_pieces = self
            .pieces
            .iter()
            .filter(|p| p.1.attrib().ident != ours && !p.1.attrib().stone);

        for (pos, piece) in opponets_pieces {
            if piece.move_variants(self, *pos).contains(&target_pos) {
                return Some(*pos);
            }
        }
        None
    }
}
