#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::ident::Ident;
    use crate::mv::move_or_capture::MoveOrCapture;
    use crate::players::{Player, Players};
    use crate::position::Position;
    use crate::state::State;

    #[test]
    fn test_roll_players() {
        let mut game = Game::new();
        assert_eq!(game.who_move_next(), Some(Ident::First));
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(Ident::Second));
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(Ident::Third));
        game.abandon_move();
        assert_eq!(game.who_move_next(), None);
        assert_eq!(game.who_win(), Some(Ident::Fourth));

        let mut game = Game::new();
        assert_eq!(game.who_move_next(), Some(Ident::First));
        assert_eq!(game.who_move_next(), Some(Ident::First));
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(Ident::Second));
        game.make_move(MoveOrCapture {
            from: Position::e2,
            to: Position::e4,
        });
        assert_eq!(game.who_move_next(), Some(Ident::Third));
        game.abandon_move();
        assert_eq!(game.who_move_next(), Some(Ident::Fourth));
        game.abandon_move();
        assert_eq!(game.who_move_next(), None);
        assert_eq!(game.who_win(), Some(Ident::Second));
    }
}
