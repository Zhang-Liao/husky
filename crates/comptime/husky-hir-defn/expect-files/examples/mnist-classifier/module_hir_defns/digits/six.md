[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        Eager(
                            6,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 47,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        Eager(
                            7,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 48,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        Lazy(
                            109,
                        ),
                    ),
                    hir_expr_region: Lazy(
                        HirLazyExprRegion(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        9,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 300,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 3,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            0.0,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 4,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 5,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 7,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..4,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 2,
                                },
                                Require {
                                    condition: 6,
                                },
                                Eval {
                                    expr_idx: 8,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        35,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 300,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 3,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            3.0,
                                        ),
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 5,
                                },
                                Binary {
                                    lopd: 4,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 6,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 8,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 10,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 9,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 11,
                                },
                                MethodCall {
                                    self_argument: 12,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 57,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            1.4,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 13,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 14,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 16,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 298,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 17,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 295,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            0.6,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 18,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 19,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 21,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 217,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 22,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 289,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 24,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                MethodCall {
                                    self_argument: 23,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 367,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            25,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 482,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 27,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            0.5,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 28,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 29,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 31,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Field {
                                    owner: 32,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Prefix {
                                    opr: Minus,
                                    opd: 33,
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        2..8,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Require {
                                    condition: 15,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 2,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 7,
                                        stmts: ArenaIdxRange(
                                            1..2,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Require {
                                    condition: 20,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 26,
                                },
                                Require {
                                    condition: 30,
                                },
                                Eval {
                                    expr_idx: 34,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 482,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                    },
                },
            ),
        ),
    ),
]