use super::*;

pub(super) fn ty_memo_ty(
    db: &dyn EtherealTypeDb,
    term: EtherealTerm,
    ident: Ident,
) -> TypeResult<Option<(TypeMemoDisambiguation, TypeResult<EtherealTerm>)>> {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeMemoDisambiguation {}