```rust
[
    SemExprRegion {
        path: RegionPath::ItemDecl(
            ItemPath(`mnist::Task`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDecl(
                ItemPath(`mnist::Task`),
            ),
            place_registry: PlaceRegistry {
                infos: [],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist::task::MnistTask`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [],
                },
            ),
            sem_expr_roots: [
                (
                    0,
                    (
                        SemExprIdx(
                            0,
                        ),
                        SynExprRootKind::TypeVarDefault,
                    ),
                ),
            ],
            syn_pattern_ty_infos: [],
            syn_pattern_variable_ty_infos: ArenaMap {
                data: [],
            },
            sem_expr_terms: [
                (
                    SemExprIdx(
                        0,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`MnistTask`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_variable_map: [],
                current_variable_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_variable_map: [],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsSort(
                                    ExpectSort {
                                        smallest_universe: Universe(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsSort(
                                                Universe(
                                                    1,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: None,
            self_value_ty: None,
            context_itd: EthTermContextItd {
                task_ty: Some(
                    EthTerm(`MnistTask`),
                ),
            },
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDecl(
            ItemPath(`mnist::TASK`),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDecl(
                ItemPath(`mnist::TASK`),
            ),
            place_registry: PlaceRegistry {
                infos: [],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist::task::MnistTask`, `Extern`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MajorItemPathAssocItem {
                                    parent_expr_idx: 3,
                                    parent_path: MajorItemPath::Type(
                                        TypePath(`mnist::task::MnistTask`, `Extern`),
                                    ),
                                    colon_colon_regional_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            13,
                                        ),
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `new`,
                                        regional_token_idx: RegionalTokenIdx(
                                            14,
                                        ),
                                    },
                                    ontology_dispatch: TypeItem {
                                        signature: AssocRitchie(
                                            TypeAssocRitchieFlySignature {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 177,
                                                        },
                                                    ),
                                                ),
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    quary: Some(
                                                        Transient,
                                                    ),
                                                    base: Eth(
                                                        ItemPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 16,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                ty: FlyTerm {
                                                    quary: None,
                                                    base: Eth(
                                                        Ritchie(
                                                            EthRitchie(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: AssocItem(
                                                        TypeItem(
                                                            TypeItemPath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 177,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    context_itd: EthTermContextItd(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    env: AssocRitchie,
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                self_ty: FlyTerm {
                                                    quary: None,
                                                    base: Eth(
                                                        ItemPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 16,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`fn(() -> MnistTask`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(() -> MnistTask`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function: SemExprIdx(
                                        1,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    quary: Some(
                                        Transient,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`MnistTask`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`MnistTask`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [],
                },
            ),
            sem_expr_roots: [
                (
                    0,
                    (
                        SemExprIdx(
                            0,
                        ),
                        SynExprRootKind::ReturnType,
                    ),
                ),
                (
                    2,
                    (
                        SemExprIdx(
                            2,
                        ),
                        SynExprRootKind::StaticExpr,
                    ),
                ),
            ],
            syn_pattern_ty_infos: [],
            syn_pattern_variable_ty_infos: ArenaMap {
                data: [],
            },
            sem_expr_terms: [
                (
                    SemExprIdx(
                        0,
                    ),
                    Ok(
                        FlyTerm {
                            quary: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`MnistTask`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_variable_map: [],
                current_variable_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_variable_map: [],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsSort(
                                    ExpectSort {
                                        smallest_universe: Universe(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Type`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsSort(
                                                Universe(
                                                    1,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::AnyDerived,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(() -> MnistTask`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        quary: Some(
                                                            Transient,
                                                        ),
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`MnistTask`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyDerived(
                                    ExpectAnyDerived,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        quary: Some(
                                            Transient,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`MnistTask`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyDerived(
                                                ExpectAnyDerivedOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: None,
            self_value_ty: None,
            context_itd: EthTermContextItd {
                task_ty: Some(
                    EthTerm(`MnistTask`),
                ),
            },
        },
    },
]
```