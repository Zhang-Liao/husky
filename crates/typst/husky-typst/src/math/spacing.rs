use unicode_math_class::MathClass;

use crate::foundations::{IsTypstElem, TypstValueAssignmentGroup};
use crate::layout::{HElem, TypstAbsLength, TypstEmLength};
use crate::math::{MathFragment, SpacingFragment, TypstMathSize};

pub(super) const THIN: TypstEmLength = TypstEmLength::new(1.0 / 6.0);
pub(super) const MEDIUM: TypstEmLength = TypstEmLength::new(2.0 / 9.0);
pub(super) const THICK: TypstEmLength = TypstEmLength::new(5.0 / 18.0);
pub(super) const QUAD: TypstEmLength = TypstEmLength::new(1.0);
pub(super) const WIDE: TypstEmLength = TypstEmLength::new(2.0);

/// Hook up all spacings.
pub(super) fn define(math: &mut TypstValueAssignmentGroup) {
    math.define("thin", HElem::new(THIN.into()).pack());
    math.define("med", HElem::new(MEDIUM.into()).pack());
    math.define("thick", HElem::new(THICK.into()).pack());
    math.define("quad", HElem::new(QUAD.into()).pack());
    math.define("wide", HElem::new(WIDE.into()).pack());
}

/// Create the spacing between two fragments in a given style.
pub(super) fn spacing(
    l: &MathFragment,
    space: Option<MathFragment>,
    r: &MathFragment,
) -> Option<MathFragment> {
    use MathClass::*;

    let resolve = |v: TypstEmLength, size_ref: &MathFragment| -> Option<MathFragment> {
        let width = size_ref
            .font_size()
            .map_or(TypstAbsLength::zero(), |size| v.at(size));
        Some(SpacingFragment { width, weak: false }.into())
    };
    let script = |f: &MathFragment| f.math_size().map_or(false, |s| s <= TypstMathSize::Script);

    match (l.class(), r.class()) {
        // No spacing before punctuation; thin spacing after punctuation, unless
        // in script size.
        (_, Punctuation) => None,
        (Punctuation, _) if !script(l) => resolve(THIN, l),

        // No spacing after opening delimiters and before closing delimiters.
        (Opening, _) | (_, Closing) => None,

        // Thick spacing around relations, unless followed by a another relation
        // or in script size.
        (Relation, Relation) => None,
        (Relation, _) if !script(l) => resolve(THICK, l),
        (_, Relation) if !script(r) => resolve(THICK, r),

        // Medium spacing around binary operators, unless in script size.
        (Binary, _) if !script(l) => resolve(MEDIUM, l),
        (_, Binary) if !script(r) => resolve(MEDIUM, r),

        // Thin spacing around large operators, unless to the left of
        // an opening delimiter. TeXBook, p170
        (Large, Opening | Fence) => None,
        (Large, _) => resolve(THIN, l),
        (_, Large) => resolve(THIN, r),

        // Spacing around spaced frames.
        _ if (l.is_spaced() || r.is_spaced()) => space,

        _ => None,
    }
}