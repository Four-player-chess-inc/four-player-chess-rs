use crate::board::Board;
use crate::move_direction::MoveDirection;
use crate::piece::CommonAttrib;
use crate::position::Position;

#[derive(Clone, Debug)]
pub(crate) struct Rook {
    pub(crate) attrib: CommonAttrib,
}

impl Rook {
    pub(crate) fn move_variants(&self, board: &Board, pos: Position) -> Vec<Position> {
        let mut variants = Vec::new();

        let dt = self.attrib.ident.direction_transformer();

        let moves = [
            |i| MoveDirection::forward(i),
            |i| MoveDirection::right(i),
            |i| MoveDirection::backward(i),
            |i| MoveDirection::left(i),
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
