#[cfg(test)]
mod tests {
    use crate::board::PieceBoardTrait;
    use crate::four_player_chess::FourPlayerChess;
    use crate::ident::Ident::*;
    use crate::mv::{MakeMoveError, Move};
    use crate::piece::Figure;
    use crate::position::Position::*;
    use crate::state::State::*;

    #[test]
    fn test_roll_players() {
        let mut game = FourPlayerChess::new();
        assert_eq!(game.who_move_next(), Some(First));
        game.surrender();
        assert_eq!(game.who_move_next(), Some(Second));
        game.surrender();
        assert_eq!(game.who_move_next(), Some(Third));
        game.surrender();
        assert_eq!(game.who_move_next(), None);
        assert_eq!(game.who_win(), Some(Fourth));

        let mut game = FourPlayerChess::new();
        assert_eq!(game.who_move_next(), Some(First));
        assert_eq!(game.who_move_next(), Some(First));
        game.surrender();
        assert_eq!(game.who_move_next(), Some(Second));
        game.make_move(Move::move_or_capture(b8, d8)).unwrap();
        assert_eq!(game.who_move_next(), Some(Third));
        game.surrender();
        assert_eq!(game.who_move_next(), Some(Fourth));
        game.surrender();
        assert_eq!(game.who_move_next(), None);
        assert_eq!(game.who_win(), Some(Second));

        let mut game = FourPlayerChess::new();
        game.make_move(Move::move_or_capture(g2, g3)).unwrap();
        game.make_move(Move::move_or_capture(b7, c7)).unwrap();
        game.surrender();
        game.surrender();
        assert_eq!(game.who_move_next(), Some(First));
        game.make_move(Move::move_or_capture(g1, g2)).unwrap();
        assert_eq!(game.who_move_next(), Some(Second));
        let t = game.make_move(Move::move_or_capture(c7, d7));
        assert_eq!(t.unwrap_err(), MakeMoveError::MoveUnderCheck);

        let mut game = FourPlayerChess::new();
        game.make_move(Move::move_or_capture(h2, h3)).unwrap();
        game.make_move(Move::move_or_capture(b8, d8)).unwrap();
        game.make_move(Move::move_or_capture(h13, h12)).unwrap();
        game.surrender();
        game.make_move(Move::move_or_capture(h3, h4)).unwrap();
        game.make_move(Move::move_or_capture(d8, e8)).unwrap();
        game.make_move(Move::move_or_capture(h12, h11)).unwrap();
        game.make_move(Move::move_or_capture(h4, h5)).unwrap();

        game.make_move(Move::move_or_capture(e8, f8)).unwrap();
        game.make_move(Move::move_or_capture(h14, h13)).unwrap();
        game.make_move(Move::move_or_capture(h5, h6)).unwrap();
        game.make_move(Move::move_or_capture(f8, g8)).unwrap();
        game.make_move(Move::move_or_capture(h13, g12)).unwrap();
        game.make_move(Move::move_or_capture(h6, h7)).unwrap();
        game.make_move(Move::move_or_capture(b10, c10)).unwrap();
        game.make_move(Move::move_or_capture(k13, k12)).unwrap();
        game.make_move(Move::move_or_capture(k2, k3)).unwrap();
        game.make_move(Move::move_or_capture(a9, b10)).unwrap();
        game.make_move(Move::move_or_capture(d13, d12)).unwrap();
        game.make_move(Move::move_or_capture(h1, h2)).unwrap();
        game.make_move(Move::move_or_capture(b4, c4)).unwrap();
        game.make_move(Move::move_or_capture(e13, e12)).unwrap();

        game.board.piece_board((Figure::King, First)).unwrap();
        game.make_move(Move::move_or_capture(h2, g3)).unwrap();
        assert_eq!(game.players.first.state, NoSpecial);
        game.make_move(Move::move_or_capture(g8, h8)).unwrap();
        assert_eq!(game.players.first.state, Check);
        game.make_move(Move::move_or_capture(g12, g3)).unwrap();
        assert_eq!(game.players.first.state, Lost);
    }
}
