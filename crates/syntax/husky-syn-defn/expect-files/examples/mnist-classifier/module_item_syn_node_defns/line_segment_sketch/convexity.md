[
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Fugitive(
                FugitiveSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::MajorItem(
                            MajorItemSynNodePathData::Fugitive(
                                FugitiveSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Some(
            ItemSynNodeDefn {
                body: 94,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::basic::bool`, `Extern`),
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
                                                        ident: `LineSegmentSketch`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `bool`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::basic::bool`, `Extern`),
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
                                                        ident: `line_segment_sketch`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `index`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
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
                                                    `line_segment_sketch`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `index`,
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `line_segment_sketch`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `index`,
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 2,
                                                    },
                                                    ty_expr_idx: 2,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 2,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 3,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            1,
                                            1,
                                        ),
                                        (
                                            2,
                                            2,
                                        ),
                                    ],
                                },
                            },
                        ),
                        path: SynNodeRegionPath::Defn(
                            ItemSynNodePath::MajorItem(
                                MajorItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Fugitive(
                                                    FugitiveSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        ),
                        syn_expr_arena: Arena {
                            data: [
                                SynExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `strokes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 2,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 4,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `strokes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            16,
                                        ),
                                    },
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `index`,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `L`,
                                    regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 6,
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ropd: 7,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 5,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 8,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 9,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 11,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `strokes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            31,
                                        ),
                                    },
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `index`,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        36,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 13,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    ropd: 14,
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    item: 15,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `L`,
                                    regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 16,
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    ropd: 17,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 12,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 18,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        40,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 19,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            42,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        44,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `previous_displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `current_displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 21,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        49,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `rotation_direction_to`,
                                        regional_token_idx: RegionalTokenIdx(
                                            50,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 22,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `is_rotation_counterclockwise_result`,
                                    regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
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
                                SynExprData::Binary {
                                    lopd: 24,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    ropd: 25,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        64,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 33,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        63,
                                    ),
                                    opd: 27,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        68,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 29,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        69,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `strokes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            70,
                                        ),
                                    },
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        73,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `index`,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        75,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 31,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        74,
                                    ),
                                    ropd: 32,
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    item: 33,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `L`,
                                    regional_token_idx: RegionalTokenIdx(
                                        78,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 34,
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        77,
                                    ),
                                    ropd: 35,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 30,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        71,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 36,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        79,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 37,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            81,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `previous_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        83,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 39,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        84,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `start`,
                                        regional_token_idx: RegionalTokenIdx(
                                            85,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        86,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        87,
                                    ),
                                },
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        89,
                                    ),
                                    ident: `i1`,
                                    frame_var_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        41,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `previous_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        91,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 40,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        88,
                                    ),
                                    ropd: 41,
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 42,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        92,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end`,
                                        regional_token_idx: RegionalTokenIdx(
                                            93,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        94,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 43,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        90,
                                    ),
                                    ropd: 44,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        100,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 46,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `contour`,
                                        regional_token_idx: RegionalTokenIdx(
                                            102,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `previous_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        106,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 48,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        107,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `start`,
                                        regional_token_idx: RegionalTokenIdx(
                                            108,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        110,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        112,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        41,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 47,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        103,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            104,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        105,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 49,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    111,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 50,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        113,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `previous_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        116,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `current_displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        120,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        124,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 53,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        121,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `cross`,
                                        regional_token_idx: RegionalTokenIdx(
                                            122,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        123,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 54,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        125,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `previous_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        114,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 52,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        117,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `max`,
                                        regional_token_idx: RegionalTokenIdx(
                                            118,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        119,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 55,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        126,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 56,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        115,
                                    ),
                                    ropd: 57,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        132,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 34,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        131,
                                    ),
                                    opd: 59,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        136,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 61,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        137,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `strokes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            138,
                                        ),
                                    },
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        140,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `index`,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `L`,
                                    regional_token_idx: RegionalTokenIdx(
                                        142,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 63,
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        141,
                                    ),
                                    ropd: 64,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 62,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        139,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 65,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        143,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 66,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        144,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            145,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `current_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        147,
                                    ),
                                    current_syn_symbol_idx: 10,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 9,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 68,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        148,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `start`,
                                        regional_token_idx: RegionalTokenIdx(
                                            149,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        150,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        151,
                                    ),
                                },
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        153,
                                    ),
                                    ident: `i2`,
                                    frame_var_symbol_idx: 11,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        70,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `current_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        155,
                                    ),
                                    current_syn_symbol_idx: 10,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 9,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 69,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        152,
                                    ),
                                    ropd: 70,
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 71,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        156,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end`,
                                        regional_token_idx: RegionalTokenIdx(
                                            157,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        158,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        159,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 72,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        154,
                                    ),
                                    ropd: 73,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        164,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `line_segment_sketch`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 75,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        165,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `contour`,
                                        regional_token_idx: RegionalTokenIdx(
                                            166,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `previous_interval`,
                                    regional_token_idx: RegionalTokenIdx(
                                        170,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 77,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        171,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `start`,
                                        regional_token_idx: RegionalTokenIdx(
                                            172,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        173,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        174,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        176,
                                    ),
                                    current_syn_symbol_idx: 11,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        70,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 76,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        167,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            168,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        169,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 78,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    175,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 79,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        177,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `current_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        180,
                                    ),
                                    current_syn_symbol_idx: 9,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `current_displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        184,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `displacement`,
                                    regional_token_idx: RegionalTokenIdx(
                                        188,
                                    ),
                                    current_syn_symbol_idx: 12,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 10,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 82,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        185,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `cross`,
                                        regional_token_idx: RegionalTokenIdx(
                                            186,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        187,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 83,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        189,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `current_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        178,
                                    ),
                                    current_syn_symbol_idx: 9,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 81,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        181,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `max`,
                                        regional_token_idx: RegionalTokenIdx(
                                            182,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        183,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 84,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        190,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 85,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        179,
                                    ),
                                    ropd: 86,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `current_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        192,
                                    ),
                                    current_syn_symbol_idx: 9,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `previous_raw_cross`,
                                    regional_token_idx: RegionalTokenIdx(
                                        194,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 88,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        193,
                                    ),
                                    ropd: 89,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `is_rotation_counterclockwise_result`,
                                    regional_token_idx: RegionalTokenIdx(
                                        198,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        200,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 91,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        199,
                                    ),
                                    ropd: 92,
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        13..18,
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
                                            97,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 7,
                                            },
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
                                                99,
                                            ),
                                        ),
                                    ),
                                    initial_value: 51,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 58,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            161,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 10,
                                            },
                                            variables: ArenaIdxRange(
                                                12..13,
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
                                                163,
                                            ),
                                        ),
                                    ),
                                    initial_value: 80,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 87,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            59,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 5,
                                            },
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
                                                62,
                                            ),
                                        ),
                                    ),
                                    initial_value: 28,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            65,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 6,
                                            },
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
                                                67,
                                            ),
                                        ),
                                    ),
                                    initial_value: 38,
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            82,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            89,
                                        ),
                                        for_between_loop_var_ident: `i1`,
                                        for_between_loop_var_expr_idx: 41,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        40,
                                                    ),
                                                    kind: LowerClosed,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        44,
                                                    ),
                                                    kind: UpperOpen,
                                                },
                                                step: Constant(
                                                    1,
                                                ),
                                            },
                                        ),
                                    },
                                    for_loop_var_symbol_idx: 7,
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    96,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        1..3,
                                    ),
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            127,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 8,
                                            },
                                            variables: ArenaIdxRange(
                                                9..10,
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
                                                130,
                                            ),
                                        ),
                                    ),
                                    initial_value: 60,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            133,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 9,
                                            },
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
                                                135,
                                            ),
                                        ),
                                    ),
                                    initial_value: 67,
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            146,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            153,
                                        ),
                                        for_between_loop_var_ident: `i2`,
                                        for_between_loop_var_expr_idx: 70,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        69,
                                                    ),
                                                    kind: LowerClosed,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        73,
                                                    ),
                                                    kind: UpperOpen,
                                                },
                                                step: Constant(
                                                    1,
                                                ),
                                            },
                                        ),
                                    },
                                    for_loop_var_symbol_idx: 11,
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    160,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        3..5,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            191,
                                        ),
                                    },
                                    result: 90,
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            197,
                                        ),
                                    },
                                    result: 93,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 1,
                                            },
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
                                    initial_value: 3,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            11,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
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
                                                13,
                                            ),
                                        ),
                                    ),
                                    initial_value: 10,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            26,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 3,
                                            },
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
                                                28,
                                            ),
                                        ),
                                    ),
                                    initial_value: 20,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            45,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 4,
                                            },
                                            variables: ArenaIdxRange(
                                                4..5,
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
                                                47,
                                            ),
                                        ),
                                    ),
                                    initial_value: 23,
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                54,
                                            ),
                                        },
                                        condition: Ok(
                                            26,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    58,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            5..12,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: Some(
                                        SynElseBranch {
                                            else_token: ElseRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    195,
                                                ),
                                            },
                                            eol_colon_token: Ok(
                                                EolColonRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        196,
                                                    ),
                                                },
                                            ),
                                            stmts: ArenaIdxRange(
                                                12..13,
                                            ),
                                        },
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
                                            ident: `L`,
                                            regional_token_idx: RegionalTokenIdx(
                                                2,
                                            ),
                                        },
                                    },
                                    SynPatternExpr::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `current_displacement`,
                                            regional_token_idx: RegionalTokenIdx(
                                                12,
                                            ),
                                        },
                                    },
                                    SynPatternExpr::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `previous_displacement`,
                                            regional_token_idx: RegionalTokenIdx(
                                                27,
                                            ),
                                        },
                                    },
                                    SynPatternExpr::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `is_rotation_counterclockwise_result`,
                                            regional_token_idx: RegionalTokenIdx(
                                                46,
                                            ),
                                        },
                                    },
                                    SynPatternExpr::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        60,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `previous_raw_cross`,
                                            regional_token_idx: RegionalTokenIdx(
                                                61,
                                            ),
                                        },
                                    },
                                    SynPatternExpr::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `previous_interval`,
                                            regional_token_idx: RegionalTokenIdx(
                                                66,
                                            ),
                                        },
                                    },
                                    SynPatternExpr::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `displacement`,
                                            regional_token_idx: RegionalTokenIdx(
                                                98,
                                            ),
                                        },
                                    },
                                    SynPatternExpr::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        128,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `current_raw_cross`,
                                            regional_token_idx: RegionalTokenIdx(
                                                129,
                                            ),
                                        },
                                    },
                                    SynPatternExpr::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `current_interval`,
                                            regional_token_idx: RegionalTokenIdx(
                                                134,
                                            ),
                                        },
                                    },
                                    SynPatternExpr::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `displacement`,
                                            regional_token_idx: RegionalTokenIdx(
                                                162,
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
                                    Move,
                                    None,
                                    None,
                                    Move,
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
                                        `L`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `current_displacement`,
                                        2,
                                    ),
                                ],
                                [
                                    (
                                        `previous_displacement`,
                                        3,
                                    ),
                                ],
                                [
                                    (
                                        `is_rotation_counterclockwise_result`,
                                        4,
                                    ),
                                ],
                                [
                                    (
                                        `previous_raw_cross`,
                                        5,
                                    ),
                                ],
                                [
                                    (
                                        `previous_interval`,
                                        6,
                                    ),
                                ],
                                [
                                    (
                                        `displacement`,
                                        7,
                                    ),
                                ],
                                [
                                    (
                                        `current_raw_cross`,
                                        8,
                                    ),
                                ],
                                [
                                    (
                                        `current_interval`,
                                        9,
                                    ),
                                ],
                                [
                                    (
                                        `displacement`,
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
                                    Mut,
                                    None,
                                    None,
                                    Mut,
                                    None,
                                    None,
                                ],
                            },
                        },
                        symbol_region: SynSymbolRegionData {
                            inherited_syn_symbol_arena: Arena {
                                data: [
                                    InheritedSynSymbol {
                                        parent_symbol_idx: Current(
                                            1,
                                        ),
                                        modifier: None,
                                        kind: InheritedSynSymbolKind::ParenateParameter {
                                            ident: `line_segment_sketch`,
                                        },
                                    },
                                    InheritedSynSymbol {
                                        parent_symbol_idx: Current(
                                            2,
                                        ),
                                        modifier: None,
                                        kind: InheritedSynSymbolKind::ParenateParameter {
                                            ident: `index`,
                                        },
                                    },
                                ],
                            },
                            current_syn_symbol_arena: Arena {
                                data: [
                                    CurrentSynSymbol {
                                        modifier: None,
                                        access_start: RegionalTokenIdx(
                                            3,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    201,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `L`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: None,
                                        access_start: RegionalTokenIdx(
                                            13,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    201,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `current_displacement`,
                                            pattern_symbol_idx: 2,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: None,
                                        access_start: RegionalTokenIdx(
                                            28,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    201,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `previous_displacement`,
                                            pattern_symbol_idx: 3,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: None,
                                        access_start: RegionalTokenIdx(
                                            47,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    201,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `is_rotation_counterclockwise_result`,
                                            pattern_symbol_idx: 4,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            62,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    195,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `previous_raw_cross`,
                                            pattern_symbol_idx: 5,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: None,
                                        access_start: RegionalTokenIdx(
                                            67,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    195,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `previous_interval`,
                                            pattern_symbol_idx: 6,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: None,
                                        access_start: RegionalTokenIdx(
                                            97,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    127,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `i1`,
                                            expr_idx: 41,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: None,
                                        access_start: RegionalTokenIdx(
                                            99,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    127,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `displacement`,
                                            pattern_symbol_idx: 7,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            130,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    195,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `current_raw_cross`,
                                            pattern_symbol_idx: 8,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: None,
                                        access_start: RegionalTokenIdx(
                                            135,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    195,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `current_interval`,
                                            pattern_symbol_idx: 9,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: None,
                                        access_start: RegionalTokenIdx(
                                            161,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    191,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `i2`,
                                            expr_idx: 70,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: None,
                                        access_start: RegionalTokenIdx(
                                            163,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    191,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `displacement`,
                                            pattern_symbol_idx: 10,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: False,
                            allow_self_value: False,
                            pattern_ty_constraints: [
                                (
                                    LoopVariable,
                                    ArenaIdxRange(
                                        7..8,
                                    ),
                                ),
                                (
                                    LoopVariable,
                                    ArenaIdxRange(
                                        11..12,
                                    ),
                                ),
                            ],
                        },
                        syn_pattern_expr_roots: [
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 1,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 2,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 3,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 4,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 5,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 6,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 7,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 8,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 9,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 10,
                            },
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 3,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 10,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 20,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 23,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 28,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 38,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 51,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 58,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 60,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 67,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 80,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 87,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 90,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 93,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 94,
                            },
                        ],
                        has_self_lifetime: false,
                        has_self_place: false,
                        syn_pattern_to_current_syn_symbol_map: [
                            (
                                1,
                                1,
                            ),
                            (
                                2,
                                2,
                            ),
                            (
                                3,
                                3,
                            ),
                            (
                                4,
                                4,
                            ),
                            (
                                5,
                                5,
                            ),
                            (
                                6,
                                6,
                            ),
                            (
                                7,
                                8,
                            ),
                            (
                                8,
                                9,
                            ),
                            (
                                9,
                                10,
                            ),
                            (
                                10,
                                12,
                            ),
                        ],
                    },
                },
            },
        ),
    ),
]