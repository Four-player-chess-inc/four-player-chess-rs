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

#[derive(Debug)]
pub(crate) struct Players {
    pub(crate) first: Player,
    pub(crate) second: Player,
    pub(crate) third: Player,
    pub(crate) fourth: Player,
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
    pub(crate) fn get_player_mut(&'a mut self, ident: Ident) -> &'a mut Player {
        match ident {
            First => &mut self.first,
            Second => &mut self.second,
            Third => &mut self.third,
            Fourth => &mut self.fourth,
        }
    }

    /*pub(crate) fn get_player(&self, ident: Ident) -> &Player {
        match ident {
            First => &self.first,
            Second => &self.second,
            Third => &self.third,
            Fourth => &self.fourth,
        }
    }*/


    pub(crate) fn iter(&self) -> impl Iterator<Item = PlayerIdent> {
        vec![  PlayerIdent {
            player: &self.first,
            ident: Ident::First,
        }, PlayerIdent {
            player: &self.second,
            ident: Ident::Second,
        }, PlayerIdent {
            player: &self.third,
            ident: Ident::Third,
        }, PlayerIdent {
            player: &self.fourth,
            ident: Ident::Fourth} ].into_iter()
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = PlayerIdentMut> {
        vec![  PlayerIdentMut {
            player: &mut self.first,
            ident: Ident::First,
        }, PlayerIdentMut {
            player: &mut self.second,
            ident: Ident::Second,
        }, PlayerIdentMut {
            player: &mut self.third,
            ident: Ident::Third,
        }, PlayerIdentMut {
            player: &mut self.fourth,
            ident: Ident::Fourth} ].into_iter()
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
