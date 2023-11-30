use crate::*;

#[salsa::jar(db = LinkageDb)]
pub struct LinkageJar(
    crate::linkage::Linkage,
    crate::dependency::linkage_path_dependencies,
    crate::amazon::package_amazon_linkages,
    crate::version_stamp::LinkageVersionStamp,
    crate::template_argument::ty::LinkageTypePathLeading,
    crate::template_argument::ty::LinkageTypeRitchie,
    crate::pantheon::package_valkyrie_linkage_pantheon,
    crate::valkyrie::item_valkyrie_rides,
    crate::valkyrie::linkage_valkyrie_linkages,
);
