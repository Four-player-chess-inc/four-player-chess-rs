use crate::board::Board;
use crate::position::Position;

#[derive(Clone)]
pub struct MoveRule;

fn pawn_move_variants(board: &Board, position: Position) {

}

pub const PAWN_MOVE_RULE: MoveRule = MoveRule;
pub const BISHOP_MOVE_RULE: MoveRule = MoveRule;
pub const KNIGHT_MOVE_RULE: MoveRule = MoveRule;
pub const QUEEN_MOVE_RULE: MoveRule = MoveRule;
pub const KING_MOVE_RULE: MoveRule = MoveRule;
pub const ROOK_MOVE_RULE: MoveRule = MoveRule;
