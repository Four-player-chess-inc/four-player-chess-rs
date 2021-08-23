use crate::board::Board;
use crate::move_direction::MoveDirection;
use crate::piece::CommonAttrib;
use crate::position::Position;

#[derive(Clone, Debug)]
pub(crate) struct Pawn {
    pub(crate) attrib: CommonAttrib,
}

impl Pawn {
    pub(crate) fn move_variants(&self, board: &Board, pos: Position) -> Vec<Position> {
        let mut variants = Vec::new();

        let dt = self.attrib.ident.direction_transformer();

        if let Ok(step_to_pos) = pos.try_step(dt(MoveDirection::Forward(1))) {
            if board.piece(step_to_pos).is_none() {
                variants.push(step_to_pos);
            }
        }

        if self.attrib.have_not_move_yet {
            let step_to_pos = pos.try_step(dt(MoveDirection::Forward(2))).unwrap();
            if board.piece(step_to_pos).is_none() {
                variants.push(step_to_pos);
            }
        }

        let eat_left = MoveDirection::Forward(1) + MoveDirection::Left(1);
        if let Ok(eat_left_pos) = pos.try_step(dt(eat_left)) {
            if let Some(piece) = board.piece(eat_left_pos) {
                if piece.attrib().ident != self.attrib.ident {
                    variants.push(eat_left_pos);
                }
            }
        }

        let eat_right = MoveDirection::Forward(1) + MoveDirection::Right(1);
        if let Ok(eat_right_pos) = pos.try_step(dt(eat_right)) {
            if let Some(piece) = board.piece(eat_right_pos) {
                if piece.attrib().ident != self.attrib.ident {
                    variants.push(eat_right_pos);
                }
            }
        }
        variants
    }
}
