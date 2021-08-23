use crate::board::Board;
use crate::move_direction::MoveDirection;
use crate::piece::CommonAttrib;
use crate::position::Position;

#[derive(Clone, Debug)]
pub(crate) struct Knight {
    pub(crate) attrib: CommonAttrib,
}

impl Knight {
    pub(crate) fn move_variants(&self, board: &Board, pos: Position) -> Vec<Position> {
        let mut variants = Vec::new();

        let dt = self.attrib.ident.direction_transformer();

        let moves = [
            MoveDirection::Forward(2) + MoveDirection::Left(1),
            MoveDirection::Forward(2) + MoveDirection::Right(1),
            MoveDirection::Right(2) + MoveDirection::Forward(1),
            MoveDirection::Right(2) + MoveDirection::Backward(1),
            MoveDirection::Backward(2) + MoveDirection::Right(1),
            MoveDirection::Backward(2) + MoveDirection::Left(1),
            MoveDirection::Left(2) + MoveDirection::Backward(1),
            MoveDirection::Left(2) + MoveDirection::Forward(1),
        ];

        for md in moves {
            if let Ok(step_to_pos) = pos.try_step(dt(md)) {
                if let Some(piece) = board.piece(step_to_pos) {
                    if piece.attrib().ident == self.attrib.ident {
                        continue;
                    }
                }
                variants.push(step_to_pos);
            }
        }
        variants
    }
}
