use super::*;

pub(super) fn generate_library(_target_crate: CratePath, _db: &::salsa::Db) -> BootLibraryStorage {
    BootLibraryStorage {}
}
