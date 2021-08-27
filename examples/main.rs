use four_player_chess_rs::mv::move_or_capture::MoveOrCapture;
use four_player_chess_rs::position::Position;
use four_player_chess_rs::game::Game;

fn main() {
    let mut game = Game::new();

    while let Some(who_move) = game.who_move_next() {
        println!("make move: {:?}", who_move);

        let mv = MoveOrCapture {
            from: Position::e2,
            to: Position::e4,
        };

        match game.make_move(mv) {
            Ok(()) => println!("move success"),
            Err(e) => println!("move failed due to error {:?}", e),
        }
    }

    //println!("gave over, winner is {:?}", game.who_win().unwrap());
}
