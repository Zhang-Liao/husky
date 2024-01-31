use husky_entity_tree::*;
use husky_syn_expr::*;
use husky_vfs::Toolchain;

use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SymbolDeclarativeTermRegion {
    symbol_registry: TermSymbolRegistry,
    symbol_signatures: SymbolOrderedMap<SymbolSignature>,
    self_ty: Option<DeclarativeTerm>,
    self_value: Option<SymbolDeclarativeTerm>,
    self_lifetime: Option<SymbolDeclarativeTerm>,
    self_place: Option<SymbolDeclarativeTerm>,
    implicit_template_parameter_symbols: SmallVec<[SymbolDeclarativeTerm; 1]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SymbolSignature {
    kind: SymbolSignatureKind,
    term_symbol: Option<SymbolDeclarativeTerm>,
    modifier: SymbolModifier,
    ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolSignatureKind {
    TemplateParameter,
    ParenateParameter,
    FieldVariable,
}

impl SymbolSignature {
    pub fn kind(self) -> SymbolSignatureKind {
        self.kind
    }

    pub fn term_symbol(self) -> Option<SymbolDeclarativeTerm> {
        self.term_symbol
    }

    pub fn modifier(&self) -> SymbolModifier {
        self.modifier
    }

    pub fn ty(&self) -> DeclarativeTermSymbolTypeResult<DeclarativeTerm> {
        self.ty
    }
}

impl SymbolDeclarativeTermRegion {
    pub fn self_lifetime(&self) -> Option<SymbolDeclarativeTerm> {
        self.self_lifetime
    }

    pub fn self_place(&self) -> Option<SymbolDeclarativeTerm> {
        self.self_place
    }

    pub fn implicit_template_parameter_symbols(&self) -> &[SymbolDeclarativeTerm] {
        &self.implicit_template_parameter_symbols
    }

    pub(crate) fn symbol_registry_mut(&mut self) -> &mut TermSymbolRegistry {
        &mut self.symbol_registry
    }

    #[inline(always)]
    pub(crate) fn add_new_template_parameter_symbol_signature(
        &mut self,
        db: &::salsa::Db,
        idx: CurrentSynSymbolIdx,
        ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
        term_symbol: SymbolDeclarativeTerm,
    ) {
        self.add_new_current_syn_symbol_signature(
            db,
            idx,
            SymbolSignature {
                kind: SymbolSignatureKind::TemplateParameter,
                term_symbol: Some(term_symbol),
                ty,
                modifier: SymbolModifier::Const,
            },
        )
    }

    #[inline(always)]
    pub(crate) fn add_new_parenate_parameter_symbol_signature(
        &mut self,
        db: &::salsa::Db,
        current_syn_symbol: CurrentSynSymbolIdx,
        modifier: SymbolModifier,
        ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    ) {
        let symbol = match modifier {
            SymbolModifier::Const => todo!(),
            _ => None,
        };
        self.add_new_current_syn_symbol_signature(
            db,
            current_syn_symbol,
            SymbolSignature {
                kind: SymbolSignatureKind::ParenateParameter,
                modifier,
                ty,
                term_symbol: symbol,
            },
        )
    }

    #[inline(always)]
    pub(crate) fn add_new_field_variable_symbol_signature(
        &mut self,
        db: &::salsa::Db,
        current_syn_symbol: CurrentSynSymbolIdx,
        ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    ) {
        self.add_new_current_syn_symbol_signature(
            db,
            current_syn_symbol,
            SymbolSignature {
                kind: SymbolSignatureKind::FieldVariable,
                modifier: SymbolModifier::Pure,
                ty,
                // ad hoc
                term_symbol: None,
            },
        )
    }

    #[inline(always)]
    fn add_new_current_syn_symbol_signature(
        &mut self,
        db: &::salsa::Db,
        idx: CurrentSynSymbolIdx,
        signature: SymbolSignature,
    ) {
        self.symbol_signatures.insert_next(idx, signature)
    }
}

impl SymbolDeclarativeTermRegion {
    /// will initialize `inherited_syn_symbol_terms`;
    /// but will leave current_syn_symbol_terms unintialized;
    /// `self_ty_term` is set to that of parent if parent exists, otherwise none;
    /// `self_value_term` is set to that of parent if parent exists, otherwise none
    pub(crate) fn new(
        parent: Option<&SymbolDeclarativeTermRegion>,
        syn_expr_region_data: &SynExprRegionData,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> Self {
        let registry = parent.map_or(Default::default(), |parent| parent.symbol_registry.clone());
        let implicit_self_lifetime = syn_expr_region_data
            .has_self_lifetime()
            .then_some(declarative_term_menu.implicit_self_lifetime());
        let implicit_self_place = syn_expr_region_data
            .has_self_place()
            .then_some(declarative_term_menu.implicit_self_place());
        Self {
            symbol_registry: registry,
            symbol_signatures: SymbolOrderedMap::new(
                parent.map(|parent| &parent.symbol_signatures),
            ),
            self_ty: parent.map(|parent| parent.self_ty).flatten(),
            self_value: parent.map(|parent| parent.self_value).flatten(),
            self_lifetime: implicit_self_lifetime,
            self_place: implicit_self_place,
            implicit_template_parameter_symbols: implicit_self_lifetime
                .into_iter()
                .chain(implicit_self_place)
                .collect(),
        }
    }

    pub(crate) fn infer_self_ty_parameter_and_self_value_parameter(
        &mut self,
        db: &::salsa::Db,
        toolchain: Toolchain,
        region_path: SynNodeRegionPath,
        symbol_region: &SynSymbolRegionData,
    ) {
        if symbol_region.allow_self_ty().to_bool() && self.self_ty.is_none() {
            self.self_ty = match region_path {
                SynNodeRegionPath::Decl(ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(_),
                )) => Some(self.new_self_ty_symbol(toolchain, db).into()),
                SynNodeRegionPath::Decl(ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(ty_node_path),
                )) => Some(
                    self.ty_defn_self_ty_term(
                        db,
                        ty_node_path
                            .unambiguous_path(db)
                            .expect("should have valid item path"),
                    ),
                ),
                SynNodeRegionPath::Decl(ItemSynNodePath::ImplBlock(syn_node_path)) => {
                    match syn_node_path {
                        ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => {
                            None // reserved for later stage
                        }
                        ImplBlockSynNodePath::TraitForTypeImplBlock(impl_block_path) => {
                            match impl_block_path.ty_sketch(db) {
                                TypeSketch::DeriveAny => {
                                    Some(self.new_self_ty_symbol(toolchain, db).into())
                                }
                                TypeSketch::Path(ty_path) => None, // reserved for later stage
                            }
                        }
                        ImplBlockSynNodePath::IllFormedImplBlock(_) => None,
                    }
                }
                _ => unreachable!(),
            }
        }
        if symbol_region.allow_self_value().to_bool() && self.self_value.is_none() {
            self.self_value = Some(
                SymbolDeclarativeTerm::new_self_value(
                    db,
                    toolchain,
                    &mut self.symbol_registry,
                    self.self_ty.expect("self type should exists"),
                )
                .into(),
            )
        }
    }
    fn new_self_ty_symbol(
        &mut self,
        toolchain: Toolchain,
        db: &::salsa::Db,
    ) -> SymbolDeclarativeTerm {
        let symbol = SymbolDeclarativeTerm::new_self_ty(db, toolchain, &mut self.symbol_registry);
        self.implicit_template_parameter_symbols.push(symbol);
        symbol
    }

    /// this only works on type definitions
    ///
    /// example:
    /// ```husky
    /// enum Animal<T> where
    /// | Dog
    /// | Cat
    /// ```
    ///
    /// then self type term is `Animal T`
    fn ty_defn_self_ty_term(&self, db: &::salsa::Db, ty_path: TypePath) -> DeclarativeTerm {
        let mut self_ty: DeclarativeTerm = EntityPathDeclarativeTerm::Type(ty_path.into()).into();
        for current_syn_symbol_signature in self
            .symbol_signatures
            .current_syn_symbol_map()
            .iter()
            .copied()
        {
            match current_syn_symbol_signature.kind {
                SymbolSignatureKind::TemplateParameter => {
                    let argument = current_syn_symbol_signature
                        .term_symbol()
                        .expect("should have term");
                    self_ty = self_ty.apply(db, argument)
                }
                SymbolSignatureKind::ParenateParameter => unreachable!(),
                SymbolSignatureKind::FieldVariable => break,
            }
        }
        self_ty
    }

    pub fn self_ty(&self) -> Option<DeclarativeTerm> {
        self.self_ty
    }

    pub(crate) fn set_self_ty(&mut self, self_ty: Option<DeclarativeTerm>) {
        debug_assert!(self.self_ty.is_none());
        self.self_ty = self_ty
    }

    pub fn self_value(&self) -> Option<SymbolDeclarativeTerm> {
        self.self_value
    }

    fn parent_symbol_term(&self, parent_symbol_idx: ParentSynSymbolIdx) -> SymbolSignature {
        match parent_symbol_idx {
            ParentSynSymbolIdx::Inherited(inherited_syn_symbol_idx) => {
                self.inherited_syn_symbol_signature(inherited_syn_symbol_idx)
            }
            ParentSynSymbolIdx::Current(current_syn_symbol_idx) => self
                .current_parameter_symbol_signature(current_syn_symbol_idx)
                .expect("should exist"),
        }
    }

    pub fn inherited_syn_symbol_signature(
        &self,
        inherited_syn_symbol_idx: InheritedSynSymbolIdx,
    ) -> SymbolSignature {
        self.symbol_signatures[inherited_syn_symbol_idx]
    }

    /// None for symbols defined in the body
    pub fn current_parameter_symbol_signature(
        &self,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
    ) -> Option<SymbolSignature> {
        self.symbol_signatures
            .current_syn_symbol_map()
            .get(current_syn_symbol_idx.index())
            .copied()
    }
}
