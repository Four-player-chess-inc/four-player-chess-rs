use crate::ident::Ident;
use crate::mv::Mv;
use crate::position::Position;

pub struct LastMove<M: Mv> {
    pub who: Ident,
    pub mv: M,
}
