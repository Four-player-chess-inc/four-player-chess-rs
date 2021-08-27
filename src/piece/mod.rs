mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use crate::board::Board;
use crate::ident::Ident;
use crate::piece::bishop::Bishop;
use crate::piece::king::King;
use crate::piece::knight::Knight;
use crate::piece::pawn::Pawn;
use crate::piece::queen::Queen;
use crate::piece::rook::Rook;
use crate::position::Position;

#[derive(Clone, Debug)]
pub(crate) enum Piece {
    Pawn(Pawn),
    Bishop(Bishop),
    Knight(Knight),
    Queen(Queen),
    King(King),
    Rook(Rook),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Figure {
    Pawn,
    Bishop,
    Knight,
    Queen,
    King,
    Rook,
}

#[derive(Clone, Debug)]
pub(crate) struct CommonAttrib {
    pub(crate) ident: Ident,
    pub(crate) have_not_move_yet: bool,
    pub(crate) stone: bool, //pub(crate) home_line: Line,
}

impl Piece {
    pub fn new(figure: Figure, ident: Ident) -> Piece {
        let attrib = CommonAttrib {
            ident,
            have_not_move_yet: true,
            stone: false,
            //home_line,
        };

        match figure {
            Figure::Pawn => Self::Pawn(Pawn { attrib }),
            Figure::Bishop => Self::Bishop(Bishop { attrib }),
            Figure::Knight => Self::Knight(Knight { attrib }),
            Figure::Queen => Self::Queen(Queen { attrib }),
            Figure::King => Self::King(King { attrib }),
            Figure::Rook => Self::Rook(Rook { attrib }),
        }
    }

    pub fn attrib(&self) -> &CommonAttrib {
        match self {
            Self::Pawn(i) => &i.attrib,
            Self::Bishop(i) => &i.attrib,
            Self::Knight(i) => &i.attrib,
            Self::Queen(i) => &i.attrib,
            Self::King(i) => &i.attrib,
            Self::Rook(i) => &i.attrib,
        }
    }

    pub fn attrib_mut(&mut self) -> &mut CommonAttrib {
        match self {
            Self::Pawn(i) => &mut i.attrib,
            Self::Bishop(i) => &mut i.attrib,
            Self::Knight(i) => &mut i.attrib,
            Self::Queen(i) => &mut i.attrib,
            Self::King(i) => &mut i.attrib,
            Self::Rook(i) => &mut i.attrib,
        }
    }

    pub fn figure(&self) -> Figure {
        match self {
            Self::Pawn(_) => Figure::Pawn,
            Self::Bishop(_) => Figure::Bishop,
            Self::Knight(_) => Figure::Knight,
            Self::Queen(_) => Figure::Queen,
            Self::King(_) => Figure::King,
            Self::Rook(_) => Figure::Rook,
        }
    }

    pub fn move_variants(&self, board: &Board, pos: Position) -> Vec<Position> {
        match self {
            Self::Pawn(i) => i.move_variants(board, pos),
            Self::Bishop(i) => i.move_variants(board, pos),
            Self::Knight(i) => i.move_variants(board, pos),
            Self::Queen(i) => i.move_variants(board, pos),
            Self::King(i) => i.move_variants(board, pos),
            Self::Rook(i) => i.move_variants(board, pos),
        }
    }
}
