use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Contract {
    None,
    Move,
    Borrow,
    BorrowMut,
    Const,
    Leash,
}

impl Contract {
    pub fn new<TG>(ephem_symbol_modifier_token_group: Option<TG>) -> Self
    where
        TG: Into<Contract>,
    {
        match ephem_symbol_modifier_token_group {
            Some(t) => t.into(),
            None => Contract::None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Contract::None => "",
            Contract::Move => "move ",
            Contract::Borrow => "borrow",
            Contract::BorrowMut => "borrow mut",
            Contract::Const => "const",
            Contract::Leash => todo!(),
        }
    }
}

impl From<SymbolModifier> for Contract {
    fn from(modifier: SymbolModifier) -> Self {
        match modifier {
            SymbolModifier::None => Contract::None,
            SymbolModifier::Mut => Contract::Move,
            SymbolModifier::RefMut => Contract::BorrowMut,
            SymbolModifier::Const => Contract::Const,
            SymbolModifier::Ambersand(_) => todo!(),
            SymbolModifier::AmbersandMut(_) => todo!(),
            SymbolModifier::Le => todo!(),
            SymbolModifier::Tilde => todo!(),
        }
    }
}
