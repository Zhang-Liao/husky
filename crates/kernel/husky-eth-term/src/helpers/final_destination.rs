use husky_entity_kind::FugitiveKind;

use super::*;

impl EthTerm {
    pub fn final_destination(self, db: &::salsa::Db) -> FinalDestination {
        match self {
            EthTerm::Literal(_) => FinalDestination::AnyDerived,
            EthTerm::Symbol(_) | EthTerm::Rune(_) => FinalDestination::AnyOriginal,
            EthTerm::EntityPath(path) => match path {
                ItemPathTerm::MajorFugitive(path) => match path.fugitive_kind(db) {
                    FugitiveKind::TypeAlias => todo!(),
                    FugitiveKind::FunctionFn
                    | FugitiveKind::FunctionGn
                    | FugitiveKind::Val
                    | FugitiveKind::Formal
                    | FugitiveKind::Const => FinalDestination::AnyDerived,
                },
                ItemPathTerm::TypeOntology(_) => FinalDestination::TypeOntology,
                ItemPathTerm::Trait(_)
                | ItemPathTerm::TypeInstance(_)
                | ItemPathTerm::TypeVariant(_) => FinalDestination::AnyDerived,
            },
            EthTerm::Category(_) => FinalDestination::Sort,
            EthTerm::Universe(_) => unreachable!("expect ty term"),
            EthTerm::Curry(slf) => curry_ethereal_term_final_destination(db, slf),
            EthTerm::Ritchie(slf) => FinalDestination::Ritchie(slf.ritchie_kind(db)),
            EthTerm::Abstraction(_) => unreachable!("expect ty term"),
            EthTerm::Application(slf) => application_ethereal_term_final_destination(db, slf),
            EthTerm::TypeAsTraitItem(_) => FinalDestination::AnyOriginal,
            EthTerm::TraitConstraint(_) => FinalDestination::Sort,
        }
    }
}

#[salsa::tracked(jar = EthTermJar)]
fn application_ethereal_term_final_destination(
    db: &::salsa::Db,
    application: EthApplication,
) -> FinalDestination {
    application.function(db).final_destination(db)
}

#[salsa::tracked(jar = EthTermJar)]
fn curry_ethereal_term_final_destination(db: &::salsa::Db, curry: EthCurry) -> FinalDestination {
    curry.return_ty(db).final_destination(db)
}