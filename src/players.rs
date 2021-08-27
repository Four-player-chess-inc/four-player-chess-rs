use crate::ident::Ident::{self, First, Fourth, Second, Third};
use crate::state::State;
use std::default::Default;
use std::iter::once;
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct PlayerIdentMut<'a> {
    pub player: &'a mut Player,
    pub ident: Ident,
}

impl<'a> Players {
    pub fn get_player_mut(&'a mut self, ident: Ident) -> &'a mut Player {
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

    // TODO: ugly, short to vec or once like iter_mut
    pub fn iter(&self) -> PlayersIter {
        PlayersIter {
            players: self,
            ident: Some(First),
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = PlayerIdentMut> {
        once(PlayerIdentMut {
            player: &mut self.first,
            ident: Ident::First,
        })
        .chain(once(PlayerIdentMut {
            player: &mut self.second,
            ident: Ident::Second,
        }))
        .chain(once(PlayerIdentMut {
            player: &mut self.third,
            ident: Ident::Third,
        }))
        .chain(once(PlayerIdentMut {
            player: &mut self.fourth,
            ident: Ident::Fourth,
        }))
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
