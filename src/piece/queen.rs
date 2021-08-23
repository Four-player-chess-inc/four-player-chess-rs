use crate::board::Board;
use crate::move_direction::MoveDirection;
use crate::piece::CommonAttrib;
use crate::position::Position;

#[derive(Clone, Debug)]
pub(crate) struct Queen {
    pub(crate) attrib: CommonAttrib,
}

impl Queen {
    pub(crate) fn move_variants(&self, board: &Board, pos: Position) -> Vec<Position> {
        let mut variants = Vec::new();

        let dt = self.attrib.ident.direction_transformer();

        let moves = [
            |i| MoveDirection::Forward(i),
            |i| MoveDirection::Forward(i) + MoveDirection::Right(i),
            |i| MoveDirection::Right(i),
            |i| MoveDirection::Backward(i) + MoveDirection::Right(i),
            |i| MoveDirection::Backward(i),
            |i| MoveDirection::Backward(i) + MoveDirection::Left(i),
            |i| MoveDirection::Left(i),
            |i| MoveDirection::Forward(i) + MoveDirection::Left(i),
        ];

        for md_func in moves {
            for i in 1.. {
                let step_to_pos = match pos.try_step(dt(md_func(i))) {
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
        }
        variants
    }
}
