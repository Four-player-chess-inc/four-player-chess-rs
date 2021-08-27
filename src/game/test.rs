#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::ident::Ident::{*};
    use crate::mv::move_or_capture::MoveOrCapture;
    use crate::mv::MakeMoveError;
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
        game.make_move(MoveOrCapture { from: b8, to: d8 }).unwrap();
        assert_eq!(game.who_move_next(), Some(Third));
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(Fourth));
        game.abandon_move();
        assert_eq!(game.who_move_next(), None);
        assert_eq!(game.who_win(), Some(Second));

        let mut game = Game::new();
        game.make_move(MoveOrCapture { from: g2, to: g3 }).unwrap();
        game.make_move(MoveOrCapture { from: b7, to: c7 }).unwrap();
        game.abandon_move();
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(First));
        game.make_move(MoveOrCapture { from: g1, to: g2 }).unwrap();
        assert_eq!(game.who_move_next(), Some(Second));
        let x = game.make_move(MoveOrCapture { from: c7, to: d7 });
        assert_eq!(x.unwrap_err(), MakeMoveError::MoveUnderCheck);
    }
}
