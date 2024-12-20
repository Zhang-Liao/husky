use super::*;
use crate::item_path_deps::item_sem_item_path_deps;
use ::graph_dynamics::{
    cycle_group::CycleGroup,
    deps::{IsGraphDepsContext, IsGraphDepsScheme},
};
use husky_entity_path::path::{ItemPath, ItemPathId};

#[salsa::interned(constructor = new)]
pub struct SemItemPathDepsCyclceGroupItd {
    #[return_ref]
    pub cycle_group: CycleGroup<SemItemPathDepsGraphDepsScheme>,
}

pub struct SemItemPathDepsGraphDepsScheme;

impl IsGraphDepsScheme for SemItemPathDepsGraphDepsScheme {
    type Node = ItemPath;

    const CYCLE_GROUP_N: usize = 4;

    type CycleGroupItd = SemItemPathDepsCyclceGroupItd;
}

#[derive(Clone, Copy)]
struct SemItemPathGraphDepsContext<'db> {
    db: &'db ::salsa::Db,
}

impl<'db> IsGraphDepsContext<'db> for SemItemPathGraphDepsContext<'db> {
    type Scheme = SemItemPathDepsGraphDepsScheme;

    fn deps_cropped(self, node: ItemPath) -> impl IntoIterator<Item = ItemPath> {
        item_sem_item_path_deps(self.db, *node)
            .as_ref()
            .unwrap()
            .iter()
            .copied()
    }

    fn full_deps_cropped(self, node: ItemPath) -> &'db [ItemPath] {
        item_sem_item_path_full_deps_cropped(self.db, *node)
    }

    fn cycle_group_itd(self, node: ItemPath) -> SemItemPathDepsCyclceGroupItd {
        item_sem_item_path_cycle_group_itd(self.db, *node)
    }
}

#[salsa::tracked(return_ref)]
pub fn item_sem_item_path_full_deps_cropped(
    db: &::salsa::Db,
    item_path_id: ItemPathId,
) -> Vec<ItemPath> {
    let ctx = SemItemPathGraphDepsContext { db };
    ctx.calc_full_deps_cropped(item_path_id.item_path(db))
}

#[test]
fn item_sem_item_path_full_deps_cropped_works() {
    use husky_entity_tree::node::ItemSynNodePath;

    DB::ast_rich_test_debug_with_db(
        item_sem_item_path_full_deps_cropped,
        &AstTestConfig::new(
            "item_sem_item_path_full_deps_cropped",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    );
}

#[salsa::tracked]
pub fn item_sem_item_path_cycle_group_itd(
    db: &::salsa::Db,
    node: ItemPathId,
) -> SemItemPathDepsCyclceGroupItd {
    let ctx = SemItemPathGraphDepsContext { db };
    let cycle_group = ctx.calc_cycle_group(node.item_path(db));
    SemItemPathDepsCyclceGroupItd::new(db, cycle_group)
}

#[test]
fn item_sem_item_path_cycle_group_itd_works() {
    use husky_entity_tree::node::ItemSynNodePath;

    DB::ast_rich_test_debug_with_db(
        item_sem_item_path_cycle_group_itd,
        &AstTestConfig::new(
            "item_sem_item_path_cycle_group_itd",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    );
}
