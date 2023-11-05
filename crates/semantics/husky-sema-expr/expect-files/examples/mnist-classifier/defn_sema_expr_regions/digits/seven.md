[
    SemaExprRegion {
        [salsa id]: 88,
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `FermiMatchResult`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: SynPatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [],
                                },
                                pattern_symbol_arena: Arena {
                                    data: [],
                                },
                                pattern_symbol_maps: [],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [],
                                },
                            },
                            symbol_region: SynSymbolRegion {
                                inherited_symbol_arena: Arena {
                                    data: [],
                                },
                                current_symbol_arena: Arena {
                                    data: [],
                                },
                                allow_self_type: False,
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: ReturnType,
                                    syn_expr_idx: 1,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                        },
                    },
                ),
                path: RegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 3,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `FunctionFn`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::List {
                            lbox_regional_token_idx: RegionalTokenIdx(
                                5,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 3,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                7,
                            ),
                        },
                        SynExprData::FunctionApplicationOrCall {
                            function: 1,
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 2,
                                    comma_regional_token_idx: Some(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                },
                                SynCommaListItem {
                                    syn_expr_idx: 4,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                8,
                            ),
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `fermi_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_concave_components`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `simple_leftdown_pattern`,
                                    regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `FunctionFn`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 5,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    SynExprRoot {
                        kind: EvalExpr,
                        syn_expr_idx: 5,
                    },
                    SynExprRoot {
                        kind: BlockExpr,
                        syn_expr_idx: 6,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            Id {
                                value: 52,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Ritchie(
                                            EtherealTermRitchie(
                                                Id {
                                                    value: 14,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 77,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 69,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Ritchie(
                                            EtherealTermRitchie(
                                                Id {
                                                    value: 13,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                NewList {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                3,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 71,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionFnCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyTermRitchieRegularParameter {
                                                contract: None,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        Application(
                                                            EtherealTermApplication(
                                                                Id {
                                                                    value: 69,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    2,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        Regular(
                                            FluffyTermRitchieRegularParameter {
                                                contract: None,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        Application(
                                                            EtherealTermApplication(
                                                                Id {
                                                                    value: 71,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    4,
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 62,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 62,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        5,
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 62,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    6,
                    (
                        SemaExprIdx(
                            6,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: TypeOntology,
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Ritchie(
                                                EtherealTermRitchie(
                                                    Id {
                                                        value: 14,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    template_parameter_substitutions: [],
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: Ethereal(
                                                            EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 62,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    variant: Ritchie {
                                                        ritchie_kind: FnType,
                                                        parameter_contracted_tys: [
                                                            Regular(
                                                                FluffyTermRitchieRegularParameter {
                                                                    contract: None,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Ethereal(
                                                                            Application(
                                                                                EtherealTermApplication(
                                                                                    Id {
                                                                                        value: 69,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            Regular(
                                                                FluffyTermRitchieRegularParameter {
                                                                    contract: None,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Ethereal(
                                                                            Application(
                                                                                EtherealTermApplication(
                                                                                    Id {
                                                                                        value: 71,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 69,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 69,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Ritchie(
                                                    EtherealTermRitchie(
                                                        Id {
                                                            value: 13,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Ritchie(
                                                EtherealTermRitchie(
                                                    Id {
                                                        value: 13,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 71,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 71,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 62,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 62,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 62,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 62,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
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
            return_ty: Some(
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 62,
                            },
                        ),
                    ),
                ),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        [salsa id]: 90,
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `FunctionFn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::Prefix {
                                        opr: Tilde,
                                        opr_regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                        opd: 1,
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 2,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::Prefix {
                                        opr: Option,
                                        opr_regional_token_idx: RegionalTokenIdx(
                                            10,
                                        ),
                                        opd: 3,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `ConcaveComponent`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `f32`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::f32`, `Extern`),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: SynPatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [
                                        SynPatternExpr::Ident {
                                            symbol_modifier_tokens: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `cc`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                            },
                                        },
                                    ],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [
                                        None,
                                    ],
                                },
                                pattern_symbol_arena: Arena {
                                    data: [
                                        SynPatternSymbol::Atom(
                                            1,
                                        ),
                                    ],
                                },
                                pattern_symbol_maps: [
                                    [
                                        (
                                            `cc`,
                                            1,
                                        ),
                                    ],
                                ],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [
                                        None,
                                    ],
                                },
                            },
                            symbol_region: SynSymbolRegion {
                                inherited_symbol_arena: Arena {
                                    data: [],
                                },
                                current_symbol_arena: Arena {
                                    data: [
                                        SynCurrentSymbol {
                                            modifier: None,
                                            access_start: RegionalTokenIdx(
                                                5,
                                            ),
                                            access_end: None,
                                            variant: SynCurrentSymbolVariant::ParenateRegularParameter {
                                                ident: `cc`,
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                    ],
                                },
                                allow_self_type: False,
                                allow_self_value: False,
                                pattern_ty_constraints: [
                                    (
                                        ExplicitRegularParameter {
                                            syn_pattern_root: SynPatternRoot(
                                                1,
                                            ),
                                            ty_expr_idx: 2,
                                        },
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                ],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: ExplicitParameterType,
                                    syn_expr_idx: 2,
                                },
                                SynExprRoot {
                                    kind: ReturnType,
                                    syn_expr_idx: 4,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                        },
                    },
                ),
                path: RegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `FunctionFn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        SynExprData::InheritedSymbol {
                            ident: `cc`,
                            regional_token_idx: RegionalTokenIdx(
                                4,
                            ),
                            inherited_symbol_idx: 1,
                            inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                                ident: `cc`,
                            },
                        },
                        SynExprData::MethodApplicationOrCall {
                            self_argument: 1,
                            dot_regional_token_idx: RegionalTokenIdx(
                                5,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `displacement`,
                                regional_token_idx: RegionalTokenIdx(
                                    6,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                7,
                            ),
                            items: [],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                8,
                            ),
                        },
                        SynExprData::CurrentSymbol {
                            ident: `dp`,
                            regional_token_idx: RegionalTokenIdx(
                                10,
                            ),
                            current_symbol_idx: 1,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 1,
                            },
                        },
                        SynExprData::Field {
                            owner: 3,
                            dot_regional_token_idx: RegionalTokenIdx(
                                11,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `y`,
                                regional_token_idx: RegionalTokenIdx(
                                    12,
                                ),
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                14,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 96,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::Binary {
                            lopd: 4,
                            opr: Comparison(
                                Less,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                13,
                            ),
                            ropd: 5,
                        },
                        SynExprData::CurrentSymbol {
                            ident: `dp`,
                            regional_token_idx: RegionalTokenIdx(
                                16,
                            ),
                            current_symbol_idx: 1,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 1,
                            },
                        },
                        SynExprData::Field {
                            owner: 7,
                            dot_regional_token_idx: RegionalTokenIdx(
                                17,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `y`,
                                regional_token_idx: RegionalTokenIdx(
                                    18,
                                ),
                            },
                        },
                        SynExprData::Prefix {
                            opr: Minus,
                            opr_regional_token_idx: RegionalTokenIdx(
                                15,
                            ),
                            opd: 8,
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                1..4,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    1,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                            ),
                            initial_value: 2,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    9,
                                ),
                            },
                            condition: 6,
                        },
                        SynStmtData::Eval {
                            expr_idx: 9,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                },
                            },
                        ],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [
                            None,
                        ],
                    },
                    pattern_symbol_arena: Arena {
                        data: [
                            SynPatternSymbol::Atom(
                                1,
                            ),
                        ],
                    },
                    pattern_symbol_maps: [
                        [
                            (
                                `dp`,
                                1,
                            ),
                        ],
                    ],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [
                            None,
                        ],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [
                            SynInheritedSymbol {
                                parent_symbol_idx: Current(
                                    1,
                                ),
                                modifier: None,
                                kind: SynInheritedSymbolKind::ParenateParameter {
                                    ident: `cc`,
                                },
                            },
                        ],
                    },
                    current_symbol_arena: Arena {
                        data: [
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    3,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            19,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::LetVariable {
                                    ident: `dp`,
                                    pattern_symbol_idx: 1,
                                },
                            },
                        ],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    SynExprRoot {
                        kind: LetStmtInitialValue,
                        syn_expr_idx: 2,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 6,
                    },
                    SynExprRoot {
                        kind: EvalExpr,
                        syn_expr_idx: 9,
                    },
                    SynExprRoot {
                        kind: BlockExpr,
                        syn_expr_idx: 10,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            Id {
                                value: 53,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 286,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_symbol_idx: 1,
                                    inherited_symbol_kind: ParenateParameter {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 286,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 302,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    Id {
                                                                        value: 53,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    current_symbol_idx: 1,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 276,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                        signature: PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 96,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        4,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ropd: SemaExprIdx(
                                        5,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    current_symbol_idx: 1,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 276,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            18,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                        signature: PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..4,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: SynPatternRoot(
                                            1,
                                        ),
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        6,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    10,
                    (
                        SemaExprIdx(
                            10,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        5,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            0.0,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [
                        None,
                    ],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        None,
                    ],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 70,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 44,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                WrapInSome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 44,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                WrapInSome,
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
            return_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 44,
                        },
                    ),
                ),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        [salsa id]: 92,
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `FermiMatchResult`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: SynPatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [],
                                },
                                pattern_symbol_arena: Arena {
                                    data: [],
                                },
                                pattern_symbol_maps: [],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [],
                                },
                            },
                            symbol_region: SynSymbolRegion {
                                inherited_symbol_arena: Arena {
                                    data: [],
                                },
                                current_symbol_arena: Arena {
                                    data: [],
                                },
                                allow_self_type: False,
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: ReturnType,
                                    syn_expr_idx: 1,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                        },
                    },
                ),
                path: RegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 3,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `FunctionFn`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 4,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `FunctionFn`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::List {
                            lbox_regional_token_idx: RegionalTokenIdx(
                                5,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 3,
                                    comma_regional_token_idx: Some(
                                        RegionalTokenIdx(
                                            7,
                                        ),
                                    ),
                                },
                                SynCommaListItem {
                                    syn_expr_idx: 4,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                9,
                            ),
                        },
                        SynExprData::FunctionApplicationOrCall {
                            function: 1,
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 2,
                                    comma_regional_token_idx: Some(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                },
                                SynCommaListItem {
                                    syn_expr_idx: 5,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                10,
                            ),
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `fermi_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_concave_components`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `leftupcc_pattern`,
                                    regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `FunctionFn`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `leftdowncc_pattern`,
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `FunctionFn`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 6,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    SynExprRoot {
                        kind: EvalExpr,
                        syn_expr_idx: 6,
                    },
                    SynExprRoot {
                        kind: BlockExpr,
                        syn_expr_idx: 7,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            Id {
                                value: 54,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Ritchie(
                                            EtherealTermRitchie(
                                                Id {
                                                    value: 14,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 77,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 69,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 55,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Ritchie(
                                            EtherealTermRitchie(
                                                Id {
                                                    value: 13,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 56,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Ritchie(
                                            EtherealTermRitchie(
                                                Id {
                                                    value: 13,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                NewList {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                3,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    7,
                                                ),
                                            ),
                                        },
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                4,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 71,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionFnCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyTermRitchieRegularParameter {
                                                contract: None,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        Application(
                                                            EtherealTermApplication(
                                                                Id {
                                                                    value: 69,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    2,
                                                ),
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        Regular(
                                            FluffyTermRitchieRegularParameter {
                                                contract: None,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        Application(
                                                            EtherealTermApplication(
                                                                Id {
                                                                    value: 71,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    5,
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 62,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 62,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        6,
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 62,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    7,
                    (
                        SemaExprIdx(
                            7,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: TypeOntology,
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Ritchie(
                                                EtherealTermRitchie(
                                                    Id {
                                                        value: 14,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    template_parameter_substitutions: [],
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: Ethereal(
                                                            EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 62,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    variant: Ritchie {
                                                        ritchie_kind: FnType,
                                                        parameter_contracted_tys: [
                                                            Regular(
                                                                FluffyTermRitchieRegularParameter {
                                                                    contract: None,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Ethereal(
                                                                            Application(
                                                                                EtherealTermApplication(
                                                                                    Id {
                                                                                        value: 69,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            Regular(
                                                                FluffyTermRitchieRegularParameter {
                                                                    contract: None,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Ethereal(
                                                                            Application(
                                                                                EtherealTermApplication(
                                                                                    Id {
                                                                                        value: 71,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 69,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 69,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Ritchie(
                                                    EtherealTermRitchie(
                                                        Id {
                                                            value: 13,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Ritchie(
                                                EtherealTermRitchie(
                                                    Id {
                                                        value: 13,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Ritchie(
                                                    EtherealTermRitchie(
                                                        Id {
                                                            value: 13,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Ritchie(
                                                EtherealTermRitchie(
                                                    Id {
                                                        value: 13,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 71,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 71,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 62,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 62,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 62,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 62,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
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
            return_ty: Some(
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 62,
                            },
                        ),
                    ),
                ),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        [salsa id]: 94,
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `FunctionFn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::Prefix {
                                        opr: Tilde,
                                        opr_regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                        opd: 1,
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 2,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::Prefix {
                                        opr: Option,
                                        opr_regional_token_idx: RegionalTokenIdx(
                                            10,
                                        ),
                                        opd: 3,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `ConcaveComponent`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `f32`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::f32`, `Extern`),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: SynPatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [
                                        SynPatternExpr::Ident {
                                            symbol_modifier_tokens: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `cc`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                            },
                                        },
                                    ],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [
                                        None,
                                    ],
                                },
                                pattern_symbol_arena: Arena {
                                    data: [
                                        SynPatternSymbol::Atom(
                                            1,
                                        ),
                                    ],
                                },
                                pattern_symbol_maps: [
                                    [
                                        (
                                            `cc`,
                                            1,
                                        ),
                                    ],
                                ],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [
                                        None,
                                    ],
                                },
                            },
                            symbol_region: SynSymbolRegion {
                                inherited_symbol_arena: Arena {
                                    data: [],
                                },
                                current_symbol_arena: Arena {
                                    data: [
                                        SynCurrentSymbol {
                                            modifier: None,
                                            access_start: RegionalTokenIdx(
                                                5,
                                            ),
                                            access_end: None,
                                            variant: SynCurrentSymbolVariant::ParenateRegularParameter {
                                                ident: `cc`,
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                    ],
                                },
                                allow_self_type: False,
                                allow_self_value: False,
                                pattern_ty_constraints: [
                                    (
                                        ExplicitRegularParameter {
                                            syn_pattern_root: SynPatternRoot(
                                                1,
                                            ),
                                            ty_expr_idx: 2,
                                        },
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                ],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: ExplicitParameterType,
                                    syn_expr_idx: 2,
                                },
                                SynExprRoot {
                                    kind: ReturnType,
                                    syn_expr_idx: 4,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                        },
                    },
                ),
                path: RegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `FunctionFn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        SynExprData::InheritedSymbol {
                            ident: `cc`,
                            regional_token_idx: RegionalTokenIdx(
                                4,
                            ),
                            inherited_symbol_idx: 1,
                            inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                                ident: `cc`,
                            },
                        },
                        SynExprData::MethodApplicationOrCall {
                            self_argument: 1,
                            dot_regional_token_idx: RegionalTokenIdx(
                                5,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `displacement`,
                                regional_token_idx: RegionalTokenIdx(
                                    6,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                7,
                            ),
                            items: [],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                8,
                            ),
                        },
                        SynExprData::CurrentSymbol {
                            ident: `dp`,
                            regional_token_idx: RegionalTokenIdx(
                                10,
                            ),
                            current_symbol_idx: 1,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 1,
                            },
                        },
                        SynExprData::Field {
                            owner: 3,
                            dot_regional_token_idx: RegionalTokenIdx(
                                11,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `y`,
                                regional_token_idx: RegionalTokenIdx(
                                    12,
                                ),
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                14,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 97,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::Binary {
                            lopd: 4,
                            opr: Comparison(
                                Less,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                13,
                            ),
                            ropd: 5,
                        },
                        SynExprData::InheritedSymbol {
                            ident: `cc`,
                            regional_token_idx: RegionalTokenIdx(
                                16,
                            ),
                            inherited_symbol_idx: 1,
                            inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                                ident: `cc`,
                            },
                        },
                        SynExprData::Field {
                            owner: 7,
                            dot_regional_token_idx: RegionalTokenIdx(
                                17,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `relative_bounding_box`,
                                regional_token_idx: RegionalTokenIdx(
                                    18,
                                ),
                            },
                        },
                        SynExprData::MethodApplicationOrCall {
                            self_argument: 8,
                            dot_regional_token_idx: RegionalTokenIdx(
                                19,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `ymax`,
                                regional_token_idx: RegionalTokenIdx(
                                    20,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                21,
                            ),
                            items: [],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                22,
                            ),
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                24,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 98,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::Binary {
                            lopd: 9,
                            opr: Comparison(
                                Greater,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                23,
                            ),
                            ropd: 10,
                        },
                        SynExprData::InheritedSymbol {
                            ident: `cc`,
                            regional_token_idx: RegionalTokenIdx(
                                25,
                            ),
                            inherited_symbol_idx: 1,
                            inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                                ident: `cc`,
                            },
                        },
                        SynExprData::MethodApplicationOrCall {
                            self_argument: 12,
                            dot_regional_token_idx: RegionalTokenIdx(
                                26,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `end`,
                                regional_token_idx: RegionalTokenIdx(
                                    27,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                28,
                            ),
                            items: [],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                29,
                            ),
                        },
                        SynExprData::Field {
                            owner: 13,
                            dot_regional_token_idx: RegionalTokenIdx(
                                30,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `y`,
                                regional_token_idx: RegionalTokenIdx(
                                    31,
                                ),
                            },
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                1..5,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    1,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                            ),
                            initial_value: 2,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    9,
                                ),
                            },
                            condition: 6,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    15,
                                ),
                            },
                            condition: 11,
                        },
                        SynStmtData::Eval {
                            expr_idx: 14,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                },
                            },
                        ],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [
                            None,
                        ],
                    },
                    pattern_symbol_arena: Arena {
                        data: [
                            SynPatternSymbol::Atom(
                                1,
                            ),
                        ],
                    },
                    pattern_symbol_maps: [
                        [
                            (
                                `dp`,
                                1,
                            ),
                        ],
                    ],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [
                            None,
                        ],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [
                            SynInheritedSymbol {
                                parent_symbol_idx: Current(
                                    1,
                                ),
                                modifier: None,
                                kind: SynInheritedSymbolKind::ParenateParameter {
                                    ident: `cc`,
                                },
                            },
                        ],
                    },
                    current_symbol_arena: Arena {
                        data: [
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    3,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            32,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::LetVariable {
                                    ident: `dp`,
                                    pattern_symbol_idx: 1,
                                },
                            },
                        ],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    SynExprRoot {
                        kind: LetStmtInitialValue,
                        syn_expr_idx: 2,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 6,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 11,
                    },
                    SynExprRoot {
                        kind: EvalExpr,
                        syn_expr_idx: 14,
                    },
                    SynExprRoot {
                        kind: BlockExpr,
                        syn_expr_idx: 15,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            Id {
                                value: 55,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 286,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_symbol_idx: 1,
                                    inherited_symbol_kind: ParenateParameter {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 286,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 302,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    Id {
                                                                        value: 53,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    current_symbol_idx: 1,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 276,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                        signature: PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 97,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        4,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ropd: SemaExprIdx(
                                        5,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 286,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    inherited_symbol_idx: 1,
                                    inherited_symbol_kind: ParenateParameter {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 286,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 300,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            18,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 59,
                                            },
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 56,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 56,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 297,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    Id {
                                                                        value: 28,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        24,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 98,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        9,
                                    ),
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    ropd: SemaExprIdx(
                                        10,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 286,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    inherited_symbol_idx: 1,
                                    inherited_symbol_kind: ParenateParameter {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 286,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 151,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            27,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    Id {
                                                                        value: 51,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 51,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        13,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 276,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            31,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                        signature: PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..5,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: SynPatternRoot(
                                            1,
                                        ),
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        6,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        11,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        14,
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    15,
                    (
                        SemaExprIdx(
                            15,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        5,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            0.0,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        10,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            0.6,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [
                        None,
                    ],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        None,
                    ],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 70,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 70,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 56,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 70,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 44,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                WrapInSome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 44,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                WrapInSome,
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
            return_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 44,
                        },
                    ),
                ),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        [salsa id]: 96,
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `FunctionFn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::Prefix {
                                        opr: Tilde,
                                        opr_regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                        opd: 1,
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 2,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::Prefix {
                                        opr: Option,
                                        opr_regional_token_idx: RegionalTokenIdx(
                                            10,
                                        ),
                                        opd: 3,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `ConcaveComponent`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `f32`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::f32`, `Extern`),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: SynPatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [
                                        SynPatternExpr::Ident {
                                            symbol_modifier_tokens: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `cc`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                            },
                                        },
                                    ],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [
                                        None,
                                    ],
                                },
                                pattern_symbol_arena: Arena {
                                    data: [
                                        SynPatternSymbol::Atom(
                                            1,
                                        ),
                                    ],
                                },
                                pattern_symbol_maps: [
                                    [
                                        (
                                            `cc`,
                                            1,
                                        ),
                                    ],
                                ],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [
                                        None,
                                    ],
                                },
                            },
                            symbol_region: SynSymbolRegion {
                                inherited_symbol_arena: Arena {
                                    data: [],
                                },
                                current_symbol_arena: Arena {
                                    data: [
                                        SynCurrentSymbol {
                                            modifier: None,
                                            access_start: RegionalTokenIdx(
                                                5,
                                            ),
                                            access_end: None,
                                            variant: SynCurrentSymbolVariant::ParenateRegularParameter {
                                                ident: `cc`,
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                    ],
                                },
                                allow_self_type: False,
                                allow_self_value: False,
                                pattern_ty_constraints: [
                                    (
                                        ExplicitRegularParameter {
                                            syn_pattern_root: SynPatternRoot(
                                                1,
                                            ),
                                            ty_expr_idx: 2,
                                        },
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                ],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: ExplicitParameterType,
                                    syn_expr_idx: 2,
                                },
                                SynExprRoot {
                                    kind: ReturnType,
                                    syn_expr_idx: 4,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                        },
                    },
                ),
                path: RegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `FunctionFn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        SynExprData::InheritedSymbol {
                            ident: `cc`,
                            regional_token_idx: RegionalTokenIdx(
                                4,
                            ),
                            inherited_symbol_idx: 1,
                            inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                                ident: `cc`,
                            },
                        },
                        SynExprData::MethodApplicationOrCall {
                            self_argument: 1,
                            dot_regional_token_idx: RegionalTokenIdx(
                                5,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `displacement`,
                                regional_token_idx: RegionalTokenIdx(
                                    6,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                7,
                            ),
                            items: [],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                8,
                            ),
                        },
                        SynExprData::CurrentSymbol {
                            ident: `dp`,
                            regional_token_idx: RegionalTokenIdx(
                                10,
                            ),
                            current_symbol_idx: 1,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 1,
                            },
                        },
                        SynExprData::Field {
                            owner: 3,
                            dot_regional_token_idx: RegionalTokenIdx(
                                11,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `y`,
                                regional_token_idx: RegionalTokenIdx(
                                    12,
                                ),
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                14,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 99,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::Binary {
                            lopd: 4,
                            opr: Comparison(
                                Less,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                13,
                            ),
                            ropd: 5,
                        },
                        SynExprData::InheritedSymbol {
                            ident: `cc`,
                            regional_token_idx: RegionalTokenIdx(
                                16,
                            ),
                            inherited_symbol_idx: 1,
                            inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                                ident: `cc`,
                            },
                        },
                        SynExprData::Field {
                            owner: 7,
                            dot_regional_token_idx: RegionalTokenIdx(
                                17,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `relative_bounding_box`,
                                regional_token_idx: RegionalTokenIdx(
                                    18,
                                ),
                            },
                        },
                        SynExprData::MethodApplicationOrCall {
                            self_argument: 8,
                            dot_regional_token_idx: RegionalTokenIdx(
                                19,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `ymin`,
                                regional_token_idx: RegionalTokenIdx(
                                    20,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                21,
                            ),
                            items: [],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                22,
                            ),
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                24,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 100,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::Binary {
                            lopd: 9,
                            opr: Comparison(
                                Less,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                23,
                            ),
                            ropd: 10,
                        },
                        SynExprData::InheritedSymbol {
                            ident: `cc`,
                            regional_token_idx: RegionalTokenIdx(
                                28,
                            ),
                            inherited_symbol_idx: 1,
                            inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                                ident: `cc`,
                            },
                        },
                        SynExprData::MethodApplicationOrCall {
                            self_argument: 12,
                            dot_regional_token_idx: RegionalTokenIdx(
                                29,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `start_tangent`,
                                regional_token_idx: RegionalTokenIdx(
                                    30,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                31,
                            ),
                            items: [],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                32,
                            ),
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                36,
                            ),
                            LiteralData::Bool(
                                True,
                            ),
                        ),
                        SynExprData::MethodApplicationOrCall {
                            self_argument: 13,
                            dot_regional_token_idx: RegionalTokenIdx(
                                33,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `angle`,
                                regional_token_idx: RegionalTokenIdx(
                                    34,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                35,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 14,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                37,
                            ),
                        },
                        SynExprData::CurrentSymbol {
                            ident: `ang`,
                            regional_token_idx: RegionalTokenIdx(
                                39,
                            ),
                            current_symbol_idx: 2,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 2,
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                41,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 101,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::Binary {
                            lopd: 16,
                            opr: Comparison(
                                Less,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                40,
                            ),
                            ropd: 17,
                        },
                        SynExprData::CurrentSymbol {
                            ident: `ang`,
                            regional_token_idx: RegionalTokenIdx(
                                42,
                            ),
                            current_symbol_idx: 2,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 2,
                            },
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                1..7,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    1,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                            ),
                            initial_value: 2,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    9,
                                ),
                            },
                            condition: 6,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    15,
                                ),
                            },
                            condition: 11,
                        },
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    25,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_root: SynPatternRoot(
                                        2,
                                    ),
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        27,
                                    ),
                                ),
                            ),
                            initial_value: 15,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    38,
                                ),
                            },
                            condition: 18,
                        },
                        SynStmtData::Eval {
                            expr_idx: 19,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `ang`,
                                    regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                },
                            },
                        ],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [
                            None,
                            None,
                        ],
                    },
                    pattern_symbol_arena: Arena {
                        data: [
                            SynPatternSymbol::Atom(
                                1,
                            ),
                            SynPatternSymbol::Atom(
                                2,
                            ),
                        ],
                    },
                    pattern_symbol_maps: [
                        [
                            (
                                `dp`,
                                1,
                            ),
                        ],
                        [
                            (
                                `ang`,
                                2,
                            ),
                        ],
                    ],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [
                            None,
                            None,
                        ],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [
                            SynInheritedSymbol {
                                parent_symbol_idx: Current(
                                    1,
                                ),
                                modifier: None,
                                kind: SynInheritedSymbolKind::ParenateParameter {
                                    ident: `cc`,
                                },
                            },
                        ],
                    },
                    current_symbol_arena: Arena {
                        data: [
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    3,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            43,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::LetVariable {
                                    ident: `dp`,
                                    pattern_symbol_idx: 1,
                                },
                            },
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    27,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            43,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::LetVariable {
                                    ident: `ang`,
                                    pattern_symbol_idx: 2,
                                },
                            },
                        ],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    SynExprRoot {
                        kind: LetStmtInitialValue,
                        syn_expr_idx: 2,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 6,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 11,
                    },
                    SynExprRoot {
                        kind: LetStmtInitialValue,
                        syn_expr_idx: 15,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 18,
                    },
                    SynExprRoot {
                        kind: EvalExpr,
                        syn_expr_idx: 19,
                    },
                    SynExprRoot {
                        kind: BlockExpr,
                        syn_expr_idx: 20,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            Id {
                                value: 56,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 286,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_symbol_idx: 1,
                                    inherited_symbol_kind: ParenateParameter {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 286,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 302,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    Id {
                                                                        value: 53,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    current_symbol_idx: 1,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 276,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                        signature: PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 99,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        4,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ropd: SemaExprIdx(
                                        5,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 286,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    inherited_symbol_idx: 1,
                                    inherited_symbol_kind: ParenateParameter {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 286,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 300,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            18,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 59,
                                            },
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 56,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 56,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 296,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    Id {
                                                                        value: 28,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        24,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 100,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        9,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    ropd: SemaExprIdx(
                                        10,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 286,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    inherited_symbol_idx: 1,
                                    inherited_symbol_kind: ParenateParameter {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 286,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 416,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            30,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    Id {
                                                                        value: 53,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        36,
                                    ),
                                    Bool(
                                        True,
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        13,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 357,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            34,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                parenate_parameters: [
                                                    Regular(
                                                        FluffyTermRitchieRegularParameter {
                                                            contract: None,
                                                            ty: FluffyTerm {
                                                                place: None,
                                                                base: Ethereal(
                                                                    EntityPath(
                                                                        TypeOntology(
                                                                            TypePath(
                                                                                Id {
                                                                                    value: 2,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    Id {
                                                                        value: 28,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyTermRitchieRegularParameter {
                                                contract: None,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    Id {
                                                                        value: 2,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    14,
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 512,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    current_symbol_idx: 2,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        41,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 101,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        16,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        40,
                                    ),
                                    ropd: SemaExprIdx(
                                        17,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 512,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    current_symbol_idx: 2,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..7,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: SynPatternRoot(
                                            1,
                                        ),
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        6,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        11,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            25,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: SynPatternRoot(
                                            2,
                                        ),
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            27,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        15,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            38,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        18,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        19,
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    20,
                    (
                        SemaExprIdx(
                            20,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        5,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            0.0,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        10,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            0.3,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        14,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        17,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            30.0,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [
                        None,
                    ],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        None,
                        None,
                    ],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 70,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 70,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 56,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 70,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 2,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 44,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                WrapInSome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 44,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 20,
                                    src: ExpectationSource {
                                        expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                WrapInSome,
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
            return_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 44,
                        },
                    ),
                ),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        [salsa id]: 98,
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 2,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 1,
                                        argument_expr_idx: 2,
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 4,
                                        opt_path: Some(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath {
                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ident: `Seven`,
                                                },
                                            ),
                                        ),
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 3,
                                        argument_expr_idx: 4,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `OneVsAll`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `MnistLabel`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `MnistLabel`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Subitem {
                                        parent: 3,
                                        colon_colon_token: ColonColonRegionalToken(
                                            RegionalTokenIdx(
                                                11,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `Seven`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                            },
                                        ),
                                        path: Ok(
                                            PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath {
                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ident: `Seven`,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: SynPatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [],
                                },
                                pattern_symbol_arena: Arena {
                                    data: [],
                                },
                                pattern_symbol_maps: [],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [],
                                },
                            },
                            symbol_region: SynSymbolRegion {
                                inherited_symbol_arena: Arena {
                                    data: [],
                                },
                                current_symbol_arena: Arena {
                                    data: [],
                                },
                                allow_self_type: False,
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: ReturnType,
                                    syn_expr_idx: 5,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                        },
                    },
                ),
                path: RegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Be {
                            src: 1,
                            be_regional_token_idx: RegionalTokenIdx(
                                3,
                            ),
                            target: Ok(
                                BePatternSynSyndicate {
                                    pattern_expr: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Be {
                            src: 3,
                            be_regional_token_idx: RegionalTokenIdx(
                                7,
                            ),
                            target: Ok(
                                BePatternSynSyndicate {
                                    pattern_expr: SynPatternRoot(
                                        2,
                                    ),
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                },
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 3,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 5,
                            dot_regional_token_idx: RegionalTokenIdx(
                                11,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `max_hole_ilen`,
                                regional_token_idx: RegionalTokenIdx(
                                    12,
                                ),
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                14,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 102,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::Binary {
                            lopd: 6,
                            opr: Comparison(
                                Eq,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                13,
                            ),
                            ropd: 7,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 4,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 9,
                            dot_regional_token_idx: RegionalTokenIdx(
                                19,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `norm`,
                                regional_token_idx: RegionalTokenIdx(
                                    20,
                                ),
                            },
                        },
                        SynExprData::CurrentSymbol {
                            ident: `simple_match_norm`,
                            regional_token_idx: RegionalTokenIdx(
                                22,
                            ),
                            current_symbol_idx: 3,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 3,
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                24,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 103,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::Binary {
                            lopd: 11,
                            opr: Comparison(
                                Less,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                23,
                            ),
                            ropd: 12,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 5,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 14,
                            dot_regional_token_idx: RegionalTokenIdx(
                                28,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `matches`,
                                regional_token_idx: RegionalTokenIdx(
                                    29,
                                ),
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                31,
                            ),
                            LiteralData::Integer(
                                UnspecifiedRegular(
                                    0,
                                ),
                            ),
                        ),
                        SynExprData::IndexOrCompositionWithList {
                            owner: 15,
                            lbox_regional_token_idx: RegionalTokenIdx(
                                30,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 16,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                32,
                            ),
                        },
                        SynExprData::Be {
                            src: 17,
                            be_regional_token_idx: RegionalTokenIdx(
                                33,
                            ),
                            target: Ok(
                                BePatternSynSyndicate {
                                    pattern_expr: SynPatternRoot(
                                        4,
                                    ),
                                    variables: ArenaIdxRange(
                                        4..5,
                                    ),
                                },
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 6,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 7,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 19,
                            dot_regional_token_idx: RegionalTokenIdx(
                                39,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `upper_mass`,
                                regional_token_idx: RegionalTokenIdx(
                                    40,
                                ),
                            },
                        },
                        SynExprData::Field {
                            owner: 20,
                            dot_regional_token_idx: RegionalTokenIdx(
                                43,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `lower_mass`,
                                regional_token_idx: RegionalTokenIdx(
                                    44,
                                ),
                            },
                        },
                        SynExprData::Binary {
                            lopd: 21,
                            opr: Closed(
                                Sub,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                41,
                            ),
                            ropd: 22,
                        },
                        SynExprData::CurrentSymbol {
                            ident: `upper_excess`,
                            regional_token_idx: RegionalTokenIdx(
                                46,
                            ),
                            current_symbol_idx: 5,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 5,
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                48,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 104,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::Binary {
                            lopd: 24,
                            opr: Comparison(
                                Less,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                47,
                            ),
                            ropd: 25,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 8,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 27,
                            dot_regional_token_idx: RegionalTokenIdx(
                                54,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `matches`,
                                regional_token_idx: RegionalTokenIdx(
                                    55,
                                ),
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                57,
                            ),
                            LiteralData::Integer(
                                UnspecifiedRegular(
                                    0,
                                ),
                            ),
                        ),
                        SynExprData::IndexOrCompositionWithList {
                            owner: 28,
                            lbox_regional_token_idx: RegionalTokenIdx(
                                56,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 29,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                58,
                            ),
                        },
                        SynExprData::Suffix {
                            opd: 30,
                            opr: UnwrapOrComposeWithNot,
                            opr_regional_token_idx: RegionalTokenIdx(
                                59,
                            ),
                        },
                        SynExprData::MethodApplicationOrCall {
                            self_argument: 31,
                            dot_regional_token_idx: RegionalTokenIdx(
                                60,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `end_tangent`,
                                regional_token_idx: RegionalTokenIdx(
                                    61,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                62,
                            ),
                            items: [],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                63,
                            ),
                        },
                        SynExprData::CurrentSymbol {
                            ident: `end_tangent`,
                            regional_token_idx: RegionalTokenIdx(
                                67,
                            ),
                            current_symbol_idx: 6,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 6,
                            },
                        },
                        SynExprData::Field {
                            owner: 33,
                            dot_regional_token_idx: RegionalTokenIdx(
                                68,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `y`,
                                regional_token_idx: RegionalTokenIdx(
                                    69,
                                ),
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                74,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 105,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::CurrentSymbol {
                            ident: `a`,
                            regional_token_idx: RegionalTokenIdx(
                                71,
                            ),
                            current_symbol_idx: 7,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 7,
                            },
                        },
                        SynExprData::Prefix {
                            opr: Minus,
                            opr_regional_token_idx: RegionalTokenIdx(
                                73,
                            ),
                            opd: 35,
                        },
                        SynExprData::Binary {
                            lopd: 36,
                            opr: Comparison(
                                Less,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                72,
                            ),
                            ropd: 37,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 10,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath {
                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        ident: `Yes`,
                                    },
                                ),
                            ),
                        },
                        SynExprData::CurrentSymbol {
                            ident: `simple_match_norm`,
                            regional_token_idx: RegionalTokenIdx(
                                80,
                            ),
                            current_symbol_idx: 3,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 3,
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                82,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 106,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::Binary {
                            lopd: 40,
                            opr: Comparison(
                                Less,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                81,
                            ),
                            ropd: 41,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 11,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 12,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 43,
                            dot_regional_token_idx: RegionalTokenIdx(
                                88,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `upper_mass`,
                                regional_token_idx: RegionalTokenIdx(
                                    89,
                                ),
                            },
                        },
                        SynExprData::Field {
                            owner: 44,
                            dot_regional_token_idx: RegionalTokenIdx(
                                92,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `lower_mass`,
                                regional_token_idx: RegionalTokenIdx(
                                    93,
                                ),
                            },
                        },
                        SynExprData::Binary {
                            lopd: 45,
                            opr: Closed(
                                Sub,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                90,
                            ),
                            ropd: 46,
                        },
                        SynExprData::CurrentSymbol {
                            ident: `upper_excess`,
                            regional_token_idx: RegionalTokenIdx(
                                95,
                            ),
                            current_symbol_idx: 8,
                            current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                                pattern_symbol_idx: 8,
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                97,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 107,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::Binary {
                            lopd: 48,
                            opr: Comparison(
                                Greater,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                96,
                            ),
                            ropd: 49,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 14,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath {
                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        ident: `Yes`,
                                    },
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 15,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 52,
                            dot_regional_token_idx: RegionalTokenIdx(
                                104,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `matches`,
                                regional_token_idx: RegionalTokenIdx(
                                    105,
                                ),
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                107,
                            ),
                            LiteralData::Integer(
                                UnspecifiedRegular(
                                    0,
                                ),
                            ),
                        ),
                        SynExprData::IndexOrCompositionWithList {
                            owner: 53,
                            lbox_regional_token_idx: RegionalTokenIdx(
                                106,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 54,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                108,
                            ),
                        },
                        SynExprData::Be {
                            src: 55,
                            be_regional_token_idx: RegionalTokenIdx(
                                109,
                            ),
                            target: Ok(
                                BePatternSynSyndicate {
                                    pattern_expr: SynPatternRoot(
                                        9,
                                    ),
                                    variables: ArenaIdxRange(
                                        9..10,
                                    ),
                                },
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 16,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 57,
                            dot_regional_token_idx: RegionalTokenIdx(
                                115,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `others`,
                                regional_token_idx: RegionalTokenIdx(
                                    116,
                                ),
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                118,
                            ),
                            LiteralData::Bool(
                                False,
                            ),
                        ),
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 18,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath {
                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        ident: `Yes`,
                                    },
                                ),
                            ),
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                11..21,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `is_six`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `is_zero`,
                                    regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_connected_component`,
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `simple_seven_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `simple_seven_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_connected_component`,
                                    regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_connected_component`,
                                    regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `simple_seven_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAll`,
                                    regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 9,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    77,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Yes`,
                                    regional_token_idx: RegionalTokenIdx(
                                        78,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath {
                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        ident: `Yes`,
                                    },
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_connected_component`,
                                    regional_token_idx: RegionalTokenIdx(
                                        87,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_connected_component`,
                                    regional_token_idx: RegionalTokenIdx(
                                        91,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAll`,
                                    regional_token_idx: RegionalTokenIdx(
                                        99,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 13,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    100,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Yes`,
                                    regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath {
                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        ident: `Yes`,
                                    },
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `special_seven_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        103,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `special_seven_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        114,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAll`,
                                    regional_token_idx: RegionalTokenIdx(
                                        119,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 17,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    120,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Yes`,
                                    regional_token_idx: RegionalTokenIdx(
                                        121,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath {
                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        ident: `Yes`,
                                    },
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    50,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_root: SynPatternRoot(
                                        6,
                                    ),
                                    variables: ArenaIdxRange(
                                        6..7,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        52,
                                    ),
                                ),
                            ),
                            initial_value: 32,
                        },
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    64,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_root: SynPatternRoot(
                                        7,
                                    ),
                                    variables: ArenaIdxRange(
                                        7..8,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        66,
                                    ),
                                ),
                            ),
                            initial_value: 34,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    70,
                                ),
                            },
                            condition: 38,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    26,
                                ),
                            },
                            condition: 18,
                        },
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    35,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_root: SynPatternRoot(
                                        5,
                                    ),
                                    variables: ArenaIdxRange(
                                        5..6,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        37,
                                    ),
                                ),
                            ),
                            initial_value: 23,
                        },
                        SynStmtData::IfElse {
                            if_branch: SynIfBranch {
                                if_token: IfRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                },
                                condition: Ok(
                                    26,
                                ),
                                eol_colon: Ok(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            49,
                                        ),
                                    },
                                ),
                                stmts: ArenaIdxRange(
                                    1..4,
                                ),
                            },
                            elif_branches: [],
                            else_branch: None,
                        },
                        SynStmtData::Return {
                            return_token: ReturnRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    75,
                                ),
                            },
                            result: 39,
                        },
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    84,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_root: SynPatternRoot(
                                        8,
                                    ),
                                    variables: ArenaIdxRange(
                                        8..9,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        86,
                                    ),
                                ),
                            ),
                            initial_value: 47,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    94,
                                ),
                            },
                            condition: 50,
                        },
                        SynStmtData::Return {
                            return_token: ReturnRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    98,
                                ),
                            },
                            result: 51,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    1,
                                ),
                            },
                            condition: 2,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    5,
                                ),
                            },
                            condition: 4,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    9,
                                ),
                            },
                            condition: 8,
                        },
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    15,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_root: SynPatternRoot(
                                        3,
                                    ),
                                    variables: ArenaIdxRange(
                                        3..4,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        17,
                                    ),
                                ),
                            ),
                            initial_value: 10,
                        },
                        SynStmtData::IfElse {
                            if_branch: SynIfBranch {
                                if_token: IfRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                },
                                condition: Ok(
                                    13,
                                ),
                                eol_colon: Ok(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            25,
                                        ),
                                    },
                                ),
                                stmts: ArenaIdxRange(
                                    4..8,
                                ),
                            },
                            elif_branches: [],
                            else_branch: None,
                        },
                        SynStmtData::IfElse {
                            if_branch: SynIfBranch {
                                if_token: IfRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        79,
                                    ),
                                },
                                condition: Ok(
                                    42,
                                ),
                                eol_colon: Ok(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            83,
                                        ),
                                    },
                                ),
                                stmts: ArenaIdxRange(
                                    8..11,
                                ),
                            },
                            elif_branches: [],
                            else_branch: None,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    102,
                                ),
                            },
                            condition: 56,
                        },
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    111,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_root: SynPatternRoot(
                                        10,
                                    ),
                                    variables: ArenaIdxRange(
                                        10..11,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        113,
                                    ),
                                ),
                            ),
                            initial_value: 58,
                        },
                        SynStmtData::Require {
                            require_token: RequireRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    117,
                                ),
                            },
                            condition: 59,
                        },
                        SynStmtData::Eval {
                            expr_idx: 60,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `none`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `none`,
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `simple_match_norm`,
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `some`,
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `upper_excess`,
                                    regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `end_tangent`,
                                    regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        65,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `upper_excess`,
                                    regional_token_idx: RegionalTokenIdx(
                                        85,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `some`,
                                    regional_token_idx: RegionalTokenIdx(
                                        110,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `others`,
                                    regional_token_idx: RegionalTokenIdx(
                                        112,
                                    ),
                                },
                            },
                        ],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        ],
                    },
                    pattern_symbol_arena: Arena {
                        data: [
                            SynPatternSymbol::Atom(
                                1,
                            ),
                            SynPatternSymbol::Atom(
                                2,
                            ),
                            SynPatternSymbol::Atom(
                                3,
                            ),
                            SynPatternSymbol::Atom(
                                4,
                            ),
                            SynPatternSymbol::Atom(
                                5,
                            ),
                            SynPatternSymbol::Atom(
                                6,
                            ),
                            SynPatternSymbol::Atom(
                                7,
                            ),
                            SynPatternSymbol::Atom(
                                8,
                            ),
                            SynPatternSymbol::Atom(
                                9,
                            ),
                            SynPatternSymbol::Atom(
                                10,
                            ),
                        ],
                    },
                    pattern_symbol_maps: [
                        [
                            (
                                `none`,
                                1,
                            ),
                        ],
                        [
                            (
                                `none`,
                                2,
                            ),
                        ],
                        [
                            (
                                `simple_match_norm`,
                                3,
                            ),
                        ],
                        [
                            (
                                `some`,
                                4,
                            ),
                        ],
                        [
                            (
                                `upper_excess`,
                                5,
                            ),
                        ],
                        [
                            (
                                `end_tangent`,
                                6,
                            ),
                        ],
                        [
                            (
                                `a`,
                                7,
                            ),
                        ],
                        [
                            (
                                `upper_excess`,
                                8,
                            ),
                        ],
                        [
                            (
                                `some`,
                                9,
                            ),
                        ],
                        [
                            (
                                `others`,
                                10,
                            ),
                        ],
                    ],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        ],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    5,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            122,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::BeVariable {
                                    ident: `none`,
                                    pattern_symbol_idx: 1,
                                },
                            },
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    9,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            122,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::BeVariable {
                                    ident: `none`,
                                    pattern_symbol_idx: 2,
                                },
                            },
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    17,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            122,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::LetVariable {
                                    ident: `simple_match_norm`,
                                    pattern_symbol_idx: 3,
                                },
                            },
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    35,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            79,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::BeVariable {
                                    ident: `some`,
                                    pattern_symbol_idx: 4,
                                },
                            },
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    37,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            79,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::LetVariable {
                                    ident: `upper_excess`,
                                    pattern_symbol_idx: 5,
                                },
                            },
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    52,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            75,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::LetVariable {
                                    ident: `end_tangent`,
                                    pattern_symbol_idx: 6,
                                },
                            },
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    66,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            75,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::LetVariable {
                                    ident: `a`,
                                    pattern_symbol_idx: 7,
                                },
                            },
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    86,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            102,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::LetVariable {
                                    ident: `upper_excess`,
                                    pattern_symbol_idx: 8,
                                },
                            },
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    111,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            122,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::BeVariable {
                                    ident: `some`,
                                    pattern_symbol_idx: 9,
                                },
                            },
                            SynCurrentSymbol {
                                modifier: None,
                                access_start: RegionalTokenIdx(
                                    113,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            122,
                                        ),
                                    ),
                                ),
                                variant: SynCurrentSymbolVariant::LetVariable {
                                    ident: `others`,
                                    pattern_symbol_idx: 10,
                                },
                            },
                        ],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 2,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 4,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 8,
                    },
                    SynExprRoot {
                        kind: LetStmtInitialValue,
                        syn_expr_idx: 10,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 18,
                    },
                    SynExprRoot {
                        kind: LetStmtInitialValue,
                        syn_expr_idx: 23,
                    },
                    SynExprRoot {
                        kind: LetStmtInitialValue,
                        syn_expr_idx: 32,
                    },
                    SynExprRoot {
                        kind: LetStmtInitialValue,
                        syn_expr_idx: 34,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 38,
                    },
                    SynExprRoot {
                        kind: ReturnExpr,
                        syn_expr_idx: 39,
                    },
                    SynExprRoot {
                        kind: LetStmtInitialValue,
                        syn_expr_idx: 47,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 50,
                    },
                    SynExprRoot {
                        kind: ReturnExpr,
                        syn_expr_idx: 51,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 56,
                    },
                    SynExprRoot {
                        kind: LetStmtInitialValue,
                        syn_expr_idx: 58,
                    },
                    SynExprRoot {
                        kind: Condition,
                        syn_expr_idx: 59,
                    },
                    SynExprRoot {
                        kind: EvalExpr,
                        syn_expr_idx: 60,
                    },
                    SynExprRoot {
                        kind: BlockExpr,
                        syn_expr_idx: 61,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            Id {
                                value: 57,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 36,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 29,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Be {
                                    src: SemaExprIdx(
                                        1,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                    target: BePatternSynSyndicate {
                                        pattern_expr: SynPatternRoot(
                                            1,
                                        ),
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Be {
                                    src: SemaExprIdx(
                                        3,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    target: BePatternSynSyndicate {
                                        pattern_expr: SynPatternRoot(
                                            2,
                                        ),
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 72,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        5,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 259,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 102,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        6,
                                    ),
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ropd: SemaExprIdx(
                                        7,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 52,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 83,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 352,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 62,
                                            },
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 514,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    current_symbol_idx: 3,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        24,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 103,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        11,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    ropd: SemaExprIdx(
                                        12,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 52,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 83,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        14,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 248,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            29,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 62,
                                            },
                                        ),
                                        signature: PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 85,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 85,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        31,
                                    ),
                                    Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        15,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                16,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 84,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 84,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Be {
                                    src: SemaExprIdx(
                                        17,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    target: BePatternSynSyndicate {
                                        pattern_expr: SynPatternRoot(
                                            4,
                                        ),
                                        variables: ArenaIdxRange(
                                            4..5,
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 6,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 72,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        19,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 245,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            40,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 7,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 72,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        21,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 246,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            44,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        20,
                                    ),
                                    opr: Closed(
                                        Sub,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    ropd: SemaExprIdx(
                                        22,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 515,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    current_symbol_idx: 5,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        48,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 104,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        24,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        47,
                                    ),
                                    ropd: SemaExprIdx(
                                        25,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 8,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 52,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 83,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        27,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 248,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            55,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 62,
                                            },
                                        ),
                                        signature: PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 85,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 85,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        57,
                                    ),
                                    Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        28,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                29,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        58,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 84,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 84,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Suffix {
                                    opd_sema_expr_idx: SemaExprIdx(
                                        30,
                                    ),
                                    opr: Unwrap,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        31,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        60,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 417,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            61,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    Id {
                                                                        value: 53,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        62,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        63,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 417,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        67,
                                    ),
                                    current_symbol_idx: 6,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        33,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        68,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 276,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            69,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                        signature: PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        71,
                                    ),
                                    current_symbol_idx: 7,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        74,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 105,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        73,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        36,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        35,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    ropd: SemaExprIdx(
                                        37,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 10,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 514,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    current_symbol_idx: 3,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        82,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 106,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        40,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        81,
                                    ),
                                    ropd: SemaExprIdx(
                                        41,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 11,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 72,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        43,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        88,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 245,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            89,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 12,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 72,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        45,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        92,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 246,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            93,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 47,
                                            },
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        44,
                                    ),
                                    opr: Closed(
                                        Sub,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        90,
                                    ),
                                    ropd: SemaExprIdx(
                                        46,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 515,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                    current_symbol_idx: 8,
                                    current_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        97,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        48,
                                    ),
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        96,
                                    ),
                                    ropd: SemaExprIdx(
                                        49,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 14,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 15,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 54,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 83,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        52,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        104,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 248,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            105,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 62,
                                            },
                                        ),
                                        signature: PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 85,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 85,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        107,
                                    ),
                                    Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        53,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        106,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                54,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        108,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 84,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 84,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Be {
                                    src: SemaExprIdx(
                                        55,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    target: BePatternSynSyndicate {
                                        pattern_expr: SynPatternRoot(
                                            9,
                                        ),
                                        variables: ArenaIdxRange(
                                            9..10,
                                        ),
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 16,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 54,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 83,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        57,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        115,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 434,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            116,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 62,
                                            },
                                        ),
                                        signature: PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 86,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 86,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Literal(
                                    RegionalTokenIdx(
                                        118,
                                    ),
                                    Bool(
                                        False,
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 18,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            11..21,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            50,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: SynPatternRoot(
                                            6,
                                        ),
                                        variables: ArenaIdxRange(
                                            6..7,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            52,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        32,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            64,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: SynPatternRoot(
                                            7,
                                        ),
                                        variables: ArenaIdxRange(
                                            7..8,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            66,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        34,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            70,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        38,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            26,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        18,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            35,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: SynPatternRoot(
                                            5,
                                        ),
                                        variables: ArenaIdxRange(
                                            5..6,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            37,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        23,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                IfElse {
                                    sema_if_branch: SemaIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                45,
                                            ),
                                        },
                                        condition: SemaExprIdx(
                                            26,
                                        ),
                                        eol_colon_token: EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                49,
                                            ),
                                        },
                                        stmts: SemaStmtIdxRange(
                                            ArenaIdxRange(
                                                1..4,
                                            ),
                                        ),
                                    },
                                    sema_elif_branches: [],
                                    sema_else_branch: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            75,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        39,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            84,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: SynPatternRoot(
                                            8,
                                        ),
                                        variables: ArenaIdxRange(
                                            8..9,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            86,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        47,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            94,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        50,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            98,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        51,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        2,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        4,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        8,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: SynPatternRoot(
                                            3,
                                        ),
                                        variables: ArenaIdxRange(
                                            3..4,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            17,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                IfElse {
                                    sema_if_branch: SemaIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                21,
                                            ),
                                        },
                                        condition: SemaExprIdx(
                                            13,
                                        ),
                                        eol_colon_token: EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                25,
                                            ),
                                        },
                                        stmts: SemaStmtIdxRange(
                                            ArenaIdxRange(
                                                4..8,
                                            ),
                                        ),
                                    },
                                    sema_elif_branches: [],
                                    sema_else_branch: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                IfElse {
                                    sema_if_branch: SemaIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                79,
                                            ),
                                        },
                                        condition: SemaExprIdx(
                                            42,
                                        ),
                                        eol_colon_token: EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                83,
                                            ),
                                        },
                                        stmts: SemaStmtIdxRange(
                                            ArenaIdxRange(
                                                8..11,
                                            ),
                                        ),
                                    },
                                    sema_elif_branches: [],
                                    sema_else_branch: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            102,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        56,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            111,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: SynPatternRoot(
                                            10,
                                        ),
                                        variables: ArenaIdxRange(
                                            10..11,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            113,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        58,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            117,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        59,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        60,
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    61,
                    (
                        SemaExprIdx(
                            61,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 29,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 84,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 84,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 86,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 29,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 84,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 84,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 86,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        7,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            0.0,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        12,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            1.0,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        16,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    USize(
                                        TermUSizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        25,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            10.0,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        29,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    USize(
                                        TermUSizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        36,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            7.0,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        41,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            4.0,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        49,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        NotNan(
                                            10.0,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        54,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    USize(
                                        TermUSizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        59,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    Bool(
                                        false,
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 29,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 84,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 84,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 86,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    ],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        16,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        29,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        35,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        54,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 4,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 29,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 46,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 83,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 83,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 85,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 84,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 20,
                                    src: ExpectationSource {
                                        expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 46,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 21,
                                    src: ExpectationSource {
                                        expr_idx: 21,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 22,
                                    src: ExpectationSource {
                                        expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 46,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 23,
                                    src: ExpectationSource {
                                        expr_idx: 22,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 24,
                                    src: ExpectationSource {
                                        expr_idx: 23,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 25,
                                    src: ExpectationSource {
                                        expr_idx: 24,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 26,
                                    src: ExpectationSource {
                                        expr_idx: 25,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 27,
                                    src: ExpectationSource {
                                        expr_idx: 26,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 28,
                                    src: ExpectationSource {
                                        expr_idx: 27,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 83,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 29,
                                    src: ExpectationSource {
                                        expr_idx: 28,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 85,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 30,
                                    src: ExpectationSource {
                                        expr_idx: 29,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 31,
                                    src: ExpectationSource {
                                        expr_idx: 30,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 32,
                                    src: ExpectationSource {
                                        expr_idx: 30,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 84,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 33,
                                    src: ExpectationSource {
                                        expr_idx: 31,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 70,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 34,
                                    src: ExpectationSource {
                                        expr_idx: 32,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 35,
                                    src: ExpectationSource {
                                        expr_idx: 33,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 36,
                                    src: ExpectationSource {
                                        expr_idx: 34,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 37,
                                    src: ExpectationSource {
                                        expr_idx: 36,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 38,
                                    src: ExpectationSource {
                                        expr_idx: 35,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 39,
                                    src: ExpectationSource {
                                        expr_idx: 37,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 40,
                                    src: ExpectationSource {
                                        expr_idx: 38,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 41,
                                    src: ExpectationSource {
                                        expr_idx: 39,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 42,
                                    src: ExpectationSource {
                                        expr_idx: 40,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 43,
                                    src: ExpectationSource {
                                        expr_idx: 41,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 44,
                                    src: ExpectationSource {
                                        expr_idx: 42,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 45,
                                    src: ExpectationSource {
                                        expr_idx: 43,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 46,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 46,
                                    src: ExpectationSource {
                                        expr_idx: 45,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 47,
                                    src: ExpectationSource {
                                        expr_idx: 44,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 46,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 48,
                                    src: ExpectationSource {
                                        expr_idx: 46,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 49,
                                    src: ExpectationSource {
                                        expr_idx: 47,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 50,
                                    src: ExpectationSource {
                                        expr_idx: 48,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 51,
                                    src: ExpectationSource {
                                        expr_idx: 49,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 52,
                                    src: ExpectationSource {
                                        expr_idx: 50,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 53,
                                    src: ExpectationSource {
                                        expr_idx: 51,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 54,
                                    src: ExpectationSource {
                                        expr_idx: 52,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 83,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 55,
                                    src: ExpectationSource {
                                        expr_idx: 53,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 85,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 56,
                                    src: ExpectationSource {
                                        expr_idx: 54,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: None,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 57,
                                    src: ExpectationSource {
                                        expr_idx: 55,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 58,
                                    src: ExpectationSource {
                                        expr_idx: 55,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 84,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 59,
                                    src: ExpectationSource {
                                        expr_idx: 56,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 60,
                                    src: ExpectationSource {
                                        expr_idx: 57,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 83,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 61,
                                    src: ExpectationSource {
                                        expr_idx: 58,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 86,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 62,
                                    src: ExpectationSource {
                                        expr_idx: 59,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 63,
                                    src: ExpectationSource {
                                        expr_idx: 60,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 64,
                                    src: ExpectationSource {
                                        expr_idx: 61,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
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
            return_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 32,
                        },
                    ),
                ),
            ),
            self_ty: None,
        },
    },
]