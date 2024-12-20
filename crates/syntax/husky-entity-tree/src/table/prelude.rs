use super::*;
use husky_entity_path::menu::item_path_menu;
use husky_manifest::HasManifest;
use husky_vfs::{jar::VfsDb, path::crate_path::CrateKind, toolchain::Toolchain};

#[derive(Debug, Clone, Copy)]
#[salsa::derive_debug_with_db]
pub struct CratePrelude<'a> {
    universal_prelude: Option<EntitySymbolTableRef<'a>>,
    crate_specific_symbol_context: EntitySymbolTableRef<'a>,
}

impl<'a> CratePrelude<'a> {
    pub(crate) fn new(db: &'a ::salsa::Db, crate_path: CratePath) -> PreludeResult<Self> {
        let crate_specific_symbol_context = crate_specific_symbol_context(db, crate_path)
            .as_ref()
            .map(|table| table.as_ref())
            .map_err(|e| e.clone())?;
        let universal_prelude =
            if crate_path == db.vfs_path_menu(crate_path.toolchain(db)).core_library() {
                None
            } else {
                Some(none_core_crate_universal_prelude(db, crate_path.toolchain(db)).as_ref())
            };
        Ok(Self {
            universal_prelude,
            crate_specific_symbol_context,
        })
    }

    pub(crate) fn resolve_ident(
        &self,
        db: &::salsa::Db,
        reference_module_path: ModulePath,
        ident: Ident,
    ) -> Option<EntitySymbol> {
        self.crate_specific_symbol_context
            .resolve_ident(db, reference_module_path.into(), ident)
            .or_else(|| {
                self.universal_prelude?
                    .resolve_ident(db, reference_module_path.into(), ident)
            })
    }
}

#[salsa::tracked(return_ref)]
pub(crate) fn none_core_crate_universal_prelude(
    db: &::salsa::Db,
    toolchain: Toolchain,
) -> EntitySymbolTable {
    let vfs_path_menu = db.vfs_path_menu(toolchain);
    let _item_path_menu = item_path_menu(db, toolchain);
    let coword_menu = coword_menu(db);
    let core_prelude_module = vfs_path_menu.core_prelude().inner();
    let mut table = EntitySymbolTable::default();
    table
        .push(EntitySymbolEntry {
            ident: coword_menu.core_ident(),
            visible_scope: Scope::Pub,
            symbol: EntitySymbol::UniversalPrelude {
                item_path: vfs_path_menu.core_root().into(),
            },
        })
        .unwrap();
    table
        .extend(
            item_tree_sheet(db, core_prelude_module)
                .module_symbols()
                .data()
                .iter()
                .map(|entry| EntitySymbolEntry {
                    ident: entry.ident,
                    visible_scope: Scope::Pub,
                    symbol: EntitySymbol::UniversalPrelude {
                        item_path: entry.symbol.principal_entity_path(db),
                    },
                }),
        )
        .unwrap();
    table
}

#[salsa::tracked(return_ref)]
fn crate_specific_symbol_context(
    db: &::salsa::Db,
    crate_path: CratePath,
) -> PreludeResult<EntitySymbolTable> {
    let package_path = crate_path.package_path(db);
    let package_dependencies = package_path.dependencies(db)?;
    let mut entries: EntitySymbolTable = Default::default();
    entries
        .push(EntitySymbolEntry::new_crate_root(db, crate_path))
        .unwrap();
    match crate_path.kind(db) {
        CrateKind::Lib | CrateKind::Main => (),
        _ => {
            let Some(lib_crate_path) = package_path.lib_crate_path(db) else {
                todo!()
            };
            entries
                .push(EntitySymbolEntry::new_self_lib(
                    db,
                    lib_crate_path,
                    crate_path,
                ))
                .unwrap();
        }
    }
    for package_dependency in package_dependencies {
        entries
            .push(EntitySymbolEntry::new_package_dependency(
                db,
                package_dependency,
            )?)
            .unwrap();
    }
    Ok(entries)
}
