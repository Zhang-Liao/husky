pub(crate) use husky_linket::test_utils::TestLinket;
pub(crate) use husky_vfs::test_utils::*;

use husky_corgi_config::jar::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::jar::CowordJar;
use husky_dec_signature::jar::DecSignatureJar;
use husky_entity_tree::jar::EntityTreeJar;
use husky_eth_signature::jar::EthSignatureJar;
use husky_eth_term::jar::EthTermJar;
use husky_fly_term::jar::FlyTermJar;
use husky_manifest::jar::ManifestJar;
use husky_manifest_ast::jar::ManifestAstJar;
use husky_sem_expr::SemExprJar;
use husky_syn_decl::jar::SynDeclJar;
use husky_syn_defn::jar::SynDefnJar;
use husky_syn_expr::jar::SynExprJar;
use husky_term_prelude::jar::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;

#[salsa::db(
    CowordJar,
    husky_vfs::jar::VfsJar,
    husky_entity_path::jar::EntityPathJar,
    husky_text::jar::TextJar,
    husky_token_data::jar::TokenDataJar,
    TokenJar,
    husky_ast::jar::AstJar,
    EntityTreeJar,
    husky_toml_token::jar::TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    SynExprJar,
    SynDefnJar,
    SynDeclJar,
    TermPreludeJar,
    husky_dec_term::jar::DecTermJar,
    DecSignatureJar,
    husky_dec_ty::jar::DecTypeJar,
    EthTermJar,
    EthSignatureJar,
    FlyTermJar,
    husky_place::jar::PlaceJar,
    SemExprJar,
    husky_sem_place_contract::jar::SemPlaceContractJar,
    husky_sem_item_path_deps::jar::SemItemPathDepsJar,
    husky_sem_static_mut_deps::jar::SemStaticMutDepsJar,
    husky_sem_var_deps::jar::SemVarDepsJar,
    husky_hir_prelude::jar::HirPreludeJar,
    husky_hir_ty::jar::HirTypeJar,
    husky_hir_eager_expr::jar::HirEagerExprJar,
    husky_hir_lazy_expr::jar::HirLazyExprJar,
    husky_hir_expr::jar::HirExprJar,
    husky_hir_decl::jar::HirDeclJar,
    husky_hir_defn::jar::HirDefnJar,
    husky_javelin::jar::JavelinJar,
    husky_ki::jar::KiJar,
    husky_ki_repr::jar::KiReprJar,
    husky_linket::jar::LinketJar,
    husky_vmir::jar::VmirJar
)]
#[derive(Default)]
pub(crate) struct DB;

// #[test]
// #[ignore]
// fn run_test_linket_works() {
//     DB::vfs_plain_test(
//         |db, test_linket: TestLinket| run_test_linket(test_linket, db),
//         &VfsTestConfig::new(
//             "run_test_linket_works",
//             FileExtensionConfig::Markdown,
//             TestDomainsConfig::TOML,
//         ),
//     );
// }

// #[cfg(test)]
// fn run_test_linket(test_linket: TestLinket, db: &::salsa::Db) {
//     use husky_virtual_linktime::VirtualLinktime;
//     use husky_vmir::storage::VirtualVmirStorage;

//     use crate::{eval::eval_linket_on_arguments, vm::VmMode};

//     let linktime = VirtualLinktime;
//     let vmir_storage = VirtualVmirStorage;
//     eval_linket_on_arguments(
//         *test_linket,
//         vec![],
//         VmMode::Quick,
//         db,
//         &linktime,
//         &vmir_storage,
//     );
// }