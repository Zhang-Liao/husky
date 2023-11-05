use crate::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirExprRegion {
    Eager(HirEagerExprRegion),
    Lazy(HirLazyExprRegion),
}

impl HirExprRegion {
    #[track_caller]
    pub fn eager(self) -> HirEagerExprRegion {
        match self {
            HirExprRegion::Eager(slf) => slf,
            HirExprRegion::Lazy(_) => panic!(),
        }
    }
}