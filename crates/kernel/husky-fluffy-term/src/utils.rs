use super::*;

impl FluffyTerm {
    pub(crate) fn curry_destination(
        self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FluffyTerm {
        match self.data_inner(db, terms) {
            FluffyTermData::Literal(_)
            | FluffyTermData::TypeOntology { .. }
            | FluffyTermData::Hole(_, _)
            | FluffyTermData::Category(_) => self,
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
        }
    }

    /// this term as ty, what's its final destination?
    pub(crate) fn final_destination_inner(
        self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        match self.data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology { .. } => FinalDestination::TypeOntology,
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Hole(kind, idx) => {
                todo!()
                //     match kind {
                //     ImplicitSymbolKind::ImplicitLifetime => todo!(),
                //     ImplicitSymbolKind::ExprEvalLifetime => todo!(),
                //     ImplicitSymbolKind::UnspecifiedIntegerType
                //     | ImplicitSymbolKind::UnspecifiedFloatType
                //     | ImplicitSymbolKind::ImplicitType => FinalDestination::TypeOntology,
                // }
            }
            FluffyTermData::Category(_) => FinalDestination::Sort,
            FluffyTermData::Ritchie { .. } => todo!(),
        }
    }
}

fn curry_destination(db: &dyn TermDb, term: Term) -> Term {
    match term {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::Hole(_) => todo!(),
        Term::EntityPath(path) => match path {
            TermEntityPath::Form(_) => todo!(),
            TermEntityPath::Trait(_)
            | TermEntityPath::TypeOntology(_)
            | TermEntityPath::TypeConstructor(_) => term,
        },
        // EntityPath::Module(_) => todo!(),
        // EntityPath::ModuleItem(path) => match path {
        //     ModuleItemPath::Type(path) => resolved_term,
        //     ModuleItemPath::Trait(_) => todo!(),
        //     ModuleItemPath::Form(_) => todo!(),
        // },
        // EntityPath::AssociatedItem(_) => todo!(),
        // EntityPath::Variant(_) => todo!(),
        Term::Category(_) => term,
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Ritchie(_) => todo!(),
        Term::Abstraction(_) => todo!(),
        Term::Application(_) => term,
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}