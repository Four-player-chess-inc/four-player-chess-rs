use crate::board::{Board, PieceBoardTrait};
use crate::ident::Ident;
use crate::move_direction::MoveDirection;
use crate::mv::{MakeMoveError, MakeMoveOk, MakeMoveResult};
use crate::piece::Figure;
use crate::position::Position;

#[derive(Debug, Clone)]
pub struct Castling {
    pub rook: Position,
}

impl Castling {
    // TODO: ugly
    pub(crate) fn make_move(&self, board: &mut Board, who_move_next: Ident) -> MakeMoveResult {
        {
            let rook = board
                .piece_board(self.rook)
                .ok_or(MakeMoveError::NothingToMove)?;

            if rook.piece.attrib().ident != who_move_next {
                return Err(MakeMoveError::OpponentsPieceMoveAttempt);
            }

            if rook.piece.figure() != Figure::Rook {
                return Err(MakeMoveError::BadMove);
            }

            if !rook.piece.attrib().have_not_move_yet {
                return Err(MakeMoveError::BadMove);
            }

            let king = board.piece_board((Figure::King, who_move_next)).unwrap();

            if !king.piece.attrib().have_not_move_yet {
                return Err(MakeMoveError::BadMove);
            }

            if king.under_attack_any().is_some() {
                return Err(MakeMoveError::BadMove);
            }
        }

        let (rook_ident, rook_pos) = match board.piece_board(self.rook) {
            Some(p) => (p.piece.attrib().ident, self.rook),
            None => return Err(MakeMoveError::NothingToMove),
        };

        let king_pos = board
            .piece_board((Figure::King, who_move_next))
            .unwrap()
            .position;

        let dt = rook_ident.direction_transformer();

        let mut path_to_king = Vec::new();

        if rook_pos.try_step(dt(MoveDirection::left(1))).is_ok() {
            for step in 1.. {
                let step_to = rook_pos.try_step(dt(MoveDirection::left(step))).unwrap();
                if step_to == king_pos {
                    break;
                }
                path_to_king.push(step_to);
            }
        } else {
            for step in 1.. {
                let step_to = rook_pos.try_step(dt(MoveDirection::right(step))).unwrap();
                if step_to == king_pos {
                    break;
                }
                path_to_king.push(step_to);
            }
        }

        for pos in &path_to_king {
            if board.piece(*pos).is_some() {
                return Err(MakeMoveError::BadMove);
            }
        }

        for pos in path_to_king.iter().rev().take(2) {
            if board.under_attack_any(*pos, who_move_next).is_some() {
                return Err(MakeMoveError::BadMove);
            }
        }

        let mut iter = path_to_king.iter().rev().take(2);
        let rook_dst_post = iter.next().unwrap();
        let king_dst_post = iter.next().unwrap();

        board.piece_move(rook_pos, *rook_dst_post);
        board.piece_move(king_pos, *king_dst_post);

        Ok(MakeMoveOk::None)
    }
}
