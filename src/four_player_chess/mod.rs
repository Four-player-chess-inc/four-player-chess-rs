use std::collections::HashMap;
use std::default::Default;

use crate::board::Board;
use crate::ident::Ident::{self, First, Fourth, Second, Third};
use crate::last_move::LastMove;
use crate::mv::{MakeMoveError, MakeMoveOk, Move};
use crate::players::Players;
use crate::state::State;
use crate::state::State::Lost;
use enum_iterator::all;

mod test;

pub struct FourPlayerChess {
    last_move: Option<LastMove>,
    who_move_next: Option<Ident>,
    players: Players,
    board: Board,
}

impl FourPlayerChess {
    pub fn new() -> FourPlayerChess {
        FourPlayerChess {
            last_move: None,
            who_move_next: Some(Ident::First),
            players: Players::default(),
            board: Board::new(),
        }
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), MakeMoveError> {
        let who_move_next = match self.who_move_next {
            Some(wmn) => wmn,
            None => return Err(MakeMoveError::GameOver),
        };

        let ok = match mv.make_move(&mut self.board, who_move_next) {
            Ok(ok) => ok,
            Err(e) => return Err(e),
        };

        if let MakeMoveOk::KingCaptured(ident) = ok {
            self.players.get_player_mut(ident).state = State::Lost;
            self.board
                .pieces
                .iter_mut()
                .filter(|p| p.1.attrib().ident == ident)
                .for_each(|p| p.1.attrib_mut().stone = true);
        }

        self.last_move = Some(LastMove {
            who: who_move_next,
            mv,
        });

        self.update_checkmates();

        self.roll_players();

        Ok(())
    }

    pub fn surrender(&mut self) {
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

        for maybe_next_move in all::<Ident>()
            .cycle()
            .skip_while(|i| *i != prev_who_move_next)
            .skip(1)
        {
            let no_lost_players = self.players.iter().filter(|p| p.player.state != Lost);
            if no_lost_players.count() == 1 {
                self.who_move_next = None;
                break;
            }

            let mut player = self.players.get_player_mut(maybe_next_move);
            if player.state.lost_when_turn_is_come() {
                player.state = Lost;
                self.board
                    .pieces
                    .iter_mut()
                    .filter(|p| p.1.attrib().ident == maybe_next_move)
                    .for_each(|p| p.1.attrib_mut().stone = true);
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

    pub fn get_last_move(&self) -> Option<&LastMove> {
        self.last_move.as_ref()
    }

    pub fn get_players_states(&self) -> HashMap<Ident, State> {
        HashMap::from([
            (First, self.players.first.state),
            (Second, self.players.second.state),
            (Third, self.players.third.state),
            (Fourth, self.players.fourth.state),
        ])
    }

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

    fn update_checkmates(&mut self) {
        let players = self
            .players
            .iter_mut()
            .filter(|p| p.player.state != State::Lost);

        for player in players {
            player.player.state = self
                .board
                .check_checkmate_on_board(player.ident)
                .unwrap()
                .into();
        }
    }
}
