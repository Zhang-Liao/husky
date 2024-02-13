mod category;
mod contract;
mod curry;
mod db;
mod error;
mod final_destination;
mod item_path;
pub mod literal;
mod modifier;
pub mod symbol;
pub mod template_var_class;
mod universe;

pub use self::category::*;
pub use self::contract::*;
pub use self::curry::*;
pub use self::error::*;
pub use self::final_destination::*;
pub use self::item_path::*;
pub use self::modifier::*;
pub use self::universe::*;

use husky_entity_kind::ritchie::RitchieItemKind;
use husky_entity_path::*;

#[salsa::jar]
pub struct TermPreludeJar(
    crate::literal::int::TermI64Literal,
    crate::literal::int::TermI128Literal,
    crate::literal::int::TermI256Literal,
    crate::literal::int::TermISizeLiteral,
    crate::literal::int::TermU64Literal,
    crate::literal::int::TermU128Literal,
    crate::literal::int::TermU256Literal,
    crate::literal::int::TermUSizeLiteral,
    crate::literal::int::TermR64Literal,
    crate::literal::int::TermR128Literal,
    crate::literal::int::TermR256Literal,
    crate::literal::int::TermRSizeLiteral,
    crate::literal::float::TermF32Literal,
    crate::literal::float::TermF64Literal,
    crate::literal::TermNatLiteral,
    crate::literal::StringLiteralTokenData,
);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum RitchieKind {
    Type(RitchieItemKind),
    Trait(RitchieTraitKind),
}

impl RitchieKind {
    pub const RITCHIE_TYPE_FN: Self = RitchieKind::Type(RitchieItemKind::Fn);
    pub const RITCHIE_TYPE_GN: Self = RitchieKind::Type(RitchieItemKind::Gn);
    pub const RITCHIE_TYPE_VN: Self = RitchieKind::Type(RitchieItemKind::Vn);
    pub const RITCHIE_TYPE_PN: Self = RitchieKind::Type(RitchieItemKind::Pn);
    pub const RITCHIE_TYPE_QN: Self = RitchieKind::Type(RitchieItemKind::Qn);
    pub const RITCHIE_TYPE_BN: Self = RitchieKind::Type(RitchieItemKind::Bn);

    pub fn code(self) -> &'static str {
        match self {
            RitchieKind::Type(ritchie_ty_kind) => match ritchie_ty_kind {
                RitchieItemKind::Fn => "fn(",
                RitchieItemKind::Gn => "gn(",
                RitchieItemKind::Vn => "vn(",
                RitchieItemKind::Pn => "pn(",
                RitchieItemKind::Qn => "qn(",
                RitchieItemKind::Bn => "bn(",
            },
            RitchieKind::Trait(ritchie_trai_kind) => match ritchie_trai_kind {
                RitchieTraitKind::Fn => "Fn(",
                RitchieTraitKind::FnMut => "FnMut(",
                RitchieTraitKind::FnOnce => "FnOnce(",
                RitchieTraitKind::Gn => "Gn(",
            },
        }
    }

    pub fn ritchie_ty_kind(self) -> Option<RitchieItemKind> {
        match self {
            RitchieKind::Type(ritchie_ty_kind) => Some(ritchie_ty_kind),
            RitchieKind::Trait(_) => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RitchieTraitKind {
    Fn,
    FnMut,
    FnOnce,
    Gn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
pub enum TypeFinalDestinationExpectation {
    EqsSort,
    EqsNonSortTypePath(TypePath),
    Any,
}

/// a type path can be interpreted in two different ways:
///
/// - \[explicit curries to\] a type
/// - \[explicit curries to\] a type constructor
///
/// the final curry destination of the two different interpretation are different
///
/// for example, the type of type path `List` can either be
///
/// - `∀ universe u, explicit covariant Sort u -> Sort u`,
///
///     the final curry destination is in universe `Sort u`
/// - `∀ universe u, explicit covariant (E: Sort u) -> () -> List E`,
///
///     the final curry destination is in universe `List E`
/// disambiguate between type itself (or template) and its instance or constructor
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypePathDisambiguation {
    OntologyConstructor,
    /// if type is a unit struct, this will become an instance,
    ///
    /// otherwise constructor
    InstanceConstructor,
}
