use four_player_chess::game::Game;
use four_player_chess::mv::move_or_capture::MoveOrCapture;
use four_player_chess::position::Position::*;
use four_player_chess::mv::Move;

fn main() {
    let mut game = Game::new();

    while let Some(who_move) = game.who_move_next() {
        println!("make move: {:?}", who_move);

        match game.make_move(Move::move_or_capture(e2, e4)) {
            Ok(()) => println!("move success"),
            Err(e) => println!("move failed due to error {:?}", e),
        }
    }

    //println!("gave over, winner is {:?}", game.who_win().unwrap());
}
