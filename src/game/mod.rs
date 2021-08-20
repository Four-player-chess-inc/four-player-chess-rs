use std::default::Default;

use crate::board::Board;
use crate::ident::Ident::{self, *};
use crate::last_move::LastMove;
use crate::mv::move_or_capture::MoveOrCapture;
use crate::mv::Mv;
use crate::players::Players;
use crate::position::Position;
use crate::state::State::{Lost, NoSpecial};

mod test;

#[derive(Debug)]
pub enum MakeMoveError {
    GameOver,
    NothingToMove,
    OpponentsPieceMoveAttempt
}

pub struct Game {
    //last_move: Option<LastMove<M>>,
    who_move_next: Option<Ident>,
    who_win: Option<Ident>,
    players: Players,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            //last_move: None,
            who_move_next: Some(Ident::First),
            who_win: None,
            players: Players::default(),
            board: Board::new(),
        }
    }

    pub fn make_move<M: Mv>(&mut self, mv: M) -> Result<(), MakeMoveError> {
        if self.who_move_next.is_none() {
            return Err(MakeMoveError::GameOver);
        }

        mv.make_move(&mut self.board, self.who_move_next.unwrap());
        /*match mv {
            Move::MoveOrCapture { from, to } => unimplemented!(),
            Move::Castling { rook } => unimplemented!(),
            Move::PawnPromotion { from, to } => unimplemented!()
        }*/

        // TODO: update checkmate check stalemate states
        // self.upate_states();

        self.roll_players();

        Ok(())
    }

    pub fn abandon_move(&mut self) {
        if let Some(who_move_next) = self.who_move_next {
            let player = self.players.get_player_mut(who_move_next);
            player.state = Lost;
            self.roll_players();
        }
    }

    fn roll_players(&mut self) {
        let prev_who_move_next = match self.who_move_next {
            Some(who_move_next) => who_move_next,
            None => return,
        };

        for maybe_next_move in prev_who_move_next.spin() {
            //if prev_who_move_next == maybe_next_move {
            //    self.who_move_next = None;
            //    break;
            //}

            let no_lost_players = self.players.iter().filter(|p| p.player.state != Lost);
            if no_lost_players.count() == 1 {
                self.who_move_next = None;
                break;
            }

            let mut player = self.players.get_player_mut(maybe_next_move);
            if player.state.lost_when_turn_is_come() {
                player.state = Lost;
            }

            if player.state == Lost {
                continue;
            }

            self.who_move_next = Some(maybe_next_move);
            break;
        }
    }

    pub fn who_move_next(&self) -> Option<Ident> {
        self.who_move_next
    }

    //pub fn get_last_move(&self) -> Option<&LastMove<M>> {
    //    //self.last_move.as_ref()
    //    unimplemented!()
    //}

    pub fn who_win(&self) -> Option<Ident> {
        match self.who_move_next() {
            None => Some(
                self.players
                    .iter()
                    .filter(|p| p.player.state != Lost)
                    .next()
                    .unwrap()
                    .ident,
            ),
            _ => None,
        }
    }
}
