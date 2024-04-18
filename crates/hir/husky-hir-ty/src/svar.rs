mod r#const;
mod lifetime;
mod quary;
mod ty;

pub use self::lifetime::*;
pub use self::quary::*;
pub use self::r#const::*;
pub use self::ty::*;

use crate::*;
use husky_eth_term::term::svar::{
    EthSymbolicVariable, EthTemplateSymbolAttrs, EthTermSymbolIndexImpl,
};
use husky_term_prelude::template_var_class::TemplateVariableClass;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTemplateVariable {
    Type(HirTypeTemplateVariable),
    Const(HirConstTemplateVariable),
    Lifetime(HirLifetimeTemplateVariable),
    Quary(HirQuaryTemplateVariable),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirTemplateVariableAttrs {
    class: HirTemplateVariableClass,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTemplateVariableClass {
    Comptime,
    Runtime,
}

impl HirTemplateVariableClass {
    fn from_term(class: TemplateVariableClass) -> Option<Self> {
        match class {
            TemplateVariableClass::Phantom => None,
            TemplateVariableClass::Runtime => Some(HirTemplateVariableClass::Runtime),
            TemplateVariableClass::Comptime => Some(HirTemplateVariableClass::Comptime),
        }
    }
}

impl HirTemplateVariableAttrs {
    pub(crate) fn from_eth(attrs: EthTemplateSymbolAttrs) -> Option<Self> {
        Some(Self {
            class: HirTemplateVariableClass::from_term(attrs.class)?,
        })
    }
}

impl HirTemplateVariable {
    pub fn from_eth(symbol: EthSymbolicVariable, db: &::salsa::Db) -> Option<Self> {
        hir_template_variable_from_eth(db, symbol)
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_template_variable_from_eth(
    db: &::salsa::Db,
    var: EthSymbolicVariable,
) -> Option<HirTemplateVariable> {
    match var.index(db).inner() {
        EthTermSymbolIndexImpl::ExplicitLifetime {
            attrs,
            variance,
            disambiguator,
        } => Some(
            HirLifetimeTemplateVariable {
                attrs: HirTemplateVariableAttrs::from_eth(attrs)?,
                variance,
                disambiguator,
            }
            .into(),
        ),
        EthTermSymbolIndexImpl::ExplicitPlace {
            attrs,
            variance,
            disambiguator,
        } => Some(
            HirQuaryTemplateVariable {
                attrs: HirTemplateVariableAttrs::from_eth(attrs)?,
                variance,
                disambiguator,
            }
            .into(),
        ),
        EthTermSymbolIndexImpl::Type {
            attrs,
            variance,
            disambiguator,
        } => Some(
            HirTypeTemplateVariable::Type {
                attrs: HirTemplateVariableAttrs::from_eth(attrs)?,
                variance,
                disambiguator,
            }
            .into(),
        ),
        EthTermSymbolIndexImpl::Prop { disambiguator: _ } => todo!(),
        EthTermSymbolIndexImpl::ConstPathLeading {
            attrs,
            disambiguator,
            ty_path,
        } => Some(
            HirConstTemplateVariable::new(
                db,
                HirType::from_eth(var.ty(db), db)?,
                HirConstTemplateVariableIndex::PathLeading {
                    attrs: HirTemplateVariableAttrs::from_eth(attrs)?,
                    disambiguator,
                    ty_path,
                },
            )
            .into(),
        ),
        EthTermSymbolIndexImpl::ConstOther {
            attrs,
            disambiguator,
        } => Some(
            HirConstTemplateVariable::new(
                db,
                HirType::from_eth(var.ty(db), db)?,
                HirConstTemplateVariableIndex::Other {
                    attrs: HirTemplateVariableAttrs::from_eth(attrs)?,
                    disambiguator,
                },
            )
            .into(),
        ),
        EthTermSymbolIndexImpl::EphemPathLeading {
            disambiguator: _,
            ty_path: _,
        } => None,
        EthTermSymbolIndexImpl::EphemOther { disambiguator: _ } => None,
        EthTermSymbolIndexImpl::SelfType => Some(HirTypeTemplateVariable::SelfType.into()),
        EthTermSymbolIndexImpl::SelfValue => todo!(),
        EthTermSymbolIndexImpl::SelfLifetime => Some(HirTypeTemplateVariable::SelfLifetime.into()),
        EthTermSymbolIndexImpl::SelfPlace => Some(HirTypeTemplateVariable::SelfPlace.into()),
    }
}
