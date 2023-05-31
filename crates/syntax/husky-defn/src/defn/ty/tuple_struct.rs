use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TupleStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: TupleStructTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn tuple_struct_ty_defn(
    db: &dyn DefnDb,
    decl: TupleStructTypeDecl,
) -> TupleStructTypeDefn {
    let path = decl.path(db);
    TupleStructTypeDefn::new(db, path, decl)
}