mod props;
mod tuple;
mod unit;

pub use props::*;
pub use tuple::*;
pub use unit::*;

use super::*;

pub(crate) fn variant_signature(db: &dyn SignatureDb, decl: VariantDecl) -> VariantSignature {
    match decl {
        VariantDecl::Props(_) => todo!(),
        VariantDecl::Unit(_) => todo!(),
        VariantDecl::Tuple(_) => todo!(),
        // TypeDecl::Enum(decl) => enum_ty_signature(db, decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VariantSignature {
    Props(PropsVariantSignature),
    Unit(UnitVariantSignature),
    Tuple(TupleVariantSignature),
}
impl VariantSignature {}

impl<Db: SignatureDb + ?Sized> salsa::DebugWithDb<Db> for VariantSignature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}