pub mod move_or_capture;
pub mod castling;
pub mod pawn_promotion;

use crate::board::Board;
use crate::ident::Ident;
use crate::mv::castling::Castling;
use crate::mv::move_or_capture::MoveOrCapture;
use crate::mv::pawn_promotion::PawnPromotion;
use crate::piece::{CommonAttrib, Figure};
use crate::position::Position;

#[derive(Debug, PartialEq)]
pub enum MakeMoveError {
    GameOver,
    NothingToMove,
    OpponentsPieceMoveAttempt,
    BadMove,
    MoveUnderCheck,
}

pub enum MakeMoveOk {
    KingCaptured(Ident),
    None,
}

type MakeMoveResult = Result<MakeMoveOk, MakeMoveError>;

pub enum Move {
    MoveOrCapture(MoveOrCapture),
    PawnPromotion(PawnPromotion),
    Castling(Castling),
}

impl Move {
    pub fn move_or_capture(from: Position, to: Position) -> Move {
        Move::MoveOrCapture(MoveOrCapture { from, to })
    }
    pub fn pawn_promotion(from: Position, promote_to: Figure) -> Move {
        Move::PawnPromotion(PawnPromotion { from, promote_to })
    }
    pub fn castling(rook: Position) -> Move {
        Move::Castling(Castling { rook })
    }

    pub(crate) fn make_move(&self, board: &mut Board, who_move_next: Ident) -> MakeMoveResult {
        match self {
            Self::MoveOrCapture(i) => i.make_move(board, who_move_next),
            Self::PawnPromotion(i) => i.make_move(board, who_move_next),
            Self::Castling(i) => i.make_move(board, who_move_next),
        }
    }
}

//pub trait Mv {
//    fn make_move(&self, board: &mut Board, who_move_next: Ident) -> MakeMoveResult;
//}

/*pub enum Move {
    MoveOrCapture { from: Position, to: Position },
    Castling { rook: Position },
    PawnPromotion { from: Position, to: Position },
}*/
