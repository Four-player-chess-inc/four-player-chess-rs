#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::ident::Ident::*;
    use crate::mv::move_or_capture::MoveOrCapture;
    use crate::mv::{MakeMoveError, Move};
    use crate::position::Position::*;

    #[test]
    fn test_roll_players() {
        let mut game = Game::new();
        assert_eq!(game.who_move_next(), Some(First));
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(Second));
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(Third));
        game.abandon_move();
        assert_eq!(game.who_move_next(), None);
        assert_eq!(game.who_win(), Some(Fourth));

        let mut game = Game::new();
        assert_eq!(game.who_move_next(), Some(First));
        assert_eq!(game.who_move_next(), Some(First));
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(Second));
        game.make_move(Move::move_or_capture(b8, d8)).unwrap();
        assert_eq!(game.who_move_next(), Some(Third));
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(Fourth));
        game.abandon_move();
        assert_eq!(game.who_move_next(), None);
        assert_eq!(game.who_win(), Some(Second));

        let mut game = Game::new();
        game.make_move(Move::move_or_capture(g2, g3)).unwrap();
        game.make_move(Move::move_or_capture(b7, c7)).unwrap();
        game.abandon_move();
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(First));
        game.make_move(Move::move_or_capture(g1, g2)).unwrap();
        assert_eq!(game.who_move_next(), Some(Second));
        let x = game.make_move(Move::move_or_capture(c7, d7));


        let mut game = Game::new();
    }
}
