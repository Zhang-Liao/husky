use super::*;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealTermAbstraction {
    x: EtherealTermPlaceholder,
    m: EtherealTerm,
}

#[test]
fn term_abstraction_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermAbstraction>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermAbstraction {
    pub(crate) fn from_raw_unchecked(
        db: &dyn EtherealTermDb,
        precise_term: RawTermAbstraction,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        todo!()
    }

    pub fn ty(&self) -> EtherealTerm {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl EtherealTermAbstraction {
    fn substitute(self, db: &dyn EtherealTermDb, substituation: &TermSubstitution) -> EtherealTerm {
        todo!()
    }
}

impl<Db: EtherealTermDb + ?Sized> salsa::DisplayWithDb<Db> for EtherealTermAbstraction {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        // use std::fmt::Write;
        // f.write_char(husky_unicode_symbols::greek::GREEK_LETTER_LOWERCASE_LAMBDA);
        // todo!()
        let db = <Db as salsa::DbWithJar<EtherealTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}