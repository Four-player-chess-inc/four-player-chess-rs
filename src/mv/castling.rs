use crate::board::{Board, PieceBoardTrait};
use crate::ident::Ident;
use crate::move_direction::MoveDirection;
use crate::mv::{MakeMoveError, MakeMoveOk, MakeMoveResult, Mv};
use crate::piece::Figure;
use crate::position::{Direction, Position};

pub struct Castling {
    pub rook: Position,
}

impl Mv for Castling {
    fn make_move(&self, board: &mut Board, who_move_next: Ident) -> MakeMoveResult {
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

        let dt = rook.piece.attrib().ident.direction_transformer();

        let mut path_to_king = Vec::new();

        if rook.position.try_step(dt(MoveDirection::Left(1))).is_ok() {
            for step in 1.. {
                let step_to = rook
                    .position
                    .try_step(dt(MoveDirection::Left(step)))
                    .unwrap();
                if step_to == king.position {
                    break;
                }
                path_to_king.push(step_to);
            }
        } else {
            for step in 1.. {
                let step_to = rook
                    .position
                    .try_step(dt(MoveDirection::Right(step)))
                    .unwrap();
                if step_to == king.position {
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

        if king.under_attack_any().is_some() {
            return Err(MakeMoveError::BadMove);
        }

        for pos in path_to_king.iter().rev().take(2) {
            if board.under_attack_any(*pos, who_move_next).is_some() {
                return Err(MakeMoveError::BadMove);
            }
        }

        let mut iter = path_to_king.iter().rev().take(2);
        let rook_dst_post = iter.next().unwrap();
        let king_dst_post = iter.next().unwrap();

        //board.piece_move(rook.position, *rook_dst_post);
        //board.piece_move(king.position, *king_dst_post);

        Ok(MakeMoveOk::None)
    }
}
