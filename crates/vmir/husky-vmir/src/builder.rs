use crate::{
    expr::{VmirExprArena, VmirExprData, VmirExprIdx, VmirExprIdxRange},
    stmt::{VmirStmtArena, VmirStmtData, VmirStmtIdx, VmirStmtIdxRange},
    vmir::VmirData,
};
use husky_coword::Ident;
use husky_hir_eager_expr::{HirEagerExprArena, HirEagerStmtArena};
use husky_linkage::instantiation::LinInstantiation;

pub(crate) struct VmirExprBuilder<'db> {
    db: &'db ::salsa::Db,
    hir_eager_expr_arena: &'db HirEagerExprArena,
    hir_eager_stmt_arena: &'db HirEagerStmtArena,
    instantiation: &'db LinInstantiation,
    vmir_expr_arena: VmirExprArena,
    vmir_stmt_arena: VmirStmtArena,
    buffer: Vec<VmirData>,
    variables: Vec<Ident>,
}

impl<'db> VmirExprBuilder<'db> {
    pub(crate) fn hir_eager_expr_arena(&self) -> &'db HirEagerExprArena {
        self.hir_eager_expr_arena
    }

    pub(crate) fn hir_eager_stmt_arena(&self) -> &'db HirEagerStmtArena {
        self.hir_eager_stmt_arena
    }

    pub(crate) fn alloc_expr(&mut self, expr_data: VmirExprData) -> VmirExprIdx {
        self.vmir_expr_arena.alloc_one(expr_data)
    }

    pub(crate) fn alloc_exprs(&mut self, expr_data: Vec<VmirExprData>) -> VmirExprIdxRange {
        self.vmir_expr_arena.alloc_batch(expr_data)
    }

    pub(crate) fn alloc_stmt(&mut self, stmt_data: VmirStmtData) -> VmirStmtIdx {
        self.vmir_stmt_arena.alloc_one(stmt_data)
    }

    pub(crate) fn alloc_stmts(&mut self, stmt_data: Vec<VmirStmtData>) -> VmirStmtIdxRange {
        self.vmir_stmt_arena.alloc_batch(stmt_data)
    }
}
