```rust
[
    (
        ItemPath(`quick_sort::quick_sort`),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Mono,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Type {
                                    ident: `T`,
                                    traits: [
                                        HirTrait {
                                            trai_path: TraitPath(`core::cmp::Ord`),
                                            template_arguments: [],
                                        },
                                    ],
                                },
                            },
                        ],
                    ),
                ),
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::slice::Slice(0)::len`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            path: ItemPath(`core::slice::Slice(0)::len`),
                            context: HirTypeContext {
                                comptime_var_overrides: [],
                            },
                            variable_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::Variable(
                                                HirTypeTemplateVariable::Type {
                                                    attrs: HirTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        path: JavPath::Form(
                            MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                Fn,
                            )`),
                        ),
                        hir_instantiation: HirInstantiation {
                            path: ItemPath(`quick_sort::quick_sort_aux`),
                            context: HirTypeContext {
                                comptime_var_overrides: [],
                            },
                            variable_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::Variable(
                                                HirTypeTemplateVariable::Type {
                                                    attrs: HirTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: None,
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath(`quick_sort::quick_sort_aux`),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Mono,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Type {
                                    ident: `T`,
                                    traits: [
                                        HirTrait {
                                            trai_path: TraitPath(`core::cmp::Ord`),
                                            template_arguments: [],
                                        },
                                    ],
                                },
                            },
                        ],
                    ),
                ),
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::Form(
                            MajorFormPath(`quick_sort::partition`, `Ritchie(
                                Fn,
                            )`),
                        ),
                        hir_instantiation: HirInstantiation {
                            path: ItemPath(`quick_sort::partition`),
                            context: HirTypeContext {
                                comptime_var_overrides: [],
                            },
                            variable_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::Variable(
                                                HirTypeTemplateVariable::Type {
                                                    attrs: HirTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: None,
                        },
                    },
                    ValkyrieRide::PathLeading {
                        path: JavPath::Form(
                            MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                Fn,
                            )`),
                        ),
                        hir_instantiation: HirInstantiation {
                            path: ItemPath(`quick_sort::quick_sort_aux`),
                            context: HirTypeContext {
                                comptime_var_overrides: [],
                            },
                            variable_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::Variable(
                                                HirTypeTemplateVariable::Type {
                                                    attrs: HirTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: None,
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath(`quick_sort::partition`),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Mono,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Type {
                                    ident: `T`,
                                    traits: [
                                        HirTrait {
                                            trai_path: TraitPath(`core::cmp::Ord`),
                                            template_arguments: [],
                                        },
                                    ],
                                },
                            },
                        ],
                    ),
                ),
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::slice::Slice(0)::swap`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            path: ItemPath(`core::slice::Slice(0)::swap`),
                            context: HirTypeContext {
                                comptime_var_overrides: [],
                            },
                            variable_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::Variable(
                                                HirTypeTemplateVariable::Type {
                                                    attrs: HirTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVariable::Lifetime(
                                        SelfLifetime,
                                    ),
                                    HirTermSymbolicVariableResolution::SelfLifetime,
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath(`quick_sort::quick_sort_works_for_integers`),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [
                    ValkyrieRide::VecConstructor {
                        element_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                    },
                ],
            },
        ),
    ),
    (
        ItemPath(`quick_sort::quick_sort_works_for_integers::#test(0)`),
        None,
    ),
    (
        ItemPath(`quick_sort::quick_sort_works_for_strs`),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [
                    ValkyrieRide::VecConstructor {
                        element_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::mem::Ref`, `Extern`),
                                template_arguments: [
                                    HirTemplateArgument::Constant(
                                        HirCompterm::StaticLifetime,
                                    ),
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::str::str`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: false,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: false,
                            },
                        ),
                    },
                ],
            },
        ),
    ),
    (
        ItemPath(`quick_sort::quick_sort_works_for_strs::#test(0)`),
        None,
    ),
]
```