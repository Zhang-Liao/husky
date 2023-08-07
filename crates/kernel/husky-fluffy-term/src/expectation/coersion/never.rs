use super::*;

impl ExpectCoersion {
    pub(super) fn resolve_never(
        &self,
        db: &dyn FluffyTermDb,
        fluffy_terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<ExpectationEffect> {
        match state.expectee().ty_data_inner(db, fluffy_terms) {
            // never can be coersed to any type
            (
                _,
                FluffyBaseTypeData::TypeOntology {
                    refined_ty_path: Left(PreludeTypePath::NEVER),
                    ..
                },
            ) => state.set_ok(Coersion::Never, smallvec![]),
            _ => AltNone,
        }
    }
}