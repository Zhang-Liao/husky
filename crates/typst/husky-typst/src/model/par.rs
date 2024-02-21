use std::fmt::{self, Debug, Formatter};

use comemo::Prehashed;

use crate::diag::TypstSourceResult;
use crate::engine::TypstEngine;
use crate::foundations::{
    elem, Args, Cast, Construct, IsTypstElem, Set, Smart, TypstContent, TypstContentRefined,
    TypstStyleChain, Unlabellable,
};
use crate::layout::{Size, TypstEmLength, TypstLayoutFragment, TypstLength};

/// Arranges text, spacing and inline-level elements into a paragraph.
///
/// Although this function is primarily used in set rules to affect paragraph
/// properties, it can also be used to explicitly render its argument onto a
/// paragraph of its own.
///
/// # Example
/// ```example
/// #show par: set block(spacing: 0.65em)
/// #set par(
///   first-line-indent: 1em,
///   justify: true,
/// )
///
/// We proceed by contradiction.
/// Suppose that there exists a set
/// of positive integers $a$, $b$, and
/// $c$ that satisfies the equation
/// $a^n + b^n = c^n$ for some
/// integer value of $n > 2$.
///
/// Without loss of generality,
/// let $a$ be the smallest of the
/// three integers. Then, we ...
/// ```
#[elem(title = "Paragraph", Debug, Construct)]
pub struct ParagraphTypstElem {
    /// The spacing between lines.
    #[resolve]
    #[ghost]
    #[default(TypstEmLength::new(0.65).into())]
    pub leading: TypstLength,

    /// Whether to justify text in its line.
    ///
    /// Hyphenation will be enabled for justified paragraphs if the
    /// [text function's `hyphenate` property]($text.hyphenate) is set to
    /// `{auto}` and the current language is known.
    ///
    /// Note that the current [alignment]($align) still has an effect on the
    /// placement of the last line except if it ends with a
    /// [justified line break]($linebreak.justify).
    #[ghost]
    #[default(false)]
    pub justify: bool,

    /// How to determine line breaks.
    ///
    /// When this property is set to `{auto}`, its default value, optimized line
    /// breaks will be used for justified paragraphs. Enabling optimized line
    /// breaks for ragged paragraphs may also be worthwhile to improve the
    /// appearance of the text.
    ///
    /// ```example
    /// #set page(width: 207pt)
    /// #set par(linebreaks: "simple")
    /// Some texts feature many longer
    /// words. Those are often exceedingly
    /// challenging to break in a visually
    /// pleasing way.
    ///
    /// #set par(linebreaks: "optimized")
    /// Some texts feature many longer
    /// words. Those are often exceedingly
    /// challenging to break in a visually
    /// pleasing way.
    /// ```
    #[ghost]
    pub linebreaks: Smart<Linebreaks>,

    /// The indent the first line of a paragraph should have.
    ///
    /// Only the first line of a consecutive paragraph will be indented (not
    /// the first one in a block or on the page).
    ///
    /// By typographic convention, paragraph breaks are indicated either by some
    /// space between paragraphs or by indented first lines. Consider reducing
    /// the [paragraph spacing]($block.spacing) to the [`leading`] when
    /// using this property (e.g. using
    /// `[#show par: set block(spacing: 0.65em)]`).
    #[ghost]
    pub first_line_indent: TypstLength,

    /// The indent all but the first line of a paragraph should have.
    #[ghost]
    #[resolve]
    pub hanging_indent: TypstLength,

    /// The contents of the paragraph.
    #[external]
    #[required]
    pub body: TypstContent,

    /// The paragraph's children.
    #[internal]
    #[variadic]
    pub children: Vec<Prehashed<TypstContent>>,
}

impl Construct for ParagraphTypstElem {
    fn construct(engine: &mut TypstEngine, args: &mut Args) -> TypstSourceResult<TypstContent> {
        // The paragraph constructor is special: It doesn't create a paragraph
        // element. Instead, it just ensures that the passed content lives in a
        // separate paragraph and styles it.
        let styles = Self::set(engine, args)?;
        let body = args.expect::<TypstContent>("body")?;
        Ok(TypstContent::sequence([
            ParbreakElem::new().pack(),
            body.styled_with_map(styles),
            ParbreakElem::new().pack(),
        ]))
    }
}

impl TypstContentRefined<ParagraphTypstElem> {
    /// Layout the paragraph into a collection of lines.
    #[husky_typst_macros::time(name = "par", span = self.span())]
    pub fn layout(
        &self,
        engine: &mut TypstEngine,
        styles: TypstStyleChain,
        consecutive: bool,
        region: Size,
        expand: bool,
    ) -> TypstSourceResult<TypstLayoutFragment> {
        crate::layout::layout_inline(self.children(), engine, styles, consecutive, region, expand)
    }
}

impl Debug for ParagraphTypstElem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Par ")?;
        f.debug_list().entries(&self.children).finish()
    }
}

/// How to determine line breaks in a paragraph.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Linebreaks {
    /// Determine the line breaks in a simple first-fit style.
    Simple,
    /// Optimize the line breaks for the whole paragraph.
    ///
    /// Typst will try to produce more evenly filled lines of text by
    /// considering the whole paragraph when calculating line breaks.
    Optimized,
}

/// A paragraph break.
///
/// This starts a new paragraph. Especially useful when used within code like
/// [for loops]($scripting/#loops). Multiple consecutive
/// paragraph breaks collapse into a single one.
///
/// # Example
/// ```example
/// #for i in range(3) {
///   [Blind text #i: ]
///   lorem(5)
///   parbreak()
/// }
/// ```
///
/// # Syntax
/// Instead of calling this function, you can insert a blank line into your
/// markup to create a paragraph break.
#[elem(title = "Paragraph Break", Unlabellable)]
pub struct ParbreakElem {}

impl Unlabellable for TypstContentRefined<ParbreakElem> {}
