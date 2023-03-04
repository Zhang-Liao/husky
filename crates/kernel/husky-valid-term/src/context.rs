mod entry;

pub(crate) use entry::is_ty_path_lifetime_ty;

use crate::*;
use entry::*;
use husky_entity_path::EntityPath;
use vec_like::{VecMap, VecPairMap, VecSet};

#[derive(Default)]
pub(crate) struct ValidTermShowContext {
    entries: VecMap<ValidTermSymbolShowEntry>,
}

impl ValidTermShowContext {
    pub(crate) fn fmt_symbol(
        &mut self,
        db: &dyn ValidTermDb,
        symbol: ValidTermSymbol,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(entry) = self.entries.get_entry(symbol) {
            entry.show(db, f)
        } else {
            let new_entry = self.new_external_entry(db, symbol, None);
            new_entry.show(db, f);
            self.entries.insert_new(new_entry).unwrap();
            Ok(())
        }
    }

    pub(crate) fn fmt_with_symbol(
        &mut self,
        db: &dyn ValidTermDb,
        symbol: ValidTermSymbol,
        f: impl FnOnce(&mut Self) -> std::fmt::Result,
    ) -> std::fmt::Result {
        self.enter_block(db, symbol);
        f(self)?;
        self.exit_block(symbol);
        Ok(())
    }
}
