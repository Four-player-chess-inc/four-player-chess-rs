use crate::ident::Ident;
use crate::mv::Mv;

pub struct LastMove<M: Mv> {
    pub who: Ident,
    pub mv: M,
}
