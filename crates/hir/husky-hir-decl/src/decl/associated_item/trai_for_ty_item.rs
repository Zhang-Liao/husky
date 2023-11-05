mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

use husky_syn_decl::TraitForTypeItemSynDecl;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum TraitForTypeItemHirDecl {
    AssociatedFn(TraitForTypeAssociatedFnHirDecl),
    AssociatedType(TraitForTypeAssociatedTypeHirDecl),
    AssociatedVal(TraitForTypeAssociatedValHirDecl),
    MethodFn(TraitForTypeMethodFnHirDecl),
}

impl TraitForTypeItemHirDecl {
    pub fn path(self, db: &dyn HirDeclDb) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemHirDecl::AssociatedFn(decl) => decl.path(db),
            TraitForTypeItemHirDecl::MethodFn(decl) => decl.path(db),
            TraitForTypeItemHirDecl::AssociatedType(decl) => decl.path(db),
            TraitForTypeItemHirDecl::AssociatedVal(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [HirTemplateParameter] {
        match self {
            TraitForTypeItemHirDecl::AssociatedFn(decl) => decl.template_parameters(db),
            TraitForTypeItemHirDecl::MethodFn(decl) => decl.template_parameters(db),
            TraitForTypeItemHirDecl::AssociatedType(decl) => decl.template_parameters(db),
            TraitForTypeItemHirDecl::AssociatedVal(_) => &[],
        }
    }

    // pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
    //     match self {
    //         TraitForTypeItemHirDecl::AssociatedFn(decl) => decl.hir_expr_region(db).into(),
    //         TraitForTypeItemHirDecl::MethodFn(decl) => decl.hir_expr_region(db).into(),
    //         TraitForTypeItemHirDecl::AssociatedType(decl) => decl.hir_expr_region(db).into(),
    //         TraitForTypeItemHirDecl::AssociatedVal(decl) => decl.hir_expr_region(db).into(),
    //     }
    // }
}

impl HasHirDecl for TraitForTypeItemPath {
    type HirDecl = TraitForTypeItemHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        trai_for_ty_item_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn trai_for_ty_item_hir_decl(
    db: &dyn HirDeclDb,
    path: TraitForTypeItemPath,
) -> Option<TraitForTypeItemHirDecl> {
    match path.syn_decl(db).expect("ok") {
        TraitForTypeItemSynDecl::AssociatedFn(syn_decl) => {
            Some(TraitForTypeAssociatedFnHirDecl::from_syn(path, syn_decl, db).into())
        }
        TraitForTypeItemSynDecl::MethodFn(method_decl) => {
            Some(TraitForTypeMethodFnHirDecl::from_syn(path, method_decl, db).into())
        }
        TraitForTypeItemSynDecl::AssociatedType(assoc_type_decl) => {
            Some(TraitForTypeAssociatedTypeHirDecl::from_syn(path, assoc_type_decl, db).into())
        }
        TraitForTypeItemSynDecl::AssociatedVal(assoc_val_decl) => {
            Some(TraitForTypeAssociatedValHirDecl::from_syn(path, assoc_val_decl, db).into())
        }
    }
}