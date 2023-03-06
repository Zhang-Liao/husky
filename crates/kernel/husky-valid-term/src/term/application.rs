use super::*;
use std::fmt::{Debug, Display};

/// in husky, application is generalized to include composition as a special case;
///
/// when shift is `0`, this is the normal application;
///
/// when shift is `1`, this is composition,
///
/// in general when shift is `n`, this is equavalent to
///
/// use abstraction `n` times, and then apply original argument to them,
///
/// then apply function to the result,
///
/// `\x1 ... \xn -> $function ($argument \x1 ... \xn)`
#[salsa::interned(db = ValidTermDb, jar = ValidTermJar, constructor = new_inner)]
pub struct ValidTermApplication {
    pub function: ValidTerm,
    pub argument: ValidTerm,
    pub shift: u8,
}

impl ValidTermApplication {
    pub fn new(
        db: &dyn ValidTermDb,
        function: ValidTerm,
        argument: ValidTerm,
        shift: u8,
    ) -> ValidTermResult<Self> {
        todo!()
    }

    pub fn from_precise(
        db: &dyn ValidTermDb,
        precise_term: PreciseTermApplication,
    ) -> ValidTermResult<Self> {
        valid_term_application_from_precise(db, precise_term)
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        self.function(db).show_with_db_fmt(f, db, ctx)?;
        f.write_str(" ")?;
        self.argument(db).show_with_db_fmt(f, db, ctx)
    }
}

#[salsa::tracked(jar = ValidTermJar)]
pub(crate) fn valid_term_application_from_precise(
    db: &dyn ValidTermDb,
    precise_term: PreciseTermApplication,
) -> ValidTermResult<ValidTermApplication> {
    let function = ValidTerm::from_precise(db, precise_term.function(db))?;
    let argument = ValidTerm::from_precise(db, precise_term.argument(db))?;
    let shift = precise_term.shift(db);
    check_application_validity(db, function, argument, shift)?;
    Ok(ValidTermApplication::new_inner(
        db, function, argument, shift,
    ))
}

fn check_application_validity(
    db: &dyn ValidTermDb,
    function: ValidTerm,
    argument: ValidTerm,
    shift: u8,
) -> ValidTermResult<()> {
    let function_precise_ty = match function.precise_ty(db)? {
        Left(PreciseTerm::Curry(function_precise_ty)) => function_precise_ty,
        _ => unreachable!(),
    };
    let argument_precise_ty = argument.precise_ty(db)?;
    p!(function_precise_ty.debug(db), argument_precise_ty.debug(db));
    todo!();
    Ok(())
}

impl<Db: ValidTermDb + ?Sized> salsa::DisplayWithDb<Db> for ValidTermApplication {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<ValidTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl ValidTermApplication {
    pub fn ty_itd(&self) -> Option<ValidTerm> {
        // TODO: delete this
        None
    }
}

impl ValidTermRewriteCopy for ValidTermApplication {
    fn substitute_copy(self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self
    where
        Self: Copy,
    {
        let old_m = self.function(db);
        let m = old_m.substitute_copy(db, substituation);
        let old_n = self.argument(db);
        let n = old_n.substitute_copy(db, substituation);
        if old_m == m && old_n == n {
            return self;
        }
        ValidTermApplication::new(db, m, n, self.shift(db))
            .expect("substitution shouldn't return Err")
    }
}

impl std::fmt::Display for ValidTermApplication {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}