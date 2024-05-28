use crate::*;

pub trait SynDeclDb {
    fn syn_node_decl_sheet(&self, module_path: ModulePath) -> SynNodeDeclSheet;

    fn syn_decl_sheet(&self, module_path: ModulePath) -> SynDeclSheet;
}

impl SynDeclDb for ::salsa::Db {
    fn syn_node_decl_sheet(&self, module_path: ModulePath) -> SynNodeDeclSheet {
        syn_node_decl_sheet(self, module_path)
    }

    fn syn_decl_sheet(&self, module_path: ModulePath) -> SynDeclSheet {
        syn_decl_sheet(self, module_path)
    }
}

#[salsa::jar]
pub struct SynDeclJar(
    // decl
    // - submodule
    crate::decl::submodule::SubmoduleSynNodeDecl,
    crate::decl::submodule::submodule_syn_node_decl,
    crate::decl::submodule::SubmoduleSynDecl,
    crate::decl::submodule::submodule_decl,
    // - type
    crate::decl::major_item::ty::ty_node_decl,
    crate::decl::major_item::ty::ty_decl,
    crate::decl::major_item::ty::r#enum::EnumSynNodeDecl,
    crate::decl::major_item::ty::r#enum::EnumSynDecl,
    crate::decl::major_item::ty::unit_struct::UnitStructSynNodeDecl,
    crate::decl::major_item::ty::unit_struct::UnitStructSynDecl,
    crate::decl::major_item::ty::tuple_struct::TupleStructSynNodeDecl,
    crate::decl::major_item::ty::tuple_struct::TupleStructSynDecl,
    crate::decl::major_item::ty::props_struct::PropsStructSynNodeDecl,
    crate::decl::major_item::ty::props_struct::PropsStructSynDecl,
    crate::decl::major_item::ty::inductive::InductiveSynNodeDecl,
    crate::decl::major_item::ty::inductive::InductiveSynDecl,
    crate::decl::major_item::ty::structure::StructureSynNodeDecl,
    crate::decl::major_item::ty::structure::StructureSynDecl,
    crate::decl::major_item::ty::r#extern::ExternSynNodeDecl,
    crate::decl::major_item::ty::r#extern::ExternSynDecl,
    crate::decl::major_item::ty::r#union::UnionSynNodeDecl,
    crate::decl::major_item::ty::r#union::UnionSynDecl,
    // - trait
    crate::decl::major_item::trai::TraitSynNodeDecl,
    crate::decl::major_item::trai::trai_syn_node_decl,
    crate::decl::major_item::trai::TraitSynDecl,
    crate::decl::major_item::trai::trai_syn_decl,
    // - form
    crate::decl::major_item::form::form_syn_node_decl,
    crate::decl::major_item::form::form_syn_decl,
    crate::decl::major_item::form::val::MajorValSynNodeDecl,
    crate::decl::major_item::form::val::MajorValSynDecl,
    crate::decl::major_item::form::compterm::MajorConstSynNodeDecl,
    crate::decl::major_item::form::compterm::MajorComptermSynDecl,
    crate::decl::major_item::form::r#static::MajorStaticSynNodeDecl,
    crate::decl::major_item::form::r#static::MajorStaticSynDecl,
    crate::decl::major_item::form::function_ritchie::MajorFunctionRitchieSynNodeDecl,
    crate::decl::major_item::form::function_ritchie::MajorFunctionRitchieSynDecl,
    crate::decl::major_item::form::ty_alias::TypeAliasSynNodeDecl,
    crate::decl::major_item::form::ty_alias::TypeAliasSynDecl,
    // - impl block
    crate::decl::impl_block::ty_impl_block::TypeImplBlockSynNodeDecl,
    crate::decl::impl_block::ty_impl_block::ty_impl_block_syn_node_decl,
    crate::decl::impl_block::ty_impl_block::TypeImplBlockSynDecl,
    crate::decl::impl_block::ty_impl_block::ty_impl_block_syn_decl,
    crate::decl::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockSynNodeDecl,
    crate::decl::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_syn_node_decl,
    crate::decl::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockSynDecl,
    crate::decl::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_syn_decl,
    crate::decl::impl_block::ill_formed::IllFormedImplBlockSynNodeDecl,
    crate::decl::impl_block::ill_formed::ill_formed_impl_block_syn_node_decl,
    // - variant
    crate::decl::ty_variant::ty_variant_syn_node_decl,
    crate::decl::ty_variant::ty_variant_syn_decl,
    crate::decl::ty_variant::unit_ty_variant::TypeUnitVariantSynNodeDecl,
    crate::decl::ty_variant::unit_ty_variant::TypeUnitVariantSynDecl,
    crate::decl::ty_variant::props_ty_variant::TypePropsVariantSynNodeDecl,
    crate::decl::ty_variant::props_ty_variant::TypePropsVariantSynDecl,
    crate::decl::ty_variant::tuple_ty_variant::TypeTupleVariantSynNodeDecl,
    crate::decl::ty_variant::tuple_ty_variant::TypeTupleVariantSynDecl,
    // - associated items
    // -- type item
    crate::decl::assoc_item::ty_item::ty_item_syn_node_decl,
    crate::decl::assoc_item::ty_item::ty_item_syn_decl,
    crate::decl::assoc_item::ty_item::assoc_ritchie::TypeAssocRitchieSynNodeDecl,
    crate::decl::assoc_item::ty_item::assoc_ritchie::TypeAssocRitchieSynDecl,
    crate::decl::assoc_item::ty_item::method_ritchie::TypeMethodRitchieSynNodeDecl,
    crate::decl::assoc_item::ty_item::method_ritchie::TypeMethodRitchieSynDecl,
    crate::decl::assoc_item::ty_item::assoc_ty::TypeAssocTypeSynNodeDecl,
    crate::decl::assoc_item::ty_item::assoc_ty::TypeAssocTypeSynDecl,
    crate::decl::assoc_item::ty_item::assoc_val::TypeAssocValSynNodeDecl,
    crate::decl::assoc_item::ty_item::assoc_val::TypeAssocValSynDecl,
    crate::decl::assoc_item::ty_item::memo_field::TypeMemoizedFieldSynNodeDecl,
    crate::decl::assoc_item::ty_item::memo_field::TypeMemoizedFieldSynDecl,
    // -- trait item
    crate::decl::assoc_item::trai_item::trai_item_syn_node_decl,
    crate::decl::assoc_item::trai_item::trai_item_syn_decl,
    crate::decl::assoc_item::trai_item::assoc_ritchie::TraitAssocRitchieSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_ritchie::TraitAssocRitchieSynDecl,
    crate::decl::assoc_item::trai_item::method_ritchie::TraitMethodRitchieSynNodeDecl,
    crate::decl::assoc_item::trai_item::method_ritchie::TraitMethodRitchieSynDecl,
    crate::decl::assoc_item::trai_item::memo_field::TraitMemoizedFieldSynNodeDecl,
    crate::decl::assoc_item::trai_item::memo_field::TraitMemoizedFieldSynDecl,
    crate::decl::assoc_item::trai_item::assoc_static::TraitAssocStaticSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_static::TraitAssocStaticSynDecl,
    crate::decl::assoc_item::trai_item::assoc_ty::TraitAssocTypeSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_ty::TraitAssocTypeSynDecl,
    crate::decl::assoc_item::trai_item::assoc_val::TraitAssocValSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_val::TraitAssocValSynDecl,
    // -- trait for type item
    crate::decl::assoc_item::trai_for_ty_item::trai_for_ty_item_syn_decl,
    crate::decl::assoc_item::trai_for_ty_item::trai_for_ty_item_syn_node_decl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_ritchie::TraitForTypeAssocRitchieSynNodeDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_ritchie::TraitForTypeAssocRitchieSynDecl,
    crate::decl::assoc_item::trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieSynNodeDecl,
    crate::decl::assoc_item::trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieSynDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_ty::TraitForTypeAssocTypeSynNodeDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_ty::TraitForTypeAssocTypeSynDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_val::TraitForTypeAssocValSynNodeDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_val::TraitForTypeAssocValSynDecl,
    // -- ill formed item
    crate::decl::assoc_item::ill_formed_item::IllFormedItemSynNodeDecl,
    // attr
    crate::decl::attr::derive::DeriveAttrSynNodeDecl,
    crate::decl::attr::derive::DeriveAttrSynDecl,
    crate::decl::attr::backprop::BackpropAttrSynNodeDecl,
    crate::decl::attr::backprop::BackpropAttrSynDecl,
    crate::decl::attr::affect::AffectAttrSynNodeDecl,
    crate::decl::attr::affect::AffectAttrSynDecl,
    crate::decl::attr::marker::MarkerAttrSynNodeDecl,
    crate::decl::attr::marker::MarkerAttrSynDecl,
    crate::decl::attr::task::TaskAttrSynNodeDecl,
    crate::decl::attr::task::TaskAttrSynDecl,
    crate::decl::attr::test::TestAttrSynNodeDecl,
    crate::decl::attr::test::TestAttrSynDecl,
    crate::decl::attr::attr_syn_node_decl,
    crate::decl::attr::attr_syn_decl,
    // sheet
    crate::sheet::SynNodeDeclSheet,
    crate::sheet::syn_node_decl_sheet,
    crate::sheet::SynDeclSheet,
    crate::sheet::syn_decl_sheet,
);
