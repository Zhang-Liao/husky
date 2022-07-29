use super::*;
use husky_eager_semantics::{
    FuncPattern, FuncPatternBranch, FuncPatternBranchVariant, ProcPattern, ProcPatternBranch,
    ProcPatternBranchVariant,
};
use std::sync::Arc;

impl<'a> RustCodeGenerator<'a> {
    pub fn gen_func_match_pattern(
        &mut self,
        ref match_expr: &EagerExpr,
        indent: u8,
        ref branches: &[Arc<FuncPatternBranch>],
    ) {
        self.write("match ");
        self.gen_expr(indent, match_expr);
        self.write(" {");
        self.newline();
        let mut has_default = false;
        for branch in branches.iter() {
            self.indent(indent + 4);
            match branch.variant {
                FuncPatternBranchVariant::Case { ref pattern } => {
                    self.gen_func_case_pattern(pattern);
                }
                FuncPatternBranchVariant::Default => {
                    has_default = true;
                    self.write("_")
                }
            }
            self.write(" => {");
            self.newline();
            self.gen_func_stmts(&branch.stmts);
            self.indent(indent + 4);
            self.write("}\n");
        }
        if !has_default {
            self.indent(indent + 4);
            self.write("_ => panic!(),\n")
        }
        self.indent(indent);
        self.write("}");
    }

    pub fn gen_proc_match_pattern(
        &mut self,
        ref match_expr: &EagerExpr,
        indent: u8,
        ref branches: &[Arc<ProcPatternBranch>],
    ) {
        self.write("match ");
        self.gen_expr(indent, match_expr);
        self.write(" {");
        self.newline();
        let mut has_default = false;
        for branch in branches.iter() {
            self.indent(indent + 4);
            match branch.variant {
                ProcPatternBranchVariant::Case { ref pattern } => {
                    self.gen_proc_case_pattern(pattern);
                }
                ProcPatternBranchVariant::Default => {
                    has_default = true;
                    self.write("_")
                }
            }
            self.write(" => {");
            self.newline();
            self.gen_proc_stmts(&branch.stmts);
            self.indent(indent + 4);
            self.write("}\n");
        }
        if !has_default {
            self.indent(indent);
            self.write("_ => panic!(),\n")
        }
        self.indent(indent);
        self.write("}");
    }

    fn gen_func_case_pattern(&mut self, pattern: &FuncPattern) {
        match pattern.variant {
            _ => todo!(),
        }
    }

    fn gen_proc_case_pattern(&mut self, pattern: &ProcPattern) {
        match pattern.variant {
            _ => todo!(),
            // RawPatternVariant::PrimitiveValue(v) => {
            //     let v: String = v.into();
            //     self.write(&(v))
            // }
            // RawPatternVariant::OneOf {
            //     subpatterns: ref patterns,
            // } => {
            //     for (i, pattern) in patterns.iter().enumerate() {
            //         if i > 0 {
            //             self.write(" | ");
            //         }
            //         self.gen_proc_case_pattern(pattern)
            //     }
            // }
            // RawPatternVariant::EnumLiteral(entity_route) => {
            //     self.gen_entity_route(entity_route, EntityRouteRole::Other)
            // }
        }
    }
}
