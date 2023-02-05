use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_block(
        &mut self,
        stmts: StmtIdxRange,
        expr_expectation: LocalTermExpectation,
    ) -> Option<LocalTerm> {
        for stmt in stmts.start()..(stmts.end() - 1) {
            self.infer_new_nonlast_stmt(stmt)
        }
        self.infer_new_last_stmt(stmts.end() - 1, expr_expectation)
    }
    fn visit_new_exprs_in_stmt(&mut self, stmt_idx: StmtIdx) {}

    fn infer_new_nonlast_stmt(&mut self, stmt_idx: StmtIdx) {
        let expect_unit = self.expect_unit();
        match self.calc_stmt(stmt_idx, expect_unit) {
            Some(ty) => match ty {
                LocalTerm::Resolved(ty) => match ty {
                    ty if ty == self.term_menu.unit() => return,
                    ty => todo!(),
                },
                LocalTerm::Unresolved(_) => todo!(),
            },
            None => (),
        }
    }

    fn expect_unit(&mut self) -> LocalTermExpectation {
        LocalTermExpectation::ImplicitlyConvertibleTo {
            term: self.term_menu.unit().into(),
        }
    }

    fn infer_new_last_stmt(
        &mut self,
        stmt_idx: StmtIdx,
        expr_expectation: LocalTermExpectation,
    ) -> Option<LocalTerm> {
        let expr_expectation = match self.return_ty {
            Some(_) => todo!(),
            None => LocalTermExpectation::None,
        };
        self.calc_stmt(stmt_idx, expr_expectation)
    }

    fn calc_stmt(
        &mut self,
        stmt_idx: StmtIdx,
        expr_expectation: LocalTermExpectation,
    ) -> Option<LocalTerm> {
        match self.expr_region_data[stmt_idx] {
            Stmt::Let {
                let_token,
                ref let_variable_pattern,
                ref initial_value,
                ..
            } => self.calc_let_stmt(let_variable_pattern, initial_value),
            Stmt::Return { ref result, .. } => {
                result.as_ref().copied().map(|result| {
                    self.infer_new_expr(result, LocalTermExpectation::Return { ty: self.return_ty })
                });
                todo!()
            }
            Stmt::Require { ref condition, .. } => {
                condition.as_ref().copied().map(|condition| {
                    self.infer_new_expr(condition, LocalTermExpectation::Condition)
                });
                Some(self.term_menu.unit().into())
            }
            Stmt::Assert { ref condition, .. } => todo!(),
            Stmt::Break { .. } => Some(self.term_menu.never().into()),
            Stmt::Eval { expr_idx } => self.infer_new_expr(expr_idx, expr_expectation),
            Stmt::ForBetween {
                ref particulars,
                ref block,
                ..
            } => todo!(),
            Stmt::ForIn {
                ref condition,
                ref block,
                ..
            } => todo!(),
            Stmt::ForExt { ref block, .. } => todo!(),
            Stmt::While {
                ref condition,
                ref block,
                ..
            }
            | Stmt::DoWhile {
                ref condition,
                ref block,
                ..
            } => {
                condition.as_ref().copied().map(|condition| {
                    self.infer_new_expr(condition, LocalTermExpectation::Condition)
                });
                block.as_ref().copied().map(|block| {
                    let expect_unit = self.expect_unit();
                    self.infer_new_block(block, expect_unit)
                });
                Some(self.term_menu.unit().into())
            }
            Stmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => self.calc_if_else_stmt(
                if_branch,
                elif_branches,
                else_branch.as_ref(),
                expr_expectation,
            ),
            Stmt::Match {} => todo!(),
            Stmt::Err(_) => todo!(),
        }
    }

    fn calc_let_stmt(
        &mut self,
        let_variable_pattern: &Result<LetVariablesPattern, ExprError>,
        initial_value: &Result<ExprIdx, ExprError>,
    ) -> Option<LocalTerm> {
        let pattern_ty = match let_variable_pattern {
            Ok(pattern) => match pattern.ty() {
                Some(ty) => todo!(),
                None => {
                    initial_value
                        .as_ref()
                        .ok()
                        .map(|initial_value| {
                            self.infer_new_expr(
                                *initial_value,
                                // ad hoc
                                LocalTermExpectation::None,
                            )
                        })
                        .flatten()
                }
            },
            Err(_) => todo!(),
        };
        match pattern_ty {
            Some(ty) if ty == self.term_menu.never().into() => Some(self.term_menu.never().into()),
            Some(ty) => {
                todo!("assign type to variables")
            }
            None => Some(self.term_menu.unit().into()),
        }
    }

    fn calc_if_else_stmt(
        &mut self,
        if_branch: &IfBranch,
        elif_branches: &[ElifBranch],
        else_branch: Option<&ElseBranch>,
        expr_expectation: LocalTermExpectation,
    ) -> Option<LocalTerm> {
        let mut branch_tys = BranchTypes::new(expr_expectation);
        if_branch
            .condition
            .as_ref()
            .copied()
            .map(|condition| self.infer_new_expr(condition, LocalTermExpectation::Condition));
        branch_tys.visit_branch(self, &if_branch.block);
        for elif_branch in elif_branches {
            elif_branch
                .condition
                .as_ref()
                .copied()
                .map(|condition| self.infer_new_expr(condition, LocalTermExpectation::Condition));
            branch_tys.visit_branch(self, &elif_branch.block);
        }
        if let Some(else_branch) = else_branch {
            branch_tys.visit_branch(self, &else_branch.block);
        }
        // exhaustive iff else branch exists
        branch_tys.merge(else_branch.is_some())
    }
}

struct BranchTypes {
    /// this is true if the type of one of the branches cannot be inferred
    has_error: bool,
    /// this is true if the type of one of the branches is inferred to be not never
    has_ever: bool,
    expr_expectation: LocalTermExpectation,
}

impl BranchTypes {
    fn new(expr_expectation: LocalTermExpectation) -> Self {
        Self {
            has_error: false,
            has_ever: false,
            expr_expectation,
        }
    }

    fn visit_branch(&mut self, engine: &mut ExprTypeEngine, block: &ExprResult<StmtIdxRange>) {
        match block {
            Ok(stmts) => match engine.infer_new_block(*stmts, self.expr_expectation) {
                Some(LocalTerm::Resolved(term)) if term == engine.term_menu.never() => (),
                Some(term) => {
                    p!(term.debug(engine.db));
                    todo!()
                }
                None => self.has_error = true,
            },
            Err(_) => self.has_error = true,
        };
    }

    fn merge(self, exhaustive: bool) -> Option<LocalTerm> {
        if self.has_error {
            return None;
        }
        if self.has_ever {
            return todo!();
        }
        todo!()
    }
}
