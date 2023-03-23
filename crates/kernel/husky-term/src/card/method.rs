mod trai;
mod ty;
mod ty_as_trai;

pub use self::trai::*;
pub use self::ty::*;
pub use self::ty_as_trai::*;

use crate::*;
use husky_decl::{TypeItemDecl, TypeMethodFnDecl};
use husky_entity_tree::{AssociatedItemId, EntityTreeBundleResult};
use husky_raw_ty::ty_path_ty_method_raw_ty;
use husky_signature::{SignatureResult, TypeMethodSignature};
use husky_word::IdentPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum MethodCard {
    TypeMethodFn(TypeMethodFnCard),
    TypeAsTraitMethodFn(TypeAsTraitMethodFnCard),
}

#[salsa::tracked(db = TermDb, jar = TermJar, constructor = new_inner)]
pub struct TypeAsTraitMethodFnCard {
    #[id]
    id: AssociatedItemId,
    method_ty: TermResult<Term>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MethodTypeInfo {
    implicit_parameters: Vec<TermSymbol>,
    self_liasoned_ty: TermRitchieParameterLiasonedType,
    nonself_parameter_liasoned_tys: Vec<TermRitchieParameterLiasonedType>,
    return_ty: Term,
    where_clause: (),
}

impl MethodTypeInfo {
    fn ty(&self, db: &dyn TermDb) -> TermResult<Term> {
        match self {
            MethodTypeInfo {
                implicit_parameters,
                self_liasoned_ty,
                nonself_parameter_liasoned_tys,
                return_ty,
                where_clause,
            } => {
                let mut parameter_tys = vec![];
                let mut method_ty: Term = TermRitchie::new_unchecked(
                    db,
                    TermRitchieKind::FnType,
                    parameter_tys,
                    *return_ty,
                )
                .into();
                for implicit_signature in implicit_parameters.iter().copied() {
                    todo!()
                }
                method_ty.checked(db)
            }
        }
    }

    pub fn implicit_parameters(&self) -> &[TermSymbol] {
        self.implicit_parameters.as_ref()
    }

    pub fn self_liasoned_ty(&self) -> TermRitchieParameterLiasonedType {
        self.self_liasoned_ty
    }

    pub fn nonself_parameter_liasoned_tys(&self) -> &[TermRitchieParameterLiasonedType] {
        self.nonself_parameter_liasoned_tys.as_ref()
    }

    pub fn return_ty(&self) -> Term {
        self.return_ty
    }

    pub fn where_clause(&self) -> () {
        self.where_clause
    }
}