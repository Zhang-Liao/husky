use super::*;
use husky_declarative_signature::InductiveTypeDeclarativeSignatureTemplate;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct InductiveTypeEtherealSignatureTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
}

impl InductiveTypeEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TypePath,
        declarative_signature_template: InductiveTypeDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters = EtherealTermTemplateParameters::from_declarative(
            db,
            declarative_signature_template.template_parameters(db),
        )?;
        Ok(Self::new(db, path, template_parameters))
    }
}
