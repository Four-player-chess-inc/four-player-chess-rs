use crate::board::{Board, PieceBoardTrait};
use crate::ident::Ident;
use crate::mv::{MakeMoveError, MakeMoveOk, MakeMoveResult};
use crate::piece::Figure;
use crate::position::Position;

pub struct MoveOrCapture {
    pub from: Position,
    pub to: Position,
}

impl MoveOrCapture {
    pub(crate) fn make_move(&self, board: &mut Board, who_move_next: Ident) -> MakeMoveResult {
        let from_piece = board
            .piece_board(self.from)
            .ok_or(MakeMoveError::NothingToMove)?;

        if from_piece.piece.attrib().ident != who_move_next {
            return Err(MakeMoveError::OpponentsPieceMoveAttempt);
        }

        if !from_piece.move_variants().contains(&self.to) {
            return Err(MakeMoveError::BadMove);
        }

        let (recover, captured) = board.recoverable_piece_move(self.from, self.to).into();

        let king = board.piece_board((Figure::King, who_move_next)).unwrap();
        if king.under_attack_any().is_some() {
            board.recover_move(recover);
            return Err(MakeMoveError::MoveUnderCheck);
        }

        if let Some(piece) = captured {
            if piece.figure() == Figure::King {
                return Ok(MakeMoveOk::KingCaptured(piece.attrib().ident));
            }
        }

        Ok(MakeMoveOk::None)
    }
}
