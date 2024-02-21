use crate::diag::TypstSourceResult;
use crate::foundations::{
    elem, func, IsTypstElem, TypstContent, TypstContentRefined, TypstStyleChain,
};
use crate::layout::{Size, TypstAbsLength, TypstFrame, TypstFrameItem, TypstPoint};
use crate::math::{
    style_cramped, EquationTypstElem, FrameFragment, GlyphFragment, MathContext, MathSize, Scaled,
    TypstLayoutMath,
};
use crate::syntax::TypstSynSpan;
use crate::text::TextElem;
use crate::visualize::{TypstFixedStroke, TypstGeometry};

/// A square root.
///
/// ```example
/// $ sqrt(3 - 2 sqrt(2)) = sqrt(2) - 1 $
/// ```
#[func(title = "Square Root")]
pub fn sqrt(
    /// The call span of this function.
    span: TypstSynSpan,
    /// The expression to take the square root of.
    radicand: TypstContent,
) -> TypstContent {
    RootElem::new(radicand).pack().spanned(span)
}

/// A general root.
///
/// ```example
/// $ root(3, x) $
/// ```
#[elem(TypstLayoutMath)]
pub struct RootElem {
    /// Which root of the radicand to take.
    #[positional]
    pub index: Option<TypstContent>,

    /// The expression to take the root of.
    #[required]
    pub radicand: TypstContent,
}

impl TypstLayoutMath for TypstContentRefined<RootElem> {
    #[husky_typst_macros::time(name = "math.root", span = self.span())]
    fn layout_math(&self, ctx: &mut MathContext, styles: TypstStyleChain) -> TypstSourceResult<()> {
        layout(
            ctx,
            styles,
            self.index(styles).as_ref(),
            self.radicand(),
            self.span(),
        )
    }
}

/// Layout a root.
///
/// TeXbook page 443, page 360
/// See also: https://www.w3.org/TR/mathml-core/#radicals-msqrt-mroot
fn layout(
    ctx: &mut MathContext,
    styles: TypstStyleChain,
    index: Option<&TypstContent>,
    radicand: &TypstContent,
    span: TypstSynSpan,
) -> TypstSourceResult<()> {
    let gap = scaled!(
        ctx, styles,
        text: radical_vertical_gap,
        display: radical_display_style_vertical_gap,
    );
    let thickness = scaled!(ctx, styles, radical_rule_thickness);
    let extra_ascender = scaled!(ctx, styles, radical_extra_ascender);
    let kern_before = scaled!(ctx, styles, radical_kern_before_degree);
    let kern_after = scaled!(ctx, styles, radical_kern_after_degree);
    let raise_factor = percent!(ctx, radical_degree_bottom_raise_percent);

    // Layout radicand.
    let cramped = style_cramped();
    let radicand = ctx.layout_frame(radicand, styles.chain(&cramped))?;

    // Layout root symbol.
    let target = radicand.height() + thickness + gap;
    let sqrt = GlyphFragment::new(ctx, styles, '√', span)
        .stretch_vertical(ctx, target, TypstAbsLength::zero())
        .frame;

    // Layout the index.
    let sscript = EquationTypstElem::set_size(MathSize::ScriptScript).wrap();
    let index = index
        .map(|elem| ctx.layout_frame(elem, styles.chain(&sscript)))
        .transpose()?;

    // TeXbook, page 443, item 11
    // Keep original gap, and then distribute any remaining free space
    // equally above and below.
    let gap = gap.max((sqrt.height() - thickness - radicand.height() + gap) / 2.0);

    let sqrt_ascent = radicand.ascent() + gap + thickness;
    let descent = sqrt.height() - sqrt_ascent;
    let inner_ascent = sqrt_ascent + extra_ascender;

    let mut sqrt_offset = TypstAbsLength::zero();
    let mut shift_up = TypstAbsLength::zero();
    let mut ascent = inner_ascent;

    if let Some(index) = &index {
        sqrt_offset = kern_before + index.width() + kern_after;
        // The formula below for how much raise the index by comes from
        // the TeXbook, page 360, in the definition of `\root`.
        // However, the `+ index.descent()` part is different from TeX.
        // Without it, descenders can collide with the surd, a rarity
        // in practice, but possible.  MS Word also adjusts index positions
        // for descenders.
        shift_up = raise_factor * (inner_ascent - descent) + index.descent();
        ascent.set_max(shift_up + index.ascent());
    }

    let radicand_x = sqrt_offset + sqrt.width();
    let radicand_y = ascent - radicand.ascent();
    let width = radicand_x + radicand.width();
    let size = Size::new(width, ascent + descent);

    // The extra "- thickness" comes from the fact that the sqrt is placed
    // in `push_frame` with respect to its top, not its baseline.
    let sqrt_pos = TypstPoint::new(sqrt_offset, radicand_y - gap - thickness);
    let line_pos = TypstPoint::new(radicand_x, radicand_y - gap - (thickness / 2.0));
    let radicand_pos = TypstPoint::new(radicand_x, radicand_y);

    let mut frame = TypstFrame::soft(size);
    frame.set_baseline(ascent);

    if let Some(index) = index {
        let index_pos = TypstPoint::new(kern_before, ascent - index.ascent() - shift_up);
        frame.push_frame(index_pos, index);
    }

    frame.push_frame(sqrt_pos, sqrt);
    frame.push(
        line_pos,
        TypstFrameItem::Shape(
            TypstGeometry::Line(TypstPoint::with_x(radicand.width())).stroked(
                TypstFixedStroke::from_pair(TextElem::fill_in(styles).as_decoration(), thickness),
            ),
            span,
        ),
    );

    frame.push_frame(radicand_pos, radicand);
    ctx.push(FrameFragment::new(ctx, styles, frame));

    Ok(())
}
