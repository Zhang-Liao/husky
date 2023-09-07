use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct ValFugitiveEtherealSignatureTemplate {
    pub path: FugitivePath,
}

impl ValFugitiveEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: FugitivePath,
        declarative_signature_template: ValFugitiveDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self::new(db, path))
    }
}