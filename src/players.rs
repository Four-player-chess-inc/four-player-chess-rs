use crate::ident::Ident::{self, First, Fourth, Second, Third};
use crate::state::State;
use std::default::Default;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Player {
    pub state: State,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            state: State::NoSpecial,
        }
    }
}

pub struct Players {
    first: Player,
    second: Player,
    third: Player,
    fourth: Player,
}

pub struct PlayersIter<'a> {
    players: &'a Players,
    ident: Option<Ident>,
}

#[derive(Debug)]
pub struct PlayerIdent<'a> {
    pub player: &'a Player,
    pub ident: Ident,
}

impl Players {
    pub fn get_player_mut(&mut self, ident: Ident) -> &mut Player {
        match ident {
            First => &mut self.first,
            Second => &mut self.second,
            Third => &mut self.third,
            Fourth => &mut self.fourth,
        }
    }
    pub fn get_player(&self, ident: Ident) -> &Player {
        match ident {
            First => &self.first,
            Second => &self.second,
            Third => &self.third,
            Fourth => &self.fourth,
        }
    }
    pub fn iter(&self) -> PlayersIter {
        PlayersIter {
            players: self,
            ident: Some(First),
        }
    }
}

impl Default for Players {
    fn default() -> Self {
        Players {
            first: Player::default(),
            second: Player::default(),
            third: Player::default(),
            fourth: Player::default(),
        }
    }
}

impl<'a> Iterator for PlayersIter<'a> {
    type Item = PlayerIdent<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let ident = match self.ident {
            None => return None,
            Some(Fourth) => {
                self.ident = None;
                Fourth
            }
            Some(ident) => {
                self.ident = Some(ident.spin().next().unwrap());
                ident
            }
        };
        Some(PlayerIdent {
            player: self.players.get_player(ident),
            ident,
        })
    }
}
