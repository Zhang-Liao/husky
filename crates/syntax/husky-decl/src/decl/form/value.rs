use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct ValueDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
}

impl ValueDecl {
    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterDecl] { &l })
            .unwrap_or(&[])
    }
}