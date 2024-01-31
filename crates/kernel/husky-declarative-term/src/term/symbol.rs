mod index;
mod set;

use crate::helpers::DeclarativeTermFamily;

pub use self::index::*;
pub use self::set::*;

use super::*;
use husky_entity_tree::SynNodeRegionPath;
use thiserror::Error;
use vec_like::VecSet;

/// symbols are defined in a top-down manner through generics
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct SymbolDeclarativeTerm {
    pub toolchain: Toolchain,
    pub ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    /// todo: change to RefinedGenericIndex
    pub index: DeclarativeTermSymbolIndex,
}

impl SymbolDeclarativeTerm {
    pub(crate) const AD_HOD_IDX_START: u8 = u8::MAX / 2;

    #[inline(always)]
    pub fn new_self_ty(
        db: &::salsa::Db,
        toolchain: Toolchain,
        registry: &mut TermSymbolRegistry,
    ) -> Self {
        // todo: general universe??? or ignore universes totally
        SymbolDeclarativeTerm::new(
            db,
            toolchain,
            Ok(DeclarativeTerm::TYPE),
            registry.issue_self_ty_index(),
        )
    }

    #[inline(always)]
    pub fn new_self_value(
        db: &::salsa::Db,
        toolchain: Toolchain,
        registry: &mut TermSymbolRegistry,
        _self_ty_term: DeclarativeTerm,
    ) -> Self {
        // todo: general universe??? or ignore universes totally
        SymbolDeclarativeTerm::new(
            db,
            toolchain,
            Ok(DeclarativeTerm::TYPE),
            registry.issue_self_value_index(),
        )
    }

    #[inline(always)]
    pub fn new_lifetime(
        db: &::salsa::Db,
        toolchain: Toolchain,
        menu: &DeclarativeTermMenu,
        registry: &mut TermSymbolRegistry,
        attrs: DeclarativeTemplateSymbolAttrs,
        variance: Option<Variance>,
    ) -> (DeclarativeTermSymbolTypeResult<DeclarativeTerm>, Self) {
        let ty = Ok(menu.lifetime_ty());
        (
            ty,
            Self::new(
                db,
                toolchain,
                ty,
                registry.issue_explicit_lifetime_index(attrs, variance),
            ),
        )
    }

    #[inline(always)]
    pub fn new_place(
        db: &::salsa::Db,
        toolchain: Toolchain,
        menu: &DeclarativeTermMenu,
        registry: &mut TermSymbolRegistry,
        attrs: DeclarativeTemplateSymbolAttrs,
        variance: Option<Variance>,
    ) -> (DeclarativeTermSymbolTypeResult<DeclarativeTerm>, Self) {
        let ty = Ok(menu.place_ty());
        (
            ty,
            Self::new(
                db,
                toolchain,
                ty,
                registry.issue_explicit_place_index(attrs, variance),
            ),
        )
    }

    #[inline(always)]
    pub fn new_ty(
        db: &::salsa::Db,
        toolchain: Toolchain,
        menu: &DeclarativeTermMenu,
        registry: &mut TermSymbolRegistry,
        attrs: DeclarativeTemplateSymbolAttrs,
        variance: Option<Variance>,
    ) -> (DeclarativeTermSymbolTypeResult<DeclarativeTerm>, Self) {
        let ty = Ok(menu.ty0().into());
        (
            ty,
            SymbolDeclarativeTerm::new(db, toolchain, ty, registry.issue_ty_index(attrs, variance)),
        )
    }

    pub fn new_const(
        db: &::salsa::Db,
        toolchain: Toolchain,
        attrs: DeclarativeTemplateSymbolAttrs,
        ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
        registry: &mut TermSymbolRegistry,
    ) -> Self {
        let idx = match ty {
            Ok(ty) => match ty.family(db) {
                DeclarativeTermFamily::Sort => todo!(),
                DeclarativeTermFamily::TypePath(ty_path) => {
                    registry.issue_const_path_leading_index(attrs, ty_path)
                }
                DeclarativeTermFamily::Other => registry.issue_const_other_index(attrs),
            },
            Err(_) => registry.issue_const_err_index(attrs),
        };
        Self::new(db, toolchain, ty, idx)
    }

    /// ephem is short for `ephemeral`
    pub fn new_ephem(
        db: &::salsa::Db,
        toolchain: Toolchain,
        ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
        registry: &mut TermSymbolRegistry,
    ) -> Self {
        let idx = match ty {
            Ok(ty) => match ty.family(db) {
                DeclarativeTermFamily::Sort => todo!(),
                DeclarativeTermFamily::TypePath(ty_path) => {
                    registry.issue_ephem_path_leading_index(ty_path)
                }
                DeclarativeTermFamily::Other => registry.issue_ephem_other_index(),
            },
            Err(_) => todo!(),
        };
        Self::new(db, toolchain, ty, idx)
    }

    pub unsafe fn new_ad_hoc(
        db: &::salsa::Db,
        toolchain: Toolchain,
        ty: DeclarativeTerm,
        disambiguator: u8,
    ) -> Self {
        Self::new(
            db,
            toolchain,
            Ok(ty),
            DeclarativeTermSymbolIndex::new_ad_hoc(disambiguator),
        )
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_symbol(db, self, f)
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeclarativeTermSymbolTypeErrorKind {
    #[error("signature declarative_term error")]
    SignatureDeclarativeTermError,
    #[error("sketch declarative_term error")]
    SketchDeclarativeTermError,
    #[error("cannot infer type expression term")]
    CannotInferTypeExprTerm(SynNodeRegionPath),
}

pub type DeclarativeTermSymbolTypeResult<T> = Result<T, DeclarativeTermSymbolTypeErrorKind>;

impl salsa::DisplayWithDb for SymbolDeclarativeTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_fmt(format_args!("${:?}", self.index(db)))
    }
}
