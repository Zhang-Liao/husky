use husky_vfs::CratePath;
use vec_like::VecSet;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VarianceId {
    path: EntityPath,
    idx: u8,
}

impl VarianceId {
    pub(super) fn new(
        db: &dyn DeclarativeTypeDb,
        crate_path: CratePath,
        path: EntityPath,
        idx: u8,
    ) -> Option<Self> {
        (crate_path == path.crate_path(db)).then_some(Self { path, idx })
    }
}

pub(crate) fn entity_variance_crate_dependencies(
    db: &dyn DeclarativeTypeDb,
    id: VarianceId,
) -> VarianceResultRef<&[VarianceId]> {
    let _declarative_term_menu = db
        .declarative_term_menu(id.path.toolchain(db))
        .as_ref()
        .unwrap();
    match id.path {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => {
                declarative_ty_entity_variance_crate_dependencies(db, path, id.idx)
                    .as_ref()
                    .map(|t| t.as_ref())
            }
            ModuleItemPath::Trait(path) => {
                trai_entity_variance_crate_dependencies(db, path, id.idx)
                    .as_ref()
                    .map(|t| t.as_ref())
            }
            ModuleItemPath::Form(path) => form_entity_variance_crate_dependencies(db, path, id.idx)
                .as_ref()
                .map(|t| t.as_ref()),
        },
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::TypeVariant(_) => todo!(),
    }
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn declarative_ty_entity_variance_crate_dependencies(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
    _idx: u8,
) -> VarianceResult<VecSet<VarianceId>> {
    let _declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let decl = match path.decl(db) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedVarianceError::DeclError.into()),
    };
    let signature = match db.ty_declarative_signature_template(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    match signature {
        TypeDeclarativeSignatureTemplate::Enum(_) => todo!(),
        TypeDeclarativeSignatureTemplate::RegularStruct(_) => todo!(),
        TypeDeclarativeSignatureTemplate::UnitStruct(_) => todo!(),
        TypeDeclarativeSignatureTemplate::TupleStruct(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Record(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Inductive(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Structure(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Extern(_) => (),
        TypeDeclarativeSignatureTemplate::Union(_) => todo!(),
    }
    todo!()
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn trai_entity_variance_crate_dependencies(
    _db: &dyn DeclarativeTypeDb,
    _path: TraitPath,
    _idx: u8,
) -> VarianceResult<VecSet<VarianceId>> {
    todo!()
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn form_entity_variance_crate_dependencies(
    db: &dyn DeclarativeTypeDb,
    path: FugitivePath,
    _idx: u8,
) -> VarianceResult<VecSet<VarianceId>> {
    let decl = match path.decl(db) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedVarianceError::DeclError.into()),
    };
    let _signature = match decl.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    todo!()
}