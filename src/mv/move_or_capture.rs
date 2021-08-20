use crate::board::Board;
use crate::game::MakeMoveError;
use crate::mv::Mv;
use crate::position::Position;
use crate::ident::Ident;

pub struct MoveOrCapture {
    pub from: Position,
    pub to: Position,
}

impl Mv for MoveOrCapture {
    fn make_move(&self, board: &mut Board, who_move_next: Ident) -> Result<(), MakeMoveError> {
        let from_piece = board
            .get_piece(self.from)
            .ok_or(MakeMoveError::NothingToMove)?;


        if from_piece.ident != who_move_next {
            return Err(MakeMoveError::OpponentsPieceMoveAttempt)
        }
        //let possible_moves = from_piece.move_variants(&board, self.from);

        // нельзя ходить  под шах


        Ok(())
    }
}
