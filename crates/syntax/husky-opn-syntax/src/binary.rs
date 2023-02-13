mod comparison;
mod logic;
mod pure_closed;

pub use comparison::*;
pub use logic::*;
pub use pure_closed::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOpr {
    PureClosed(BinaryPureClosedOpr),
    Comparison(BinaryComparisonOpr),
    ShortCircuitLogic(BinaryShortcuitLogicOpr),
    Assign(Option<BinaryPureClosedOpr>),
    ScopeResolution,
    Curry, // ->
    As,    // as
    Ins,   // :
    In,    // in
}

impl From<Option<BinaryPureClosedOpr>> for BinaryOpr {
    fn from(v: Option<BinaryPureClosedOpr>) -> Self {
        Self::Assign(v)
    }
}

impl From<BinaryShortcuitLogicOpr> for BinaryOpr {
    fn from(v: BinaryShortcuitLogicOpr) -> Self {
        Self::ShortCircuitLogic(v)
    }
}

impl From<BinaryPureClosedOpr> for BinaryOpr {
    fn from(v: BinaryPureClosedOpr) -> Self {
        Self::PureClosed(v)
    }
}

impl From<BinaryComparisonOpr> for BinaryOpr {
    fn from(v: BinaryComparisonOpr) -> Self {
        Self::Comparison(v)
    }
}

impl BinaryOpr {
    pub fn code(self) -> &'static str {
        match self {
            BinaryOpr::PureClosed(pure_opr) => pure_opr.husky_code(),
            BinaryOpr::Assign(None) => "=",
            BinaryOpr::Assign(Some(pure_opr)) => match pure_opr {
                BinaryPureClosedOpr::Add => "+=",
                BinaryPureClosedOpr::BitAnd => "&=",
                BinaryPureClosedOpr::BitOr => "|=",
                BinaryPureClosedOpr::BitXor => "^=",
                BinaryPureClosedOpr::Div => "/=",
                BinaryPureClosedOpr::Mul => "*=",
                BinaryPureClosedOpr::RemEuclid => "%=",
                BinaryPureClosedOpr::Power => "**=",
                BinaryPureClosedOpr::Shl => "<<=",
                BinaryPureClosedOpr::Shr => ">>=",
                BinaryPureClosedOpr::Sub => "-=",
            },
            BinaryOpr::Comparison(cmp_opr) => cmp_opr.husky_code(),
            BinaryOpr::ShortCircuitLogic(logic_opr) => logic_opr.husky_code(),
            BinaryOpr::Curry => "->",
            BinaryOpr::As => todo!(),
            BinaryOpr::Ins => todo!(),
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::In => "in",
        }
    }

    pub fn spaced_code(self) -> &'static str {
        match self {
            BinaryOpr::PureClosed(pure_binary_opr) => pure_binary_opr.spaced_husky_code(),
            BinaryOpr::Comparison(cmp_opr) => cmp_opr.spaced_husky_code(),
            BinaryOpr::ShortCircuitLogic(logic_opr) => logic_opr.spaced_husky_code(),
            BinaryOpr::Assign(opt_binary_opr) => {
                if let Some(binary_opr) = opt_binary_opr {
                    match binary_opr {
                        BinaryPureClosedOpr::Add => " += ",
                        BinaryPureClosedOpr::BitAnd => " &= ",
                        BinaryPureClosedOpr::BitOr => " |= ",
                        BinaryPureClosedOpr::BitXor => " ^= ",
                        BinaryPureClosedOpr::Div => " /= ",
                        BinaryPureClosedOpr::Mul => " *= ",
                        BinaryPureClosedOpr::RemEuclid => " %= ",
                        BinaryPureClosedOpr::Power => " **= ",
                        BinaryPureClosedOpr::Shl => " <<= ",
                        BinaryPureClosedOpr::Shr => " >>= ",
                        BinaryPureClosedOpr::Sub => " -= ",
                    }
                } else {
                    " = "
                }
            }
            BinaryOpr::Curry => " -> ",
            BinaryOpr::As => " as ",
            BinaryOpr::Ins => " : ",
            BinaryOpr::ScopeResolution => " :: ",
            BinaryOpr::In => " in ",
        }
    }
}
