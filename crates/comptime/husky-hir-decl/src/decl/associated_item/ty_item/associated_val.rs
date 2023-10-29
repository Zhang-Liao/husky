use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAssociatedValHirDecl {
    pub path: TypeItemPath,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TypeAssociatedValHirDecl {
    pub(super) fn from_ethereal(
        path: TypeItemPath,
        ethereal_signature_template: TypeMemoizedFieldEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> (Self, HirEagerExprSourceMap) {
        todo!()
        // let mut builder = HirEagerExprBuilder::new(db, syn_decl.expr_region(db));
        // let return_ty = HirType::from_ethereal(ethereal_signature_template.return_ty(db), db);
        // let hir_expr_region = builder.finish();
        // Self::new(db, path, return_ty, hir_expr_region)
    }
}
