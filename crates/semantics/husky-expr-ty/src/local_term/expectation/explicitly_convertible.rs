use super::*;

/// expect a type that is explicitly convertible to dst
#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectExplicitlyConvertible {
    pub(crate) destination: LocalTerm,
}

impl ExpectExplicitlyConvertible {
    pub(in super::super) fn try_substitute_unresolved_local_term<'a>(
        &self,
        unresolved_terms: &'a UnresolvedTerms,
    ) -> Result<Option<LocalTermExpectation>, &'a LocalTermResolveError> {
        match unresolved_terms.try_reduce_local_term(self.destination)? {
            Some(destination) => Ok(Some(ExpectExplicitlyConvertible { destination }.into())),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectExplicitlyConvertibleOutcome {
    destination: LocalTerm,
}

impl ExpectLocalTerm for ExpectExplicitlyConvertible {
    type Outcome = ExpectExplicitlyConvertibleOutcome;

    fn retrieve_outcome(outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            LocalTermExpectationOutcome::ExplicitlyConvertible(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        todo!()
    }

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.destination)
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_explicitly_convertible(
        &self,
        expectee: LocalTerm,
        expectation: &ExpectExplicitlyConvertible,
        level: LocalTermResolveLevel,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        // todo
        None
    }
}