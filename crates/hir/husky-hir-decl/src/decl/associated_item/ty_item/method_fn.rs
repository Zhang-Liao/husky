use super::*;
use husky_hir_ty::ritchie::HirRitchieParameter;
use husky_syn_decl::TypeMethodFnSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMethodFnHirDecl {
    pub path: TypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub self_value_parameter: HirParenateParameter,
    #[return_ref]
    pub parenate_parameters: HirParenateParameters,
    pub return_ty: HirType,
}

impl TypeMethodFnHirDecl {
    pub(super) fn from_syn(
        path: TypeItemPath,
        syn_decl: TypeMethodFnSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let self_value_parameter = HirParenateParameter::from_self_value_parameter_syndicate(
            syn_decl.self_value_parameter(db),
            db,
        );
        let parenate_parameters =
            HirParenateParameters::from_syn(syn_decl.parenate_parameters(db), db);
        let return_ty = syn_decl
            .return_ty(db)
            .map(|syndicate| builder.hir_ty(syndicate.syn_expr_idx()))
            .unwrap_or(builder.hir_ty_menu().unit_ty().into());
        TypeMethodFnHirDecl::new(
            db,
            path,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        )
    }
}