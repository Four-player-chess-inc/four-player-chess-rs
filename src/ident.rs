use std::iter::Iterator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ident {
    First,
    Second,
    Third,
    Fourth,
}

impl Ident {
    pub fn spin(&self) -> IdentSpin {
        IdentSpin { ident: *self }
    }
}

pub struct IdentSpin {
    ident: Ident,
}

impl Iterator for IdentSpin {
    type Item = Ident;
    fn next(&mut self) -> Option<Self::Item> {
        self.ident = match self.ident {
            Ident::First => Ident::Second,
            Ident::Second => Ident::Third,
            Ident::Third => Ident::Fourth,
            Ident::Fourth => Ident::First,
        };
        Some(self.ident)
    }
}
