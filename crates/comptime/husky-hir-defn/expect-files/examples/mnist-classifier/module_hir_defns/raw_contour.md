[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `cc`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 63,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `points`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 64,
                                        },
                                    ),
                                ),
                            },
                        ],
                        hir_expr_region: HirEagerExprRegion {
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumTypeHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        hir_expr_region: HirEagerExprRegion {
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        8,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 310,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 2,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 3,
                                },
                                Binary {
                                    lopd: 1,
                                    opr: Shift(
                                        Shr,
                                    ),
                                    ropd: 4,
                                },
                                Literal(
                                    R32(
                                        3,
                                    ),
                                ),
                                Binary {
                                    lopd: 5,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 6,
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 7,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        6,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 310,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 1,
                                    opr: Shift(
                                        Shr,
                                    ),
                                    ropd: 2,
                                },
                                Literal(
                                    R32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 3,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 4,
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 5,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        8,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 310,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 2,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 3,
                                },
                                Binary {
                                    lopd: 1,
                                    opr: Shift(
                                        Shr,
                                    ),
                                    ropd: 4,
                                },
                                Literal(
                                    R32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 5,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 6,
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 7,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
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
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 314,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 1,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            2,
                                        ),
                                        Regular(
                                            3,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 315,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 5,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            6,
                                        ),
                                        Regular(
                                            7,
                                        ),
                                    ],
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
                                    initial_value: 4,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 8,
                                },
                                Match,
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 316,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 317,
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        12,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 321,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 15,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 1,
                                    opr: As,
                                    ropd: 2,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 320,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 15,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 4,
                                    opr: As,
                                    ropd: 5,
                                },
                                Binary {
                                    lopd: 3,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 6,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 29,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 7,
                                    opr: As,
                                    ropd: 8,
                                },
                                Literal(
                                    I32(
                                        2,
                                    ),
                                ),
                                MethodCall {
                                    self_argument: 9,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 123,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            10,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..3,
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
                                    initial_value: 11,
                                },
                                Match,
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 322,
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
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
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 314,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 1,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            2,
                                        ),
                                        Regular(
                                            3,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 315,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 5,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            6,
                                        ),
                                        Regular(
                                            7,
                                        ),
                                    ],
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
                                    initial_value: 4,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 8,
                                },
                                Match,
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 316,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 317,
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
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `prev1`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `prev2`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                        ],
                        hir_expr_region: HirEagerExprRegion {
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        29,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 138,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 302,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        2,
                                    ),
                                ),
                                Binary {
                                    lopd: 4,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 5,
                                },
                                Index {
                                    owner: 3,
                                    items: [
                                        6,
                                    ],
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 302,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 9,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 10,
                                },
                                Index {
                                    owner: 8,
                                    items: [
                                        11,
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 49,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 329,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 14,
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
                                                value: 330,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 16,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 15,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 17,
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            2.0,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 18,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 19,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 329,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 21,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 330,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 23,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 22,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 24,
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            2.0,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 25,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 26,
                                },
                                FnCall {
                                    function: 13,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            20,
                                        ),
                                        Regular(
                                            27,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..5,
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
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 7,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 12,
                                },
                                Eval {
                                    expr_idx: 28,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 302,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 329,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 330,
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
                    path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        267,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                List {
                                    items: [],
                                },
                                AssociatedFn,
                                FnCall {
                                    function: 2,
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        29,
                                    ),
                                ),
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
                                    owner: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 252,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 8,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 9,
                                },
                                Index {
                                    owner: 7,
                                    items: [
                                        10,
                                    ],
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
                                    owner: 12,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 252,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 13,
                                    items: [
                                        14,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 333,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 16,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    ropd: 17,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 334,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 19,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    ropd: 20,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 331,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 22,
                                    items: [
                                        23,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 333,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 334,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 25,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 26,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 335,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 27,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 28,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 336,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 29,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 30,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 333,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 334,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 32,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 33,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 335,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 34,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 35,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 336,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 36,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 37,
                                },
                                Prefix {
                                    opr: Tilde,
                                    opd: 38,
                                },
                                Binary {
                                    lopd: 31,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 39,
                                },
                                Binary {
                                    lopd: 24,
                                    opr: Assign,
                                    ropd: 40,
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        29,
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 331,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 124,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 44,
                                    items: [
                                        45,
                                    ],
                                },
                                List {
                                    items: [],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 124,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 331,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 124,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 49,
                                    items: [
                                        50,
                                    ],
                                },
                                MethodCall {
                                    self_argument: 51,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 125,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
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
                                    owner: 53,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 252,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 55,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 56,
                                },
                                Index {
                                    owner: 54,
                                    items: [
                                        57,
                                    ],
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
                                    owner: 59,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 252,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 60,
                                    items: [
                                        61,
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 13,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 314,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 315,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 63,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            64,
                                        ),
                                        Regular(
                                            65,
                                        ),
                                        Regular(
                                            66,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 324,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 74,
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 76,
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 78,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 338,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 80,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 81,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 339,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 83,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 84,
                                },
                                Binary {
                                    lopd: 82,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 85,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 324,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 340,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 87,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 88,
                                },
                                Binary {
                                    lopd: 86,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 89,
                                },
                                Prefix {
                                    opr: Not,
                                    opd: 90,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 15,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 314,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 315,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 324,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 92,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            93,
                                        ),
                                        Regular(
                                            94,
                                        ),
                                        Regular(
                                            95,
                                        ),
                                        Regular(
                                            96,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 14,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 324,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 347,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 98,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            99,
                                        ),
                                        Regular(
                                            100,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 331,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 102,
                                    items: [
                                        103,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 331,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 105,
                                    items: [
                                        106,
                                    ],
                                },
                                Literal(
                                    R32(
                                        1,
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 108,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    ropd: 109,
                                },
                                Prefix {
                                    opr: Tilde,
                                    opd: 110,
                                },
                                Binary {
                                    lopd: 107,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 111,
                                },
                                Binary {
                                    lopd: 104,
                                    opr: Assign,
                                    ropd: 112,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 348,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 341,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 116,
                                },
                                Binary {
                                    lopd: 115,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 117,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 342,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 120,
                                },
                                Binary {
                                    lopd: 119,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 121,
                                },
                                Binary {
                                    lopd: 118,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 122,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 346,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 124,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 125,
                                },
                                Binary {
                                    lopd: 123,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 126,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 129,
                                },
                                Binary {
                                    lopd: 128,
                                    opr: Comparison(
                                        Neq,
                                    ),
                                    ropd: 130,
                                },
                                Binary {
                                    lopd: 127,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 131,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 345,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 133,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 134,
                                },
                                Binary {
                                    lopd: 132,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 135,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 337,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 137,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 142,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Suffix {
                                    opd: 138,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 16,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 337,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 140,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            141,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 139,
                                    opr: Assign,
                                    ropd: 142,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 337,
                                            },
                                        ),
                                    ),
                                },
                                AssociatedFn,
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 145,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            146,
                                        ),
                                        Regular(
                                            147,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 144,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 139,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            148,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 345,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 151,
                                },
                                Binary {
                                    lopd: 150,
                                    opr: Assign,
                                    ropd: 152,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 155,
                                },
                                Binary {
                                    lopd: 154,
                                    opr: Assign,
                                    ropd: 156,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 341,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 159,
                                },
                                Binary {
                                    lopd: 158,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 160,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 162,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 163,
                                },
                                Binary {
                                    lopd: 161,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 164,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 166,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 167,
                                },
                                Binary {
                                    lopd: 165,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 168,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 337,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 170,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 142,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Suffix {
                                    opd: 171,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                AssociatedFn,
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 173,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            174,
                                        ),
                                        Regular(
                                            175,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 172,
                                    opr: Assign,
                                    ropd: 176,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 345,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 178,
                                    opr: Assign,
                                    ropd: 179,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 346,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 181,
                                    opr: Assign,
                                    ropd: 182,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 341,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 185,
                                },
                                Binary {
                                    lopd: 184,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 186,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 188,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 189,
                                },
                                Binary {
                                    lopd: 187,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 190,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 346,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 192,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 193,
                                },
                                Binary {
                                    lopd: 191,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 194,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 196,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 197,
                                },
                                Binary {
                                    lopd: 195,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 198,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 337,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 200,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 142,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Suffix {
                                    opd: 201,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                AssociatedFn,
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 203,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            204,
                                        ),
                                        Regular(
                                            205,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 202,
                                    opr: Assign,
                                    ropd: 206,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 345,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 209,
                                },
                                Binary {
                                    lopd: 208,
                                    opr: Assign,
                                    ropd: 210,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 213,
                                },
                                Binary {
                                    lopd: 212,
                                    opr: Assign,
                                    ropd: 214,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 337,
                                            },
                                        ),
                                    ),
                                },
                                AssociatedFn,
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 217,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            218,
                                        ),
                                        Regular(
                                            219,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 216,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 139,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            220,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 345,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 222,
                                    opr: Assign,
                                    ropd: 223,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 346,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 225,
                                    opr: Assign,
                                    ropd: 226,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 346,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 228,
                                    opr: Assign,
                                    ropd: 229,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 342,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 341,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 231,
                                    opr: Assign,
                                    ropd: 232,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 341,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 348,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 234,
                                    opr: Assign,
                                    ropd: 235,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 324,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 347,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 237,
                                    opr: Assign,
                                    ropd: 238,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 346,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 241,
                                },
                                Binary {
                                    lopd: 240,
                                    opr: Comparison(
                                        Neq,
                                    ),
                                    ropd: 242,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 346,
                                            },
                                        ),
                                    ),
                                },
                                Suffix {
                                    opd: 244,
                                    opr: Incr,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 341,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 247,
                                },
                                Binary {
                                    lopd: 246,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 248,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 346,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 250,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 251,
                                },
                                Binary {
                                    lopd: 249,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 252,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 254,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 255,
                                },
                                Binary {
                                    lopd: 253,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 256,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 337,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 258,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 143,
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
                                                value: 17,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 337,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 261,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            262,
                                        ),
                                        Regular(
                                            263,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 260,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 139,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            264,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 17,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        51..56,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 11,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 15,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 5,
                                        ty: None,
                                    },
                                    initial_value: 18,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 6,
                                        ty: None,
                                    },
                                    initial_value: 21,
                                },
                                Eval {
                                    expr_idx: 41,
                                },
                                Eval {
                                    expr_idx: 143,
                                },
                                Eval {
                                    expr_idx: 149,
                                },
                                Eval {
                                    expr_idx: 153,
                                },
                                Eval {
                                    expr_idx: 157,
                                },
                                Eval {
                                    expr_idx: 177,
                                },
                                Eval {
                                    expr_idx: 180,
                                },
                                Eval {
                                    expr_idx: 183,
                                },
                                Eval {
                                    expr_idx: 207,
                                },
                                Eval {
                                    expr_idx: 211,
                                },
                                Eval {
                                    expr_idx: 215,
                                },
                                Eval {
                                    expr_idx: 221,
                                },
                                Eval {
                                    expr_idx: 224,
                                },
                                Eval {
                                    expr_idx: 227,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 136,
                                        stmts: ArenaIdxRange(
                                            6..10,
                                        ),
                                    },
                                    elif_branches: [
                                        HirEagerElifBranch {
                                            condition: 169,
                                            stmts: ArenaIdxRange(
                                                10..13,
                                            ),
                                        },
                                        HirEagerElifBranch {
                                            condition: 199,
                                            stmts: ArenaIdxRange(
                                                13..16,
                                            ),
                                        },
                                    ],
                                    else_branch: Some(
                                        HirEagerElseBranch {
                                            stmts: ArenaIdxRange(
                                                16..19,
                                            ),
                                        },
                                    ),
                                },
                                Eval {
                                    expr_idx: 230,
                                },
                                Eval {
                                    expr_idx: 233,
                                },
                                Eval {
                                    expr_idx: 236,
                                },
                                Eval {
                                    expr_idx: 245,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 22,
                                        ty: None,
                                    },
                                    initial_value: 97,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 23,
                                        ty: None,
                                    },
                                    initial_value: 101,
                                },
                                Eval {
                                    expr_idx: 113,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 114,
                                        stmts: ArenaIdxRange(
                                            19..23,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Match,
                                Eval {
                                    expr_idx: 239,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 243,
                                        stmts: ArenaIdxRange(
                                            23..24,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 259,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 7,
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    initial_value: 47,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 8,
                                        ty: None,
                                    },
                                    initial_value: 48,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 9,
                                        ty: None,
                                    },
                                    initial_value: 52,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 10,
                                        ty: None,
                                    },
                                    initial_value: 58,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 11,
                                        ty: None,
                                    },
                                    initial_value: 62,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 12,
                                        ty: None,
                                    },
                                    initial_value: 67,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 13,
                                        ty: None,
                                    },
                                    initial_value: 68,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 14,
                                        ty: None,
                                    },
                                    initial_value: 69,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 15,
                                        ty: None,
                                    },
                                    initial_value: 70,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 16,
                                        ty: None,
                                    },
                                    initial_value: 71,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 17,
                                        ty: None,
                                    },
                                    initial_value: 72,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 18,
                                        ty: None,
                                    },
                                    initial_value: 73,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 19,
                                        ty: None,
                                    },
                                    initial_value: 75,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 20,
                                        ty: None,
                                    },
                                    initial_value: 77,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 21,
                                        ty: None,
                                    },
                                    initial_value: 79,
                                },
                                DoWhile {
                                    condition: 91,
                                    block: ArenaIdxRange(
                                        24..31,
                                    ),
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 257,
                                        stmts: ArenaIdxRange(
                                            31..32,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 265,
                                },
                                While {
                                    condition: 46,
                                    stmts: ArenaIdxRange(
                                        32..50,
                                    ),
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 36,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    initial_value: 1,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 3,
                                },
                                ForBetween {
                                    particulars: HirEagerForBetweenParticulars {
                                        frame_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 258,
                                                },
                                            ),
                                        ),
                                        range: HirEagerForBetweenRange {
                                            initial_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    4,
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    5,
                                                ),
                                                kind: UpperClosed,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        1..6,
                                    ),
                                },
                                ForBetween {
                                    particulars: HirEagerForBetweenParticulars {
                                        frame_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 124,
                                                },
                                            ),
                                        ),
                                        range: HirEagerForBetweenRange {
                                            initial_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    42,
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    43,
                                                ),
                                                kind: UpperClosed,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        50..51,
                                    ),
                                },
                                Return {
                                    result: 266,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 17,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 331,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 333,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 334,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 335,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 336,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 337,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 314,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 315,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 324,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 338,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 339,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 340,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 341,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 342,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 343,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 345,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 346,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 347,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 348,
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
    HirDefn::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `mnist_classifier::raw_contour`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    ),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                trai: HirTrait {
                    trai_path: TraitPath(`core::visual::Visualize`),
                    template_arguments: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 29,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::MethodFn(
                TraitForTypeMethodFnHirDefn {
                    path: TraitForTypeItemPath {
                        impl_block: TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::raw_contour`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                        ident: `visualize`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `visualize`,
                            item_kind: MethodFn,
                        },
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_value_parameter: HirRitchieParameter {
                            contract: None,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 29,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirParenateParameters {
                            data: [],
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 27,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        4,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                SelfValue,
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                EmptyHtmlTag {
                                    function_ident: Ident(
                                        Coword(
                                            Id {
                                                value: 286,
                                            },
                                        ),
                                    ),
                                    arguments: [
                                        HirEagerHtmlArgumentExpr {
                                            property_ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 260,
                                                    },
                                                ),
                                            ),
                                            expr: 2,
                                        },
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 3,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `mnist_classifier::raw_contour`,
                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 29,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `line_segment_sketch`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `line_segment_sketch`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 52,
                                },
                            ),
                        ),
                        hir_expr_region: HirEagerExprRegion {
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                        },
                    },
                    body: Some(
                        5,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                AssociatedFn,
                                SelfValue,
                                Literal(
                                    F32(
                                        NotNan(
                                            1.4,
                                        ),
                                    ),
                                ),
                                FnCall {
                                    function: 1,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            2,
                                        ),
                                        Regular(
                                            3,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 4,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `bounding_box`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `bounding_box`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        hir_expr_region: HirEagerExprRegion {
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                        },
                    },
                    body: Some(
                        54,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                SelfValue,
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    USize(
                                        TermUSizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                Index {
                                    owner: 2,
                                    items: [
                                        3,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 291,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 5,
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
                                                value: 291,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 7,
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
                                                value: 291,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 9,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 291,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 11,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                SelfValue,
                                Field {
                                    owner: 13,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 14,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 138,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                SelfValue,
                                Field {
                                    owner: 16,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 17,
                                    items: [
                                        18,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 296,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 22,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 21,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 63,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            23,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 20,
                                    opr: Assign,
                                    ropd: 24,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 293,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 293,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 296,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 28,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 27,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 58,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            29,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 26,
                                    opr: Assign,
                                    ropd: 30,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 294,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 294,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 296,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 34,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 33,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 63,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            35,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 32,
                                    opr: Assign,
                                    ropd: 36,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 295,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 295,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 296,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 40,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 39,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 58,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            41,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 38,
                                    opr: Assign,
                                    ropd: 42,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 52,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 293,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 45,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            46,
                                        ),
                                        Regular(
                                            47,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 52,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 294,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 295,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 49,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            50,
                                        ),
                                        Regular(
                                            51,
                                        ),
                                    ],
                                },
                                FnCall {
                                    function: 44,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            48,
                                        ),
                                        Regular(
                                            52,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        6..13,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 6,
                                        ty: None,
                                    },
                                    initial_value: 19,
                                },
                                Eval {
                                    expr_idx: 25,
                                },
                                Eval {
                                    expr_idx: 31,
                                },
                                Eval {
                                    expr_idx: 37,
                                },
                                Eval {
                                    expr_idx: 43,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 4,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 6,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 8,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 10,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 5,
                                        ty: None,
                                    },
                                    initial_value: 12,
                                },
                                ForBetween {
                                    particulars: HirEagerForBetweenParticulars {
                                        frame_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 258,
                                                },
                                            ),
                                        ),
                                        range: HirEagerForBetweenRange {
                                            initial_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: None,
                                                kind: LowerClosed,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    15,
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        1..6,
                                    ),
                                },
                                Return {
                                    result: 53,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 291,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 293,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 294,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 295,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 296,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `relative_bounding_box`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `relative_bounding_box`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 50,
                                },
                            ),
                        ),
                        hir_expr_region: HirEagerExprRegion {
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                        },
                    },
                    body: Some(
                        10,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                SelfValue,
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 2,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 254,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    USize(
                                        TermUSizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                Index {
                                    owner: 3,
                                    items: [
                                        4,
                                    ],
                                },
                                Field {
                                    owner: 5,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 289,
                                            },
                                        ),
                                    ),
                                },
                                SelfValue,
                                Field {
                                    owner: 7,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 289,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 298,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            8,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 9,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `contour_len`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `contour_len`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 14,
                                },
                            ),
                        ),
                        hir_expr_region: HirEagerExprRegion {
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                        },
                    },
                    body: Some(
                        59,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                Literal(
                                    F32(
                                        NotNan(
                                            0.0,
                                        ),
                                    ),
                                ),
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                SelfValue,
                                Field {
                                    owner: 3,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 4,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 138,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                SelfValue,
                                Field {
                                    owner: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 8,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 9,
                                },
                                Index {
                                    owner: 7,
                                    items: [
                                        10,
                                    ],
                                },
                                SelfValue,
                                Field {
                                    owner: 12,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 13,
                                    items: [
                                        14,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 17,
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
                                                value: 154,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 19,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 18,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 20,
                                },
                                MethodCall {
                                    self_argument: 21,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 23,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 154,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 25,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 24,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 26,
                                },
                                MethodCall {
                                    self_argument: 27,
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
                                Binary {
                                    lopd: 22,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 28,
                                },
                                Binary {
                                    lopd: 16,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 29,
                                },
                                SelfValue,
                                Field {
                                    owner: 31,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                SelfValue,
                                Field {
                                    owner: 33,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 34,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 138,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 35,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 36,
                                },
                                Index {
                                    owner: 32,
                                    items: [
                                        37,
                                    ],
                                },
                                SelfValue,
                                Field {
                                    owner: 39,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    USize(
                                        TermUSizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                Index {
                                    owner: 40,
                                    items: [
                                        41,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 44,
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
                                                value: 154,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 46,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 45,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 47,
                                },
                                MethodCall {
                                    self_argument: 48,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 50,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 154,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 52,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 51,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 53,
                                },
                                MethodCall {
                                    self_argument: 54,
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
                                Binary {
                                    lopd: 49,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 55,
                                },
                                Binary {
                                    lopd: 43,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 56,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        4..10,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 11,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 15,
                                },
                                Eval {
                                    expr_idx: 30,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 1,
                                },
                                ForBetween {
                                    particulars: HirEagerForBetweenParticulars {
                                        frame_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 258,
                                                },
                                            ),
                                        ),
                                        range: HirEagerForBetweenRange {
                                            initial_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    2,
                                                ),
                                                kind: LowerOpen,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    5,
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        1..4,
                                    ),
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 38,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 5,
                                        ty: None,
                                    },
                                    initial_value: 42,
                                },
                                Eval {
                                    expr_idx: 57,
                                },
                                Return {
                                    result: 58,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 154,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 154,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `displacement`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `displacement`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 29,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_value_parameter: HirRitchieParameter {
                            contract: None,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 29,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirParenateParameters {
                            data: [
                                HirRitchieParameter {
                                    contract: None,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                },
                                HirRitchieParameter {
                                    contract: None,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 47,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        19,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                SelfValue,
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 2,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 138,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                SelfValue,
                                Field {
                                    owner: 4,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 302,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 6,
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    ropd: 7,
                                },
                                Index {
                                    owner: 5,
                                    items: [
                                        8,
                                    ],
                                },
                                SelfValue,
                                Field {
                                    owner: 10,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 302,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 12,
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    ropd: 13,
                                },
                                Index {
                                    owner: 11,
                                    items: [
                                        14,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 303,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 304,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 16,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 305,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            17,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..5,
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
                                    initial_value: 3,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 9,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 15,
                                },
                                Eval {
                                    expr_idx: 18,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 302,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 303,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 304,
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