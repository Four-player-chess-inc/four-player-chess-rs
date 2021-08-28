#[cfg(test)]
mod test {
    use crate::board::{Board, CheckMate};
    use crate::ident::Ident;
    use crate::ident::Ident::{First, Fourth, Second, Third};
    use crate::piece::Figure;
    use crate::position::Position::{self, *};

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

    #[test]
    fn check_checkmate_on_board() {
        let mut board = Board::new();
        assert_eq!(
            board.check_checkmate_on_board(First).unwrap(),
            CheckMate::No
        );
        assert_eq!(
            board.check_checkmate_on_board(Second).unwrap(),
            CheckMate::No
        );
        assert_eq!(
            board.check_checkmate_on_board(Third).unwrap(),
            CheckMate::No
        );
        assert_eq!(
            board.check_checkmate_on_board(Fourth).unwrap(),
            CheckMate::No
        );
        board.piece_move(g2, g3);
        board.piece_move(g1, g2);
        board.piece_move(b7, c7);
        assert_eq!(
            board.check_checkmate_on_board(Second).unwrap(),
            CheckMate::Check
        );
    }
}
