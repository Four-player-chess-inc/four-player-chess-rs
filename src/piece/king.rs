use crate::board::Board;
use crate::move_direction::MoveDirection;
use crate::piece::CommonAttrib;
use crate::position::Position;

#[derive(Clone, Debug)]
pub(crate) struct King {
    pub(crate) attrib: CommonAttrib,
}

impl King {
    pub(crate) fn move_variants(&self, board: &Board, pos: Position) -> Vec<Position> {
        let mut variants = Vec::new();

        let dt = self.attrib.ident.direction_transformer();

        let moves = [
            MoveDirection::Forward(1),
            MoveDirection::Forward(1) + MoveDirection::Right(1),
            MoveDirection::Right(1),
            MoveDirection::Backward(1) + MoveDirection::Right(1),
            MoveDirection::Backward(1),
            MoveDirection::Backward(1) + MoveDirection::Left(1),
            MoveDirection::Left(1),
            MoveDirection::Forward(1) + MoveDirection::Left(1),
        ];

        for md in moves {
            let step_to_pos = match pos.try_step(dt(md)) {
                Ok(step_to_pos) => step_to_pos,
                Err(_) => break,
            };

            if let Some(piece) = board.piece(step_to_pos) {
                if piece.attrib().ident != self.attrib.ident {
                    variants.push(step_to_pos);
                }
                break;
            }

            variants.push(step_to_pos);
        }
        variants
    }
}
