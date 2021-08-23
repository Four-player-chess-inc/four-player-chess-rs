use crate::board::Board;
use crate::ident::Ident;
use crate::piece::Figure;
use crate::position::Position;

#[test]
fn init_board() {
    let board = Board::new();
    assert_eq!(
        board.piece(Position::d1).unwrap().attrib().ident,
        Ident::First
    );
    assert_eq!(board.piece(Position::d1).unwrap().figure(), Figure::Rook);
    assert_eq!(board.piece(Position::g1).unwrap().figure(), Figure::Queen);
    assert_eq!(board.piece(Position::i1).unwrap().figure(), Figure::Bishop);
    assert_eq!(
        board.piece(Position::d2).unwrap().attrib().ident,
        Ident::First
    );
    assert_eq!(board.piece(Position::d2).unwrap().figure(), Figure::Pawn);
    assert_eq!(
        board.piece(Position::n7).unwrap().attrib().ident,
        Ident::Fourth
    );
    assert_eq!(board.piece(Position::n7).unwrap().figure(), Figure::King);
    assert_eq!(board.piece(Position::e1).unwrap().figure(), Figure::Knight);
}
