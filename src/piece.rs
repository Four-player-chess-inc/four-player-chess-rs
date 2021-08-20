use self::Figure::*;
use crate::ident::Ident;
use crate::move_variants::*;
use crate::position::{Column, Line, Row};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Figure {
    Pawn,
    Bishop,
    Knight,
    Queen,
    King,
    Rook,
}

/*pub trait Piece {
    fn test(&self) -> String;
}


struct PieceCommonAttrib {
    figure: Figure,
    ident: Ident,
    have_not_move_yet: bool,
    home_line: Line
}

pub struct Pawn {
    attributes: PieceCommonAttrib
}

impl Pawn {
    pub fn new() -> Pawn {
        Pawn {
            figure: Figure::Pawn
        }
    }
}

pub struct Rook {
    figure: Figure
}

impl Rook {
    pub fn new() -> Rook {
        Rook {
            figure: Figure::Rook
        }
    }
}


impl Piece for Pawn {
    fn test(&self) -> String {
        return "pawn".to_string();
    }
}

impl Piece for Rook {
    fn test(&self) -> String {
        return "rook".to_string();
    }
}*/

#[derive(Clone)]
pub struct Piece {
    pub figure: Figure,
    pub ident: Ident,
    pub have_not_move_yet: bool,
    pub home_line: Line,
    pub move_rule: MoveRule,
}

impl Piece {
    pub fn new(figure: Figure, ident: Ident, home_line: Line) -> Piece {
        let move_rule = match figure {
            Pawn => PAWN_MOVE_RULE,
            Bishop => BISHOP_MOVE_RULE,
            Knight => KNIGHT_MOVE_RULE,
            Queen => QUEEN_MOVE_RULE,
            King => KING_MOVE_RULE,
            Rook => ROOK_MOVE_RULE,
        };
        Piece {
            figure,
            ident,
            have_not_move_yet: true,
            home_line,
            move_rule,
        }
    }
}
