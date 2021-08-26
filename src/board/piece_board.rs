use crate::board::Board;
use crate::ident::Ident;
use crate::piece::{Figure, Piece};
use crate::position::Position::{self, *};

pub(crate) struct PieceBoard<'a> {
    pub(crate) piece: &'a Piece,
    pub(crate) position: Position,
    pub(crate) board: &'a Board,
}

impl<'a> PieceBoard<'a> {
    pub fn move_variants(&self) -> Vec<Position> {
        self.piece.move_variants(self.board, self.position)
    }

    pub fn under_attack_any(&self) -> Option<Position> {
        self.board
            .under_attack_any(self.position, self.piece.attrib().ident)
    }
}

pub(crate) trait PieceBoardTrait<T> {
    fn piece_board(&self, find: T) -> Option<PieceBoard>;
}

impl PieceBoardTrait<Position> for Board {
    fn piece_board(&self, find: Position) -> Option<PieceBoard> {
        if let Some(piece) = self.pieces.get(&find) {
            return Some(PieceBoard {
                piece,
                position: find,
                board: self,
            });
        }
        None
    }
}

impl PieceBoardTrait<(Figure, Ident)> for Board {
    fn piece_board(&self, find: (Figure, Ident)) -> Option<PieceBoard> {
        let find = self
            .pieces
            .iter()
            .find(|p| p.1.figure() == find.0 && p.1.attrib().ident == find.1);
        if let Some((pos, piece)) = find {
            return Some(PieceBoard {
                piece,
                position: *pos,
                board: self,
            });
        }
        None
    }
}

#[test]
fn move_variants() {
    let board = Board::new();
    let t = board.piece_board(e2).unwrap();
    assert_eq!(t.move_variants(), vec![e3, e4]);
    let t = board.piece_board(e1).unwrap();
    assert_eq!(t.move_variants(), vec![d3, f3]);
    let t = board.piece_board(g1).unwrap();
    assert_eq!(t.move_variants(), vec![]);

    let mut board = Board::new();
    board.pieces.remove(&f2);
    let t = board.piece_board(g1).unwrap();
    assert_eq!(t.move_variants(), vec![f2, e3, d4, c5, b6]);

    let mut board = Board::new();
    board.piece_move(g1, a7);
    board.pieces.remove(&b8);
    let t = board.piece_board(a7).unwrap();
    assert_eq!(
        t.move_variants(),
        vec![a8, b8, c9, d10, e11, f12, g13, b7, b6, a6]
    );
}
