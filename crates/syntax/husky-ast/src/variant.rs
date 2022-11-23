use husky_entity_path::EntityPath;
use husky_source_path::SourcePath;
use husky_token::Paradigm;

use crate::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DeprecatedAstVariant {
    TypeDefnHead {
        ident: RangedIdentifier,
        kind: TyKind,
        spatial_parameters: IdentMap<SpatialParameter>,
    },
    MainDefnHead,
    CallFormDefnHead {
        ident: RangedIdentifier,
        paradigm: Paradigm,
        spatial_parameters: IdentMap<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        return_ty: ExprIdx,
        // output_liason: OutputModifier,
        // opt_this_liason: Option<ParameterModifier>,
    },
    FeatureDefnHead {
        paradigm: Paradigm,
        ident: RangedIdentifier,
        return_ty: ExprIdx,
    },
    FieldDefnHead {
        // liason: MemberModifier,
        ranged_ident: RangedIdentifier,
        field_ty: ExprIdx,
        ast_field_kind: AstFieldKind,
    },
    DatasetConfigDefnHead,
    Stmt(RawStmt),
    EnumVariantDefnHead {
        ident: RangedIdentifier,
        variant_class: EnumVariantKind,
    },
    Use {
        use_variant: UseVariant,
    },
    Submodule {
        ident: RangedIdentifier,
        source_file: SourcePath,
    },
    Visual,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UseVariant {
    Route { entity_path: EntityPath },
    All { parent: Ty },
}

impl From<RawStmt> for DeprecatedAst {
    fn from(stmt: RawStmt) -> Self {
        Self {
            range: stmt.range,
            variant: DeprecatedAstVariant::Stmt(stmt),
        }
    }
}

impl From<RawStmt> for DeprecatedAstVariant {
    fn from(stmt: RawStmt) -> Self {
        DeprecatedAstVariant::Stmt(stmt)
    }
}
