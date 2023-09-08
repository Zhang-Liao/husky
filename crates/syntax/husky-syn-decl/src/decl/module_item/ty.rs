mod r#enum;
mod r#extern;
mod inductive;
mod props_struct;
mod record;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::record::*;
pub use self::structure::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use super::*;
use husky_entity_taxonomy::{EntityKind, TypeKind};
use parsec::parse_separated_list2;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum TypeSynNodeDecl {
    Enum(EnumTypeSynNodeDecl),
    PropsStruct(PropsStructTypeSynNodeDecl),
    UnitStruct(UnitStructTypeSynNodeDecl),
    TupleStruct(TupleStructTypeSynNodeDecl),
    Record(RecordTypeSynNodeDecl),
    Inductive(InductiveTypeSynNodeDecl),
    Structure(StructureTypeSynNodeDecl),
    Extern(ExternTypeSynNodeDecl),
    Union(UnionTypeSynNodeDecl),
}

impl TypeSynNodeDecl {
    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> TypeSynNodePath {
        match self {
            TypeSynNodeDecl::Enum(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::Inductive(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::Record(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::UnitStruct(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::PropsStruct(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::TupleStruct(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::Structure(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::Extern(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeSynNodeDecl::Union(syn_node_decl) => syn_node_decl.syn_node_path(db),
        }
    }

    pub fn ast_idx(self, db: &dyn SynDeclDb) -> AstIdx {
        match self {
            TypeSynNodeDecl::Enum(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeSynNodeDecl::UnitStruct(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeSynNodeDecl::TupleStruct(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeSynNodeDecl::PropsStruct(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeSynNodeDecl::Record(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeSynNodeDecl::Inductive(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeSynNodeDecl::Structure(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeSynNodeDecl::Extern(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeSynNodeDecl::Union(syn_node_decl) => syn_node_decl.ast_idx(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            TypeSynNodeDecl::Enum(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::UnitStruct(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::TupleStruct(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::PropsStruct(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::Record(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::Inductive(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::Structure(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::Extern(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeSynNodeDecl::Union(syn_node_decl) => syn_node_decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        match self {
            TypeSynNodeDecl::Enum(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::PropsStruct(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::UnitStruct(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::TupleStruct(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::Record(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::Inductive(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::Structure(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::Extern(syn_node_decl) => syn_node_decl.errors(db),
            TypeSynNodeDecl::Union(syn_node_decl) => syn_node_decl.errors(db),
        }
    }
}

impl HasSynNodeDecl for TypeSynNodePath {
    type NodeDecl = TypeSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        ty_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_node_decl(db: &dyn SynDeclDb, syn_node_path: TypeSynNodePath) -> TypeSynNodeDecl {
    let ctx = DeclParserFactory::new(db, syn_node_path);
    ctx.parse_ty_node_decl(syn_node_path)
}

impl<'a> DeclParserFactory<'a> {
    fn parse_ty_node_decl(&self, syn_node_path: TypeSynNodePath) -> TypeSynNodeDecl {
        let db = self.db();
        let node = syn_node_path.node(db);
        let ast_idx: AstIdx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Identifiable {
                token_group_idx,
                block: DefnBlock::Type { path, variants },
                item_kind,
                saved_stream_state,
                ..
            } => self.parse_ty_node_decl_aux(
                syn_node_path,
                ast_idx,
                path.ty_kind(self.db()),
                item_kind,
                token_group_idx,
                variants,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_ty_node_decl_aux(
        &self,
        syn_node_path: TypeSynNodePath,
        ast_idx: AstIdx,
        type_kind: TypeKind,
        _item_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        variants: Option<TypeVariants>,
        saved_stream_state: TokenStreamState,
    ) -> TypeSynNodeDecl {
        match type_kind {
            TypeKind::Enum => self
                .parse_enum_ty_node_decl(
                    syn_node_path,
                    ast_idx,
                    token_group_idx,
                    variants.expect("guaranteed by `husky-ast`"),
                    saved_stream_state,
                )
                .into(),
            TypeKind::Inductive => self
                .parse_inductive_ty_node_decl(
                    ast_idx,
                    syn_node_path,
                    token_group_idx,
                    variants.expect("guaranteed by `husky-ast`"),
                    saved_stream_state,
                )
                .into(),
            TypeKind::Record => todo!(),
            TypeKind::Struct => {
                debug_assert!(variants.is_none());
                self.parse_struct_ty_node_decl(
                    syn_node_path,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
            }
            TypeKind::Structure => {
                debug_assert!(variants.is_none());
                self.parse_structure_ty_node_decl(
                    syn_node_path,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
            }
            TypeKind::Extern => {
                debug_assert!(variants.is_none());
                self.parse_extern_ty_node_decl(
                    syn_node_path,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
                .into()
            }
        }
    }
}

impl<'a> DeclParserFactory<'a> {
    pub(super) fn parse_struct_ty_node_decl(
        &self,
        syn_node_path: TypeSynNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TypeSynNodeDecl {
        let db = self.db();
        let mut parser = self.expr_parser(
            syn_node_path,
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let template_parameters = ctx.try_parse_option();
        if let Some(lpar) = ctx.try_parse_err_as_none::<LparToken>() {
            let field_comma_list = ctx.try_parse();
            let rpar = ctx.try_parse();
            TupleStructTypeSynNodeDecl::new(
                db,
                syn_node_path,
                ast_idx,
                template_parameters,
                lpar,
                field_comma_list,
                rpar,
                parser.finish(),
            )
            .into()
        } else if let Some(semicolon) = ctx.try_parse_err_as_none::<SemiColonToken>() {
            todo!()
            // Err(OriginalDeclError::ExpectedLCurlOrLParOrSemicolon(ctx.save_state()).into())
        } else {
            let lcurl = ctx.try_parse();
            let field_comma_list = ctx.try_parse();
            let rcurl = ctx.try_parse();
            PropsStructTypeSynNodeDecl::new(
                db,
                syn_node_path,
                ast_idx,
                template_parameters,
                lcurl,
                field_comma_list,
                rcurl,
                parser.finish(),
            )
            .into()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum TypeSynDecl {
    Enum(EnumTypeSynDecl),
    PropsStruct(PropsStructTypeSynDecl),
    UnitStruct(UnitStructTypeSynDecl),
    TupleStruct(TupleStructTypeSynDecl),
    Record(RecordTypeSynDecl),
    Inductive(InductiveTypeSynDecl),
    Structure(StructureTypeSynDecl),
    Extern(ExternTypeSynDecl),
    Union(UnionTypeSynDecl),
}

impl TypeSynDecl {
    pub fn path(self, db: &dyn SynDeclDb) -> TypePath {
        match self {
            TypeSynDecl::Enum(decl) => decl.path(db),
            TypeSynDecl::Inductive(decl) => decl.path(db),
            TypeSynDecl::Record(decl) => decl.path(db),
            TypeSynDecl::UnitStruct(decl) => decl.path(db),
            TypeSynDecl::PropsStruct(decl) => decl.path(db),
            TypeSynDecl::TupleStruct(decl) => decl.path(db),
            TypeSynDecl::Structure(decl) => decl.path(db),
            TypeSynDecl::Extern(decl) => decl.path(db),
            TypeSynDecl::Union(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [TemplateParameterObelisk] {
        match self {
            TypeSynDecl::Enum(decl) => decl.template_parameters(db),
            TypeSynDecl::UnitStruct(decl) => decl.template_parameters(db),
            TypeSynDecl::TupleStruct(decl) => decl.template_parameters(db),
            TypeSynDecl::PropsStruct(decl) => decl.template_parameters(db),
            TypeSynDecl::Record(decl) => decl.template_parameters(db),
            TypeSynDecl::Inductive(decl) => decl.template_parameters(db),
            TypeSynDecl::Structure(decl) => decl.template_parameters(db),
            TypeSynDecl::Extern(decl) => decl.template_parameters(db),
            TypeSynDecl::Union(decl) => decl.template_parameters(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            TypeSynDecl::Enum(decl) => decl.syn_expr_region(db),
            TypeSynDecl::UnitStruct(decl) => decl.syn_expr_region(db),
            TypeSynDecl::TupleStruct(decl) => decl.syn_expr_region(db),
            TypeSynDecl::PropsStruct(decl) => decl.syn_expr_region(db),
            TypeSynDecl::Record(decl) => decl.syn_expr_region(db),
            TypeSynDecl::Inductive(decl) => decl.syn_expr_region(db),
            TypeSynDecl::Structure(decl) => decl.syn_expr_region(db),
            TypeSynDecl::Extern(decl) => decl.syn_expr_region(db),
            TypeSynDecl::Union(decl) => decl.syn_expr_region(db),
        }
    }

    #[inline(always)]
    fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: TypeSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            TypeSynNodeDecl::Enum(syn_node_decl) => {
                EnumTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::PropsStruct(syn_node_decl) => {
                PropsStructTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::UnitStruct(syn_node_decl) => {
                UnitStructTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::TupleStruct(syn_node_decl) => {
                TupleStructTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::Record(syn_node_decl) => {
                RecordTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::Inductive(syn_node_decl) => {
                InductiveTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::Structure(syn_node_decl) => {
                StructureTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::Extern(syn_node_decl) => {
                ExternTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TypeSynNodeDecl::Union(syn_node_decl) => {
                UnionTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
        })
    }
}

impl HasSynDecl for TypePath {
    type Decl = TypeSynDecl;

    #[inline(always)]
    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        ty_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_decl(db: &dyn SynDeclDb, path: TypePath) -> DeclResult<TypeSynDecl> {
    TypeSynDecl::from_node_decl(db, path, path.syn_node_path(db).syn_node_decl(db))
}

#[test]
fn ty_decl_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let menu = db.item_path_menu(toolchain);
    assert!(menu.never_ty_path().syn_decl(&db).is_ok());
}
