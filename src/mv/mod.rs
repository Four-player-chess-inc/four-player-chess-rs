pub mod move_or_capture;

use crate::board::Board;
use crate::ident::Ident;
use crate::position::Position;

#[derive(Debug)]
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

pub trait Mv {
    fn make_move(&self, board: &mut Board, who_move_next: Ident) -> MakeMoveResult;
}

/*pub enum Move {
    MoveOrCapture { from: Position, to: Position },
    Castling { rook: Position },
    PawnPromotion { from: Position, to: Position },
}*/
