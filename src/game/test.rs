#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::ident::Ident::{self, *};
    use crate::mv::move_or_capture::MoveOrCapture;
    use crate::players::{Player, Players};
    use crate::position::Position::{self, *};
    use crate::state::State;

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
        game.make_move(MoveOrCapture { from: b8, to: d8 });
        assert_eq!(game.who_move_next(), Some(Third));
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(Fourth));
        game.abandon_move();
        assert_eq!(game.who_move_next(), None);
        assert_eq!(game.who_win(), Some(Second));
    }
}
