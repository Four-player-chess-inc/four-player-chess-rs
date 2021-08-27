use crate::board::CheckMate;
use std::convert::From;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    NoSpecial,
    Check,
    Checkmate,
    Stalemate,
    Lost,
}

impl State {
    pub fn lost_when_turn_is_come(&self) -> bool {
        match self {
            Self::Checkmate | Self::Stalemate => true,
            Self::Lost | Self::NoSpecial | Self::Check => false,
        }
    }
}

impl From<CheckMate> for State {
    fn from(checkmate: CheckMate) -> Self {
        match checkmate {
            CheckMate::No => Self::NoSpecial,
            CheckMate::Stalemate => Self::Stalemate,
            CheckMate::Checkmate => Self::Checkmate,
            CheckMate::Check => Self::Check,
        }
    }
}
