mod r#fn;
mod gn;
mod ty_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ty_alias::*;
pub use self::val::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum FugitiveDeclarativeSignatureTemplate {
    FunctionFn(FnFugitiveDeclarativeSignatureTemplate),
    FunctionGn(GnFugitiveDeclarativeSignatureTemplate),
    TypeAlias(TypeAliasFugitiveDeclarativeSignatureTemplate),
    Val(ValFugitiveDeclarativeSignatureTemplate),
}

impl FugitiveDeclarativeSignatureTemplate {
    pub fn template_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[DeclarativeTemplateParameter] {
        match self {
            FugitiveDeclarativeSignatureTemplate::FunctionFn(decl) => decl.template_parameters(db),
            FugitiveDeclarativeSignatureTemplate::Val(decl) => decl.template_parameters(db),
            FugitiveDeclarativeSignatureTemplate::FunctionGn(decl) => decl.template_parameters(db),
            FugitiveDeclarativeSignatureTemplate::TypeAlias(_) => todo!(),
        }
    }
}

impl HasDeclarativeSignatureTemplate for FugitivePath {
    type DeclarativeSignatureTemplate = FugitiveDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        fugitive_syn_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn fugitive_syn_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    path: FugitivePath,
) -> DeclarativeSignatureResult<FugitiveDeclarativeSignatureTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        FugitiveSynDecl::FunctionFn(decl) => {
            FnFugitiveDeclarativeSignatureTemplate::from_decl(db, decl).map(Into::into)
        }
        FugitiveSynDecl::Val(decl) => {
            ValFugitiveDeclarativeSignatureTemplate::from_decl(db, decl).map(Into::into)
        }
        FugitiveSynDecl::FunctionGn(decl) => {
            GnFugitiveDeclarativeSignatureTemplate::from_decl(db, decl).map(Into::into)
        }
    }
}
