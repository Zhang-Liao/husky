use super::*;
use thiserror::Error;

#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar, constructor = new_inner)]
pub struct PreciseTermSymbol {
    pub ty: PreciseTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl PreciseTermSymbol {
    pub fn from_raw(
        db: &dyn PreciseTermDb,
        raw_term: RawTermSymbol,
        raw_ty_expectation: TermTypeExpectation,
    ) -> PreciseTermResult<Self> {
        let ty = raw_term.ty(db)?;
        let ty = PreciseTerm::from_raw(db, ty, TermTypeExpectation::FinalDestinationEqsSort)?;
        Ok(Self::new_inner(db, ty, raw_term.idx(db)))
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_symbol(db, self, f)
    }
}

impl<Db: PreciseTermDb + ?Sized> salsa::DisplayWithDb<Db> for PreciseTermSymbol {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<PreciseTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!("${}", self.idx(db)))
    }
}