use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct PropsVariantSynNodeDefn {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub syn_node_decl: PropsTypeVariantSynNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct PropsVariantSynDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: PropsTypeVariantSynDecl,
}