Ok(
    [
        (
            TokenIdx(
                0,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 0,
                                },
                                end: Position {
                                    line: 2,
                                    character: 3,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                3,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 3;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::LaOrLt,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 15,
                                },
                                end: Position {
                                    line: 2,
                                    character: 16,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                6,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 6;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Ident(\n    `E`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {\n        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `E`,\n                token_idx: TokenIdx(\n                    6,\n                ),\n            },\n        },\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        7,\n    ),\n    access_end: None,\n    variant: CurrentSymbolVariant::ImplicitParameter {\n        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {\n            ident_token: IdentToken {\n                ident: `E`,\n                token_idx: TokenIdx(\n                    6,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 19,
                                },
                                end: Position {
                                    line: 2,
                                    character: 20,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                9,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 9;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Ident(\n    `Ok`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 2,
                                },
                                end: Position {
                                    line: 3,
                                    character: 4,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                12,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 12;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 6,
                                },
                                end: Position {
                                    line: 3,
                                    character: 7,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                15,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 15;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 5,
                                },
                                end: Position {
                                    line: 4,
                                    character: 6,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                18,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 18;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Keyword(\n    Keyword::Impl,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 0,
                                },
                                end: Position {
                                    line: 6,
                                    character: 4,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                21,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 21;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 6,
                                },
                                end: Position {
                                    line: 6,
                                    character: 7,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                24,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 24;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Ident(\n    `E2`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 2,\n    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {\n        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `E2`,\n                token_idx: TokenIdx(\n                    24,\n                ),\n            },\n        },\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        25,\n    ),\n    access_end: None,\n    variant: CurrentSymbolVariant::ImplicitParameter {\n        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {\n            ident_token: IdentToken {\n                ident: `E2`,\n                token_idx: TokenIdx(\n                    24,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 12,
                                },
                                end: Position {
                                    line: 6,
                                    character: 14,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                27,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 27;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 21,
                                },
                                end: Position {
                                    line: 6,
                                    character: 23,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                30,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 30;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Ident(\n    `Unveil`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::ModuleItem(\n            ModuleItemPath::Trait(\n                TraitPath(`core::ops::Unveil`),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 28,
                                },
                                end: Position {
                                    line: 6,
                                    character: 34,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                33,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 33;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Ident(\n    `T`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {\n        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `T`,\n                token_idx: TokenIdx(\n                    20,\n                ),\n            },\n        },\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        21,\n    ),\n    access_end: None,\n    variant: CurrentSymbolVariant::ImplicitParameter {\n        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {\n            ident_token: IdentToken {\n                ident: `T`,\n                token_idx: TokenIdx(\n                    20,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 43,
                                },
                                end: Position {
                                    line: 6,
                                    character: 44,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                36,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 36;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Keyword(\n    Keyword::Connection(\n        For,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 49,
                                },
                                end: Position {
                                    line: 6,
                                    character: 52,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                39,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 39;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Ident(\n    `E2`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 2,\n    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {\n        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `E2`,\n                token_idx: TokenIdx(\n                    24,\n                ),\n            },\n        },\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        25,\n    ),\n    access_end: None,\n    variant: CurrentSymbolVariant::ImplicitParameter {\n        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {\n            ident_token: IdentToken {\n                ident: `E2`,\n                token_idx: TokenIdx(\n                    24,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 62,
                                },
                                end: Position {
                                    line: 6,
                                    character: 64,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                42,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 42;\n\ntoken_line_group_idx = 4\n\ntoken = Token::Ident(\n    `Continue`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::AssociatedItem(\n            AssociatedItemPath::TraitForTypeItem(\n                TraitForTypeItemPath {\n                    parent_ty: TypePath(`core::result::Result`, `Enum`),\n                    trai: TraitPath(`core::ops::Unveil`),\n                    ident: `Continue`,\n                    item_kind: AssociatedType,\n                },\n            ),\n        ),\n    ),\n    Some(\n        AssociatedItem {\n            associated_item_kind: TraitForTypeItem(\n                AssociatedType,\n            ),\n        },\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 9,
                                },
                                end: Position {
                                    line: 7,
                                    character: 17,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                45,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "This is a paradigm\ntoken_idx = 45;\n\ntoken_line_group_idx = 5\n\ntoken = Token::Keyword(\n    Keyword::Fugitive(\n        Fn,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 4,
                                },
                                end: Position {
                                    line: 9,
                                    character: 6,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                48,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 48;\n\ntoken_line_group_idx = 5\n\ntoken = Token::Ident(\n    `result`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSymbolKind::Parameter {\n        pattern_symbol_idx: 0,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Pure,\n    access_start: TokenIdx(\n        49,\n    ),\n    access_end: None,\n    variant: CurrentSymbolVariant::ExplicitParameter {\n        ident: `result`,\n        pattern_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 14,
                                },
                                end: Position {
                                    line: 9,
                                    character: 20,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                51,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 51;\n\ntoken_line_group_idx = 5\n\ntoken = Token::Ident(\n    `T`,\n);\n\ntoken_info = TokenInfo::InheritedSymbol {\n    inherited_symbol_idx: 0,\n    inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(\n        InheritedImplicitParameterSymbol::Type {\n            ident: `T`,\n        },\n    ),\n    expr_region: ExprRegionLeash(_),\n};\n\nInheritedSymbol {\n    parent_symbol_idx: Current(\n        0,\n    ),\n    modifier: Const,\n    kind: InheritedSymbolKind::ImplicitParameter(\n        InheritedImplicitParameterSymbol::Type {\n            ident: `T`,\n        },\n    ),\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 29,
                                },
                                end: Position {
                                    line: 9,
                                    character: 30,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                54,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 54;\n\ntoken_line_group_idx = 5\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            Curry,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 35,
                                },
                                end: Position {
                                    line: 9,
                                    character: 37,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                57,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 57;\n\ntoken_line_group_idx = 5\n\ntoken = Token::Ident(\n    `E2`,\n);\n\ntoken_info = TokenInfo::InheritedSymbol {\n    inherited_symbol_idx: 2,\n    inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(\n        InheritedImplicitParameterSymbol::Type {\n            ident: `E2`,\n        },\n    ),\n    expr_region: ExprRegionLeash(_),\n};\n\nInheritedSymbol {\n    parent_symbol_idx: Current(\n        2,\n    ),\n    modifier: Const,\n    kind: InheritedSymbolKind::ImplicitParameter(\n        InheritedImplicitParameterSymbol::Type {\n            ident: `E2`,\n        },\n    ),\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 47,
                                },
                                end: Position {
                                    line: 9,
                                    character: 49,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
    ],
)