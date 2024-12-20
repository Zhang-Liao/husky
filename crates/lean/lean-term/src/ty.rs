use crate::term::LnTerm;
use crate::*;
use eterned::db::EternerDb;
use lean_entity_path::LnItemPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LnType(LnTerm);

impl LnType {
    // TODO: check it??
    pub fn new_item_path(path: LnItemPath) -> Self {
        Self(LnTerm::new_item_path(path))
    }

    pub fn show(&self, db: &EternerDb) -> String {
        self.0.show(db)
    }
}

impl LnType {
    pub const UNIT: Self = Self(LnTerm::UNIT);
}
