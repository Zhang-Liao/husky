use husky_token::{RangedTokenSheet, TokenSheetData};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct AstRangeSheet {
    text_ranges: Vec<TextRange>,
}

#[salsa::tracked(jar = AstJar, return_ref)]
pub(crate) fn ast_range_sheet(db: &dyn AstDb, module_path: ModulePath) -> VfsResult<AstRangeSheet> {
    let ranged_token_sheet = db.ranged_token_sheet(module_path)?;
    let token_sheet_data = db.token_sheet_data(module_path)?;
    let ast_sheet = db.ast_sheet(module_path)?;
    Ok(AstRangeSheet {
        text_ranges: AstRangeCalculator {
            ranged_token_sheet,
            token_sheet_data,
            ast_sheet,
            text_ranges: Default::default(),
        }
        .calc_all(),
    })
}

#[test]
fn ast_range_sheet_works() {
    use tests::*;
    DB::expect_test_probable_modules_debug_result_with_db(
        "ast_range_sheet",
        AstDb::ast_range_sheet,
    );
}

impl std::ops::Index<AstIdx> for AstRangeSheet {
    type Output = TextRange;

    fn index(&self, index: AstIdx) -> &Self::Output {
        &self.text_ranges[index.raw()]
    }
}

impl<Db: AstDb> salsa::DebugWithDb<Db> for AstRangeSheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &Db,
        _include_all_fields: bool,
    ) -> std::fmt::Result {
        f.debug_struct("AstRangeSheet")
            .field("text_ranges", &self.text_ranges)
            .finish()
    }
}

struct AstRangeCalculator<'a> {
    ranged_token_sheet: &'a RangedTokenSheet,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    text_ranges: Vec<TextRange>,
}

impl<'a> AstRangeCalculator<'a> {
    fn calc_all(mut self) -> Vec<TextRange> {
        for ast in self.ast_sheet.ast_arena.data() {
            self.text_ranges.push(self.calc_ast_range(ast))
        }
        self.text_ranges
    }

    fn token_group_text_range(&self, token_group_idx: TokenGroupIdx) -> TextRange {
        self.ranged_token_sheet.tokens_text_range(
            self.token_sheet_data
                .token_group_token_idx_range(token_group_idx),
        )
    }

    fn calc_ast_range(&self, ast: &Ast) -> TextRange {
        match ast {
            Ast::Err {
                token_group_idx, ..
            }
            | Ast::Use {
                token_group_idx, ..
            }
            | Ast::Decor {
                token_group_idx, ..
            }
            | Ast::ModuleItemVariant {
                token_group_idx, ..
            } => self.token_group_text_range(*token_group_idx),
            Ast::BasicStmt {
                token_group_idx,
                body,
                ..
            }
            | Ast::Defn {
                token_group_idx,
                body,
                ..
            }
            | Ast::Impl {
                token_group_idx,
                body,
                ..
            }
            | Ast::Main {
                token_group_idx,
                body,
            }
            | Ast::Config {
                token_group_idx,
                body,
            } => {
                let token_group_text_range = self.token_group_text_range(*token_group_idx);
                let start = token_group_text_range.start;
                let end = match body.last() {
                    Some(last) => self.text_ranges[last.raw()].text_end(),
                    None => token_group_text_range.end,
                };
                (start..end).into()
            }
            Ast::IfElseStmts {
                if_stmt,
                elif_stmts,
                else_stmt,
            } => {
                let start = self.text_ranges[if_stmt.raw()].start;
                let end = match else_stmt {
                    Some(else_stmt) => self.text_ranges[else_stmt.raw()].end,
                    None => {
                        if let Some(last) = elif_stmts.last() {
                            self.text_ranges[last.raw()].end
                        } else {
                            self.text_ranges[if_stmt.raw()].end
                        }
                    }
                };
                (start..end).into()
            }
            Ast::MatchStmts {
                pattern_stmt,
                case_stmts,
            } => {
                let start = self.text_ranges[pattern_stmt.raw()].start;
                let end = {
                    if let Some(last) = case_stmts.last() {
                        self.text_ranges[last.raw()].end
                    } else {
                        self.text_ranges[pattern_stmt.raw()].end
                    }
                };
                (start..end).into()
            }
        }
    }
}
