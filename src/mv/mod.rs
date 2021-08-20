pub mod move_or_capture;

use crate::board::Board;
use crate::game::MakeMoveError;
use crate::position::Position;
use crate::ident::Ident;

pub trait Mv {
    fn make_move(&self, board: &mut Board, who_move_next: Ident) -> Result<(), MakeMoveError>;
}

/*pub enum Move {
    MoveOrCapture { from: Position, to: Position },
    Castling { rook: Position },
    PawnPromotion { from: Position, to: Position },
}*/
