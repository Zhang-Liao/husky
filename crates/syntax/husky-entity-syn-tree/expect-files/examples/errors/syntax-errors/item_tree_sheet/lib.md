EntitySynTreeSheet {
    module_path: `syntax_errors`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::Submodule(
                    SubmoduleSynNode {
                        syn_node_path: SubmoduleSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::Submodule(
                                    SubmoduleSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmoduleItemPath(
                                                ItemPathId {
                                                    data: ItemPathData::SubmoduleItem(
                                                        SubmoduleItemPathData {
                                                            submodule_path: SubmodulePath(
                                                                `syntax_errors::ast`,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            },
                        ),
                        visibility: Scope::PubUnder(
                            `syntax_errors`,
                        ),
                        ast_idx: 1,
                        ident_token: IdentToken {
                            ident: `ast`,
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::Submodule(
                    Room32,
                    SubmoduleSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::Submodule(
                                SubmoduleSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmoduleItemPath(
                                            ItemPathId {
                                                data: ItemPathData::SubmoduleItem(
                                                    SubmoduleItemPathData {
                                                        submodule_path: SubmodulePath(
                                                            `syntax_errors::ast`,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        },
                    ),
                ),
                ident: `ast`,
                visibility: Scope::PubUnder(
                    `syntax_errors`,
                ),
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `ast`,
                visibility: Scope::PubUnder(
                    `syntax_errors`,
                ),
                symbol: EntitySymbol::Submodule {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `syntax_errors::ast`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
        ],
    ),
    impl_block_syn_node_table: [],
    once_use_rules: UseOneRules(
        [],
    ),
    use_all_rules: UseAllRules(
        [],
    ),
    errors: [],
}