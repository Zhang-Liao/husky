mod context_mut;
mod map;
mod ordered_map;
mod region;

pub use self::context_mut::*;
pub use self::map::*;
pub use self::ordered_map::*;
pub use self::region::*;

use crate::*;
use husky_entity_tree::ModuleSymbolContext;
use husky_term_prelude::symbol::SymbolName;
use idx_arena::{map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};
use parsec::{IsStreamParser, TryParseFromStream};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    PrincipalEntity(PrincipalEntityPath),
    Inherited(InheritedSymbolicVariableIdx, InheritedVariableKind),
    Current(CurrentVariableIdx, CurrentVariableKind),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TemplateVariable {
    Lifetime {
        label_token: LifetimeLabelRegionalToken,
    },
    Type {
        ident_token: IdentRegionalToken,
    },
    Const {},
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InheritedVariable {
    modifier: VariableModifier,
    kind: InheritedVariableKind,
}

impl InheritedVariable {
    pub fn kind(&self) -> InheritedVariableKind {
        self.kind
    }

    pub fn name(&self) -> SymbolName {
        match self.kind {
            InheritedVariableKind::Template(
                InheritedTemplateVariable::Lifetime { label, .. }
                | InheritedTemplateVariable::Place { label, .. },
            ) => label.into(),
            InheritedVariableKind::Template(
                InheritedTemplateVariable::Type { ident }
                | InheritedTemplateVariable::Constant { ident },
            )
            | InheritedVariableKind::Parenate { ident }
            | InheritedVariableKind::SelfField { ident } => ident.into(),
        }
    }

    pub fn ident(&self) -> Option<Ident> {
        self.name().ident()
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InheritedVariableKind {
    Template(InheritedTemplateVariable),
    Parenate { ident: Ident },
    SelfField { ident: Ident },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InheritedTemplateVariable {
    Lifetime { label: Label },
    Place { label: Label },
    Type { ident: Ident },
    Constant { ident: Ident },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct CurrentVariableEntry {
    modifier: VariableModifier,
    access_start: RegionalTokenIdx,
    /// this is none for template, parenate, lambda and field variable
    access_end: Option<RegionalTokenIdxRangeEnd>,
    data: CurrentVariableData,
}

impl CurrentVariableEntry {
    pub fn new(
        pattern_expr_region: &SynPatternExprRegion,
        access_start: RegionalTokenIdx,
        access_end: Option<RegionalTokenIdxRangeEnd>,
        variant: CurrentVariableData,
    ) -> Self {
        Self {
            modifier: variant.svar_modifier(pattern_expr_region),
            access_start,
            access_end,
            data: variant,
        }
    }
}

/// # getters
impl CurrentVariableEntry {
    pub fn modifier(&self) -> VariableModifier {
        self.modifier
    }

    pub fn kind(&self) -> CurrentVariableKind {
        self.data.kind()
    }

    pub fn data(&self) -> &CurrentVariableData {
        &self.data
    }

    pub fn name(&self) -> SymbolName {
        match self.data {
            CurrentVariableData::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. }
                    | CurrentTemplateParameterSynSymbolVariant::Constant { ident_token, .. },
                ..
            }
            | CurrentVariableData::VariadicParenateParameter { ident_token, .. }
            | CurrentVariableData::FieldVariable { ident_token } => ident_token.ident().into(),
            CurrentVariableData::SimpleParenateParameter { ident, .. }
            | CurrentVariableData::LetVariable { ident, .. }
            | CurrentVariableData::BeVariable { ident, .. }
            | CurrentVariableData::CaseVariable { ident, .. }
            | CurrentVariableData::LoopVariable { ident, .. }
            | CurrentVariableData::SimpleClosureParameter { ident, .. } => ident.into(),
            CurrentVariableData::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Lifetime { label_token, .. },
                ..
            } => label_token.label().into(),
            CurrentVariableData::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Place { label_token, .. },
                ..
            } => label_token.label().into(),
            CurrentVariableData::SelfType => SymbolName::SelfType,
            CurrentVariableData::SelfValue { .. } => SymbolName::SelfValue,
        }
    }

    pub fn ident(&self) -> Option<Ident> {
        match self.data {
            CurrentVariableData::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. }
                    | CurrentTemplateParameterSynSymbolVariant::Constant { ident_token, .. },
                ..
            }
            | CurrentVariableData::VariadicParenateParameter { ident_token, .. }
            | CurrentVariableData::FieldVariable { ident_token } => Some(ident_token.ident()),
            CurrentVariableData::SimpleParenateParameter { ident, .. }
            | CurrentVariableData::SimpleClosureParameter { ident, .. }
            | CurrentVariableData::LetVariable { ident, .. }
            | CurrentVariableData::BeVariable { ident, .. }
            | CurrentVariableData::CaseVariable { ident, .. }
            | CurrentVariableData::LoopVariable { ident, .. } => Some(ident),
            CurrentVariableData::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Lifetime { .. }
                    | CurrentTemplateParameterSynSymbolVariant::Place { .. },
                ..
            } => None,
            CurrentVariableData::SelfType | CurrentVariableData::SelfValue { .. } => None,
        }
    }

    pub fn label(&self) -> Option<Label> {
        todo!()
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentVariableKind {
    TemplateParameter {
        template_parameter_kind: CurrentTemplateParameterSynSymbolKind,
    },
    SimpleParenateParameter {
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    VariadicParenateParameter {
        ident_token: IdentRegionalToken,
    },
    SimpleClosureParameter {
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    LetVariable {
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    BeVariable {
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    CaseVariable {
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    FieldVariable {
        ident_token: IdentRegionalToken,
    },
    LoopVariable(SynExprIdx),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentTemplateParameterSynSymbolKind {
    Type {
        ident_token: IdentRegionalToken,
    },
    Lifetime {
        label_token: LifetimeLabelRegionalToken,
    },
    Place {
        label_token: PlaceLabelRegionalToken,
    },
    Constant {
        ident_token: IdentRegionalToken,
    },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum CurrentVariableData {
    TemplateParameter {
        syn_attrs: TemplateParameterSynAttrs,
        annotated_variance_token: Option<VarianceRegionalToken>,
        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant,
    },
    SelfType,
    SelfValue {
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
    },
    SimpleParenateParameter {
        ident: Ident,
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    VariadicParenateParameter {
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
    },
    SimpleClosureParameter {
        ident: Ident,
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    LetVariable {
        ident: Ident,
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    BeVariable {
        ident: Ident,
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    CaseVariable {
        ident: Ident,
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    FieldVariable {
        ident_token: IdentRegionalToken,
    },
    LoopVariable {
        ident: Ident,
        expr_idx: SynExprIdx,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct TemplateParameterSynAttrs {
    syn_attrs: SmallVec<[TemplateSymbolSynAttr; 1]>,
}

impl std::ops::Deref for TemplateParameterSynAttrs {
    type Target = [TemplateSymbolSynAttr];

    fn deref(&self) -> &Self::Target {
        &self.syn_attrs
    }
}

impl<'a, C> TryParseFromStream<SynExprParser<'a, C>> for TemplateParameterSynAttrs
where
    C: IsSynExprContext<'a>,
{
    type Error = SynExprError;

    fn try_parse_from_stream(sp: &mut SynExprParser<'a, C>) -> Result<Self, Self::Error> {
        let mut syn_attrs: SmallVec<[TemplateSymbolSynAttr; 1]> = smallvec::smallvec![];
        while let Some(pound) = sp.try_parse_option::<PoundRegionalToken>()? {
            if let Some(attr_token) = sp.try_parse_option::<AttrRegionalToken>()? {
                let syn_attr = match attr_token {
                    AttrRegionalToken::Phantom(phantom_token) => {
                        TemplateSymbolSynAttr::Phantom(pound, phantom_token)
                    }
                    AttrRegionalToken::Runtime(runtime_token) => {
                        TemplateSymbolSynAttr::Runtime(pound, runtime_token)
                    }
                };
                syn_attrs.push(syn_attr)
            };
        }
        Ok(TemplateParameterSynAttrs { syn_attrs })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TemplateSymbolSynAttr {
    Phantom(PoundRegionalToken, PhantomRegionalToken),
    Runtime(PoundRegionalToken, RuntimeRegionalToken),
}

impl CurrentVariableData {
    fn svar_modifier(&self, pattern_expr_region: &SynPatternExprRegion) -> VariableModifier {
        match self {
            CurrentVariableData::TemplateParameter { .. } => VariableModifier::Const,
            CurrentVariableData::SimpleParenateParameter {
                pattern_symbol_idx, ..
            }
            | CurrentVariableData::SimpleClosureParameter {
                pattern_symbol_idx, ..
            }
            | CurrentVariableData::LetVariable {
                pattern_symbol_idx, ..
            }
            | CurrentVariableData::BeVariable {
                pattern_symbol_idx, ..
            }
            | CurrentVariableData::CaseVariable {
                pattern_symbol_idx, ..
            } => pattern_expr_region.pattern_symbol_modifier(*pattern_symbol_idx),
            CurrentVariableData::VariadicParenateParameter {
                symbol_modifier_keyword_group,
                ..
            } => VariableModifier::new(*symbol_modifier_keyword_group),
            CurrentVariableData::LoopVariable { .. } => VariableModifier::Pure,
            CurrentVariableData::SelfType => VariableModifier::Const,
            CurrentVariableData::SelfValue {
                symbol_modifier_keyword_group,
            } => VariableModifier::new(*symbol_modifier_keyword_group),
            CurrentVariableData::FieldVariable { .. } => VariableModifier::Pure,
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CurrentTemplateParameterSynSymbolVariant {
    Lifetime {
        label_token: LifetimeLabelRegionalToken,
    },
    Place {
        label_token: PlaceLabelRegionalToken,
    },
    Type {
        ident_token: IdentRegionalToken,
    },
    Constant {
        ident_token: IdentRegionalToken,
        ty_expr_idx: SynExprIdx,
    },
}

impl CurrentTemplateParameterSynSymbolVariant {
    fn bequeath(&self) -> InheritedTemplateVariable {
        match self {
            CurrentTemplateParameterSynSymbolVariant::Lifetime { label_token } => {
                InheritedTemplateVariable::Lifetime {
                    label: label_token.label(),
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Place { label_token } => {
                InheritedTemplateVariable::Place {
                    label: label_token.label(),
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. } => {
                InheritedTemplateVariable::Type {
                    ident: ident_token.ident(),
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Constant { ident_token, .. } => {
                InheritedTemplateVariable::Constant {
                    ident: ident_token.ident(),
                }
            }
        }
    }
}

impl CurrentVariableData {
    pub fn kind(&self) -> CurrentVariableKind {
        match self {
            CurrentVariableData::TemplateParameter {
                template_parameter_variant,
                ..
            } => CurrentVariableKind::TemplateParameter {
                template_parameter_kind: template_parameter_variant.kind(),
            },
            CurrentVariableData::SimpleParenateParameter {
                pattern_symbol_idx, ..
            } => CurrentVariableKind::SimpleParenateParameter {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentVariableData::SimpleClosureParameter {
                ident,
                pattern_symbol_idx,
            } => CurrentVariableKind::SimpleClosureParameter {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentVariableData::LetVariable {
                pattern_symbol_idx, ..
            } => CurrentVariableKind::LetVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentVariableData::BeVariable {
                pattern_symbol_idx, ..
            } => CurrentVariableKind::BeVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentVariableData::CaseVariable {
                pattern_symbol_idx, ..
            } => CurrentVariableKind::CaseVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentVariableData::LoopVariable { expr_idx, .. } => {
                CurrentVariableKind::LoopVariable(*expr_idx)
            }
            CurrentVariableData::VariadicParenateParameter { ident_token, .. } => {
                CurrentVariableKind::VariadicParenateParameter {
                    ident_token: *ident_token,
                }
            }
            CurrentVariableData::SelfType => todo!(),
            CurrentVariableData::SelfValue { .. } => todo!(),
            CurrentVariableData::FieldVariable { ident_token } => {
                CurrentVariableKind::FieldVariable {
                    ident_token: *ident_token,
                }
            }
        }
    }
}

impl CurrentTemplateParameterSynSymbolVariant {
    fn kind(&self) -> CurrentTemplateParameterSynSymbolKind {
        match self {
            CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. } => {
                CurrentTemplateParameterSynSymbolKind::Type {
                    ident_token: *ident_token,
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Lifetime { label_token } => {
                CurrentTemplateParameterSynSymbolKind::Lifetime {
                    label_token: *label_token,
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Place { label_token } => {
                CurrentTemplateParameterSynSymbolKind::Place {
                    label_token: *label_token,
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Constant { ident_token, .. } => {
                CurrentTemplateParameterSynSymbolKind::Constant {
                    ident_token: *ident_token,
                }
            }
        }
    }
}

pub type InheritedVariableArena = Arena<InheritedVariable>;
pub type InheritedSymbolicVariableIdx = ArenaIdx<InheritedVariable>;
pub type InheritedSynSymbolIdxRange = ArenaIdxRange<InheritedVariable>;
pub(crate) type InheritedSynSymbolMap<V> = ArenaMap<InheritedVariable, V>;
pub(crate) type InheritedSynSymbolOrderedMap<V> = ArenaOrderedMap<InheritedVariable, V>;

pub type CurrentVariableArena = Arena<CurrentVariableEntry>;
pub type CurrentVariableIdx = ArenaIdx<CurrentVariableEntry>;
pub type CurrentSynSymbolIdxRange = ArenaIdxRange<CurrentVariableEntry>;
pub(crate) type CurrentSynSymbolMap<V> = ArenaMap<CurrentVariableEntry, V>;
pub(crate) type CurrentSynSymbolOrderedMap<V> = ArenaOrderedMap<CurrentVariableEntry, V>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentVariableIdx {
    Inherited(InheritedSymbolicVariableIdx),
    Current(CurrentVariableIdx),
}

impl From<InheritedSymbolicVariableIdx> for ParentVariableIdx {
    fn from(v: InheritedSymbolicVariableIdx) -> Self {
        Self::Inherited(v)
    }
}

impl From<CurrentVariableIdx> for ParentVariableIdx {
    fn from(v: CurrentVariableIdx) -> Self {
        Self::Current(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VariableMap<V> {
    inherited_syn_symbol_map: InheritedSynSymbolMap<V>,
    current_syn_symbol_map: CurrentSynSymbolMap<V>,
}

impl<V> VariableMap<V> {
    pub fn push_inherited(&mut self, inherited_syn_symbol_idx: InheritedSymbolicVariableIdx, v: V) {
        self.inherited_syn_symbol_map
            .insert_new(inherited_syn_symbol_idx, v)
    }

    pub fn push_current(&mut self, current_syn_symbol_idx: CurrentVariableIdx, v: V) {
        self.current_syn_symbol_map
            .insert_new(current_syn_symbol_idx, v)
    }

    pub fn get_inherited(
        &self,
        inherited_syn_symbol_idx: InheritedSymbolicVariableIdx,
    ) -> Option<&V> {
        self.inherited_syn_symbol_map.get(inherited_syn_symbol_idx)
    }

    pub fn get_current(&self, current_syn_symbol_idx: CurrentVariableIdx) -> Option<&V> {
        self.current_syn_symbol_map.get(current_syn_symbol_idx)
    }
}

impl<V> VariableMap<V> {
    pub fn new(syn_symbol_region: &VariableRegionData) -> Self {
        Self {
            inherited_syn_symbol_map: InheritedSynSymbolMap::new(
                syn_symbol_region.inherited_syn_symbol_arena(),
            ),
            current_syn_symbol_map: CurrentSynSymbolMap::new(
                syn_symbol_region.current_syn_symbol_arena(),
            ),
        }
    }
}

impl<V> std::ops::Index<InheritedSymbolicVariableIdx> for VariableMap<V> {
    type Output = V;

    fn index(&self, idx: InheritedSymbolicVariableIdx) -> &Self::Output {
        &self.inherited_syn_symbol_map[idx]
    }
}

impl<V> std::ops::Index<CurrentVariableIdx> for VariableMap<V> {
    type Output = V;

    fn index(&self, idx: CurrentVariableIdx) -> &Self::Output {
        &self.current_syn_symbol_map[idx]
    }
}

pub struct SynSymbolOrderedMap<V> {
    inherited_syn_symbol_ordered_map: InheritedSynSymbolOrderedMap<V>,
    current_syn_symbol_ordered_map: CurrentSynSymbolOrderedMap<V>,
}

impl<V> SynSymbolOrderedMap<V> {
    pub fn push_inherited(&mut self, inherited_syn_symbol_idx: InheritedSymbolicVariableIdx, v: V) {
        self.inherited_syn_symbol_ordered_map
            .insert_next(inherited_syn_symbol_idx, v)
    }

    pub fn push_current(&mut self, current_syn_symbol_idx: CurrentVariableIdx, v: V) {
        self.current_syn_symbol_ordered_map
            .insert_next(current_syn_symbol_idx, v)
    }
}

impl<V> Default for SynSymbolOrderedMap<V> {
    fn default() -> Self {
        Self {
            inherited_syn_symbol_ordered_map: Default::default(),
            current_syn_symbol_ordered_map: Default::default(),
        }
    }
}

impl<V> std::ops::Index<InheritedSymbolicVariableIdx> for SynSymbolOrderedMap<V> {
    type Output = V;

    fn index(&self, idx: InheritedSymbolicVariableIdx) -> &Self::Output {
        &self.inherited_syn_symbol_ordered_map[idx]
    }
}

impl<V> std::ops::Index<CurrentVariableIdx> for SynSymbolOrderedMap<V> {
    type Output = V;

    fn index(&self, idx: CurrentVariableIdx) -> &Self::Output {
        &self.current_syn_symbol_ordered_map[idx]
    }
}
