use crate::board::{Board, PieceBoardTrait};
use crate::ident::Ident;
use crate::move_direction::MoveDirection;
use crate::mv::{MakeMoveError, MakeMoveOk, MakeMoveResult};
use crate::piece::{Figure, Piece};
use crate::position::Position;

pub struct PawnPromotion {
    pub from: Position,
    pub promote_to: Figure,
}

impl PawnPromotion {
    pub(crate) fn make_move(&self, board: &mut Board, who_move_next: Ident) -> MakeMoveResult {
        let from_piece = board
            .piece_board(self.from)
            .ok_or(MakeMoveError::NothingToMove)?;

        if from_piece.piece.attrib().ident != who_move_next {
            return Err(MakeMoveError::OpponentsPieceMoveAttempt);
        }

        if from_piece.piece.figure() != Figure::Pawn {
            return Err(MakeMoveError::BadMove);
        }

        let dt = from_piece.piece.attrib().ident.direction_transformer();
        let dst_pos = match from_piece.position.try_step(dt(MoveDirection::forward(1))) {
            Ok(pos) => pos,
            Err(_) => return Err(MakeMoveError::BadMove),
        };

        if !from_piece.move_variants().contains(&dst_pos) {
            return Err(MakeMoveError::BadMove);
        }

        if from_piece
            .piece
            .attrib()
            .ident
            .how_far_from_the_beginning(dst_pos)
            != 8
        {
            return Err(MakeMoveError::BadMove);
        }

        let (recover, _) = board.recoverable_piece_move(self.from, dst_pos).into();

        let king = board.piece_board((Figure::King, who_move_next)).unwrap();
        if king.under_attack_any().is_some() {
            board.recover_move(recover);
            return Err(MakeMoveError::MoveUnderCheck);
        }

        let mut promoted = Piece::new(self.promote_to, who_move_next);
        promoted.attrib_mut().have_not_move_yet = false;

        board.pieces.insert(dst_pos, promoted);

        Ok(MakeMoveOk::None)
    }
}
