use four_player_chess::four_player_chess::FourPlayerChess;
use four_player_chess::mv::Move;
use four_player_chess::position::Position::*;

fn main() {
    let mut game = FourPlayerChess::new();

    while let Some(who_move) = game.who_move_next() {
        println!("make move: {:?}", who_move);

        match game.make_move(Move::move_or_capture(e2, e4)) {
            Ok(()) => println!("move success"),
            Err(e) => println!("move failed due to error {:?}", e),
        }
    }
    println!("gave over, winner is {:?}", game.who_win().unwrap());
}
