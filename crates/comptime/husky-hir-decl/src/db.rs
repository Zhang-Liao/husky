use crate::*;
use husky_hir_expr::db::*;

pub trait HirDeclDb: salsa::DbWithJar<HirDeclJar> + HirExprDb {}

impl<Db> HirDeclDb for Db where Db: salsa::DbWithJar<HirDeclJar> + HirExprDb {}

#[salsa::jar(db = HirDeclDb)]
pub struct HirDeclJar(
    submodule_hir_decl,
    SubmoduleHirDecl,
    // associated_items
    // - type items
    ty_item_hir_decl,
    TypeMethodFnHirDecl,
    TypeMemoizedFieldHirDecl,
    TypeAssociatedFnHirDecl,
    TypeAssociatedValHirDecl,
    TypeAssociatedTypeHirDecl,
    // - trait items
    TraitAssociatedFnHirDecl,
    TraitAssociatedTypeHirDecl,
    TraitAssociatedValHirDecl,
    TraitMethodFnHirDecl,
    // - trait for type
    trai_for_ty_item_hir_decl,
    TraitForTypeAssociatedFnHirDecl,
    TraitForTypeAssociatedTypeHirDecl,
    TraitForTypeAssociatedValHirDecl,
    TraitForTypeMethodFnHirDecl,
    // ty
    ty_hir_decl,
    EnumTypeHirDecl,
    ExternTypeHirDecl,
    RecordTypeHirDecl,
    PropsStructTypeHirDecl,
    TupleStructTypeHirDecl,
    UnionTypeHirDecl,
    UnitStructTypeHirDecl,
    // trai
    trai_hir_decl,
    TraitHirDecl,
    // fugitive
    fugitive_hir_decl,
    FnFugitiveHirDecl,
    GnFugitiveHirDecl,
    TypeAliasFugitiveHirDecl,
    ValFugitiveHirDecl,
    // ty variant
    ty_variant_hir_decl_with_source_map,
    ty_variant_hir_decl,
    EnumTupleTypeVariantHirDecl,
    EnumPropsTypeVariantHirDecl,
    EnumUnitTypeVariantHirDecl,
    // impl block
    // - type
    ty_impl_block_hir_decl_with_source_map,
    ty_impl_block_hir_decl,
    TypeImplBlockHirDecl,
    // - trait for type
    trai_for_ty_impl_block_hir_decl_with_source_map,
    trai_for_ty_impl_block_hir_decl,
    TraitForTypeImplBlockHirDecl,
    // attr
    DeriveAttrHirDecl,
);
