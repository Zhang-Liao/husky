mod trai_method;
mod ty_method;

use super::*;
use husky_entity_path::path::major_item::trai::TraitPath;

impl SolTerm {
    pub(super) fn method_dispatch_aux(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident: Ident,
        available_traits: &[TraitPath],
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<MethodFlyInstanceDispatch> {
        match self.data(engine) {
            SolTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolTermData::Curry { .. } | SolTermData::Ritchie { .. } => Nothing,
        }
    }
}
