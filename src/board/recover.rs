use crate::piece::Piece;
use crate::position::Position;

pub(crate) struct SquarePos {
    pub(crate) square: Option<Piece>,
    pub(crate) position: Position,
}

pub(crate) struct Recover {
    pub(crate) from: SquarePos,
    pub(crate) to: SquarePos,
}

pub(crate) struct RecoverablePieceMove {
    pub(crate) recover: Recover,
    pub(crate) captured: Option<Piece>,
}

impl From<RecoverablePieceMove> for (Recover, Option<Piece>) {
    fn from(rpm: RecoverablePieceMove) -> Self {
        (rpm.recover, rpm.captured)
    }
}
