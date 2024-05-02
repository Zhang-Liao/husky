use crate::*;

#[salsa::jar]
pub struct HirDefnJar(
    // defn
    // - type
    ty_hir_defn,
    EnumHirDefn,
    crate::defn::enum_hir_defn_dependencies,
    crate::defn::enum_hir_defn_version_stamp,
    UnitStructHirDefn,
    crate::defn::unit_struct_hir_defn_dependencies,
    crate::defn::unit_struct_hir_defn_version_stamp,
    TupleStructHirDefn,
    crate::defn::tuple_struct_hir_defn_dependencies,
    crate::defn::tuple_struct_hir_defn_version_stamp,
    PropsStructHirDefn,
    crate::defn::props_struct_hir_defn_dependencies,
    crate::defn::props_struct_hir_defn_version_stamp,
    ExternHirDefn,
    crate::defn::extern_hir_defn_dependencies,
    crate::defn::extern_hir_defn_version_stamp,
    UnionHirDefn,
    crate::defn::union_hir_defn_dependencies,
    crate::defn::union_hir_defn_version_stamp,
    // - major form
    form_hir_defn,
    MajorValHirDefn,
    crate::defn::val_hir_defn_dependencies,
    crate::defn::val_hir_defn_version_stamp,
    crate::defn::r#const::MajorConstHirDefn,
    crate::defn::r#const::major_const_hir_defn_dependencies,
    crate::defn::r#const::major_const_hir_defn_version_stamp,
    crate::defn::r#static::MajorStaticHirDefn,
    crate::defn::r#static::major_static_hir_defn_dependencies,
    crate::defn::r#static::major_static_hir_defn_version_stamp,
    MajorFunctionRitchieHirDefn,
    crate::defn::major_function_ritchie_hir_defn_dependencies,
    crate::defn::major_function_ritchie_hir_defn_version_stamp,
    // - morphism_defn,
    TypeAliasHirDefn,
    crate::defn::ty_alias_hir_defn_dependencies,
    crate::defn::ty_alias_hir_defn_version_stamp,
    // - type_alias_defn,
    // - trait
    TraitHirDefn,
    trai_hir_defn,
    crate::defn::trai_hir_defn_dependencies,
    crate::defn::trai_hir_defn_version_stamp,
    // - enum variant,
    EnumUnitVariantHirDefn,
    crate::defn::enum_unit_variant_hir_defn_dependencies,
    crate::defn::enum_unit_variant_hir_defn_version_stamp,
    EnumTupleVariantHirDefn,
    crate::defn::enum_tuple_variant_hir_defn_dependencies,
    crate::defn::enum_tuple_variant_hir_defn_version_stamp,
    EnumPropsVariantHirDefn,
    crate::defn::enum_props_variant_hir_defn_dependencies,
    crate::defn::enum_props_variant_hir_defn_version_stamp,
    // - type item
    ty_item_hir_defn,
    TypeAssocRitchieHirDefn,
    crate::defn::ty_assoc_fn_hir_defn_dependencies,
    crate::defn::ty_assoc_fn_hir_defn_version_stamp,
    TypeMethodRitchieHirDefn,
    crate::defn::ty_method_ritchie_hir_defn_dependencies,
    crate::defn::ty_method_ritchie_hir_defn_version_stamp,
    TypeAssocTypeHirDefn,
    crate::defn::ty_assoc_ty_hir_defn_dependencies,
    crate::defn::ty_assoc_ty_hir_defn_version_stamp,
    TypeAssocValHirDefn,
    crate::defn::ty_assoc_val_hir_defn_dependencies,
    crate::defn::ty_assoc_val_hir_defn_version_stamp,
    TypeMemoizedFieldHirDefn,
    crate::defn::ty_memo_field_hir_defn_dependencies,
    crate::defn::ty_memo_field_hir_defn_version_stamp,
    // - trait item
    trai_item_hir_defn,
    TraitAssocRitchieHirDefn,
    crate::defn::trai_assoc_fn_hir_defn_dependencies,
    crate::defn::trai_assoc_fn_hir_defn_version_stamp,
    TraitMethodFnHirDefn,
    crate::defn::trai_method_ritchie_hir_defn_dependencies,
    crate::defn::trai_method_ritchie_hir_defn_version_stamp,
    TraitAssocTypeHirDefn,
    crate::defn::trai_assoc_ty_hir_defn_dependencies,
    crate::defn::trai_assoc_ty_hir_defn_version_stamp,
    TraitAssocValHirDefn,
    crate::defn::trai_assoc_val_hir_defn_dependencies,
    crate::defn::trai_assoc_val_hir_defn_version_stamp,
    // - trait for type item
    trai_for_ty_item_hir_defn,
    TraitForTypeAssocRitchieHirDefn,
    crate::defn::trai_for_ty_assoc_fn_hir_defn_dependencies,
    crate::defn::trai_for_ty_assoc_fn_hir_defn_version_stamp,
    TraitForTypeMethodRitchieHirDefn,
    crate::defn::trai_for_ty_method_ritchie_hir_defn_dependencies,
    crate::defn::trai_for_ty_method_ritchie_hir_defn_version_stamp,
    TraitForTypeAssocTypeHirDefn,
    crate::defn::trai_for_ty_assoc_ty_hir_defn_dependencies,
    crate::defn::trai_for_ty_assoc_ty_hir_defn_version_stamp,
    TraitForTypeAssocValHirDefn,
    crate::defn::trai_for_ty_assoc_val_hir_defn_dependencies,
    crate::defn::trai_for_ty_assoc_val_hir_defn_version_stamp,
    // - impl block
    crate::defn::ty_impl_block_dependencies,
    crate::defn::ty_impl_block_version_stamp,
    crate::defn::trai_for_ty_impl_block_dependencies,
    crate::defn::trai_for_ty_impl_block_version_stamp,
    // dependencies
    crate::dependencies::HirDefnDependencies,
    // version stamp
    crate::version_stamp::HirDefnVersionStamp,
);
