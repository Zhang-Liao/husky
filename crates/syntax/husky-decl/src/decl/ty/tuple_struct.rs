use super::*;
use husky_expr::ExprIdx;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub lpar: LeftParenthesisToken,
    #[return_ref]
    field_comma_list: (Vec<TupleStructFieldDeclPattern>, Vec<CommaToken>),
    pub rpar: RightParenthesisToken,
}

impl TupleStructTypeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }

    pub fn fields<'a>(self, db: &'a dyn DeclDb) -> &'a [TupleStructFieldDeclPattern] {
        &self.field_comma_list(db).0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TupleStructFieldDecl {
    ty: ExprIdx,
}