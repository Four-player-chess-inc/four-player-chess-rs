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
            Self::Check | Self::Checkmate | Self::Stalemate => true,
            Self::Lost | Self::NoSpecial => false,
        }
    }
}
