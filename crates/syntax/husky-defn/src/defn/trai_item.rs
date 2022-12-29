mod assoc_ty;
mod assoc_val;
mod function;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use method::*;

use crate::*;
use husky_entity_path::AssociatedItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TraitItemDefn {
    Function(TraitAssociatedFunctionDefn),
    Method(TraitMethodDefn),
    AlienType(TraitAssociatedTypeDefn),
    Value(TraitAssociatedValueDefn),
}

impl TraitItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> TraitItemDecl {
        todo!()
    }

    pub fn path(self, db: &dyn DefnDb) -> AssociatedItemPath {
        todo!()
    }
}
