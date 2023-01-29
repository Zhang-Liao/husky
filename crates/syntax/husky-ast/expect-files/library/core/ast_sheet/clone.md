Ok(
    AstSheet {
        arena: Arena {
            data: [
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TraitItem(
                            Method,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentifierToken {
                        ident: `clone`,
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        6,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..1,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    entity_path: Some(
                        TraitPath(`core::clone::Clone`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `Clone`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        3,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            1..2,
        ),
    },
)