use ecow::eco_format;

use crate::diag::{At, SourceResult};
use crate::eval::{Eval, Vm};
use crate::foundations::{IsTexElem, TexContent, TexValue};
use crate::math::{AttachTexElem, FracElem, LrElem, PrimesElem, RootElem, TexAlignPointElem};
use crate::syntax::ast::{self, TexAstNode};
use crate::text::TextElem;

impl Eval for ast::Math<'_> {
    type Output = TexContent;
    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        Ok(TexContent::sequence(
            self.exprs()
                .map(|expr| expr.eval_display(vm))
                .collect::<SourceResult<Vec<_>>>()?,
        ))
    }
}

impl Eval for ast::MathIdent<'_> {
    type Output = TexValue;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        vm.scopes.get_in_math(&self).cloned().at(self.span())
    }
}

impl Eval for ast::MathAlignPoint<'_> {
    type Output = TexContent;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(TexAlignPointElem::new().pack())
    }
}

impl Eval for ast::MathDelimited<'_> {
    type Output = TexContent;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let open = self.open().eval_display(vm)?;
        let body = self.body().eval(vm)?;
        let close = self.close().eval_display(vm)?;
        Ok(LrElem::new(open + body + close).pack())
    }
}

impl Eval for ast::MathAttach<'_> {
    type Output = TexContent;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let base = self.base().eval_display(vm)?;
        let mut elem = AttachTexElem::new(base);

        if let Some(expr) = self.top() {
            elem.push_t(Some(expr.eval_display(vm)?));
        } else if let Some(primes) = self.primes() {
            elem.push_t(Some(primes.eval(vm)?));
        }

        if let Some(expr) = self.bottom() {
            elem.push_b(Some(expr.eval_display(vm)?));
        }

        Ok(elem.pack())
    }
}

impl Eval for ast::MathPrimes<'_> {
    type Output = TexContent;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(PrimesElem::new(self.count()).pack())
    }
}

impl Eval for ast::MathFrac<'_> {
    type Output = TexContent;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let num = self.num().eval_display(vm)?;
        let denom = self.denom().eval_display(vm)?;
        Ok(FracElem::new(num, denom).pack())
    }
}

impl Eval for ast::MathRoot<'_> {
    type Output = TexContent;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let index = self.index().map(|i| TextElem::packed(eco_format!("{i}")));
        let radicand = self.radicand().eval_display(vm)?;
        Ok(RootElem::new(radicand).with_index(index).pack())
    }
}

trait ExprExt {
    fn eval_display(&self, vm: &mut Vm) -> SourceResult<TexContent>;
}

impl ExprExt for ast::Expr<'_> {
    fn eval_display(&self, vm: &mut Vm) -> SourceResult<TexContent> {
        Ok(self.eval(vm)?.display().spanned(self.span()))
    }
}