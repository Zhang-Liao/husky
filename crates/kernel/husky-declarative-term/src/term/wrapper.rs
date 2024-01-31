use super::*;

/// wrappers are special applications
///
/// we treat them separately because we need to apply special reduction and avoid toolchain
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar, constructor = new)]
pub struct WrapperDeclarativeTerm {
    pub kind: DeclarativeTermWrapperKind,
    pub inner_ty: DeclarativeTerm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclarativeTermWrapperKind {
    ValReturnType,
}

impl DeclarativeTerm {
    pub fn leashed_ty(self, db: &::salsa::Db) -> Self {
        WrapperDeclarativeTerm::new(db, DeclarativeTermWrapperKind::ValReturnType, self).into()
    }
}

impl WrapperDeclarativeTerm {
    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        match self.kind(db) {
            DeclarativeTermWrapperKind::ValReturnType => f.write_str("{val_type} ")?,
        }
        self.inner_ty(db).show_with_db_fmt(f, db, ctx)
    }
}
