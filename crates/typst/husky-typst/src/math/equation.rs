use std::num::NonZeroUsize;

use unicode_math_class::MathClass;

use crate::diag::{bail, TypstSourceResult};
use crate::engine::TypstEngine;
use crate::foundations::{
    elem, IsTypstElem, Resolve, Smart, TypstContent, TypstContentRefined, TypstShowSet,
    TypstStyleChain, TypstStyles, TypstSynthesize,
};
use crate::introspection::{Counter, CounterUpdate, TypstCount, TypstLocatable};
use crate::layout::{
    AlignElem, Axes, FixedAlignment, LayoutMultiple, Size, TypstAbsLength, TypstAlignment,
    TypstEmLength, TypstFrame, TypstLayoutDirection, TypstLayoutSingle, TypstPoint, TypstRegions,
};
use crate::math::{
    scaled_font_size, MathVariant, TypstLayoutMath, TypstMathContext, TypstMathSize,
};
use crate::model::{Numbering, ParagraphTypstElem, Refable, Supplement, TypstOutlinable};
use crate::syntax::TypstSynSpan;
use crate::text::{
    families, variant, FontFamily, FontList, FontWeight, Lang, LocalName, Region, TextElem,
    TypstFont,
};
use crate::util::{option_eq, NonZeroExt, TypstNumeric};
use crate::IsTypstWorld;

/// A mathematical equation.
///
/// Can be displayed inline with text or as a separate block.
///
/// # Example
/// ```example
/// #set text(font: "New Computer Modern")
///
/// Let $a$, $b$, and $c$ be the side
/// lengths of right-angled triangle.
/// Then, we know that:
/// $ a^2 + b^2 = c^2 $
///
/// Prove by induction:
/// $ sum_(k=1)^n k = (n(n+1)) / 2 $
/// ```
///
/// # Syntax
/// This function also has dedicated syntax: Write mathematical markup within
/// dollar signs to create an equation. Starting and ending the equation with at
/// least one space lifts it into a separate block that is centered
/// horizontally. For more details about math syntax, see the
/// [main math page]($category/math).
#[elem(
    TypstLocatable,
    TypstSynthesize,
    TypstShowSet,
    TypstLayoutSingle,
    TypstLayoutMath,
    TypstCount,
    LocalName,
    Refable,
    TypstOutlinable
)]
pub struct TypstEquationElem {
    /// Whether the equation is displayed as a separate block.
    #[default(false)]
    pub block: bool,

    /// How to [number]($numbering) block-level equations.
    ///
    /// ```example
    /// #set math.equation(numbering: "(1)")
    ///
    /// We define:
    /// $ phi.alt := (1 + sqrt(5)) / 2 $ <ratio>
    ///
    /// With @ratio, we get:
    /// $ F_n = floor(1 / sqrt(5) phi.alt^n) $
    /// ```
    #[borrowed]
    pub numbering: Option<Numbering>,

    /// A supplement for the equation.
    ///
    /// For references to equations, this is added before the referenced number.
    ///
    /// If a function is specified, it is passed the referenced equation and
    /// should return content.
    ///
    /// ```example
    /// #set math.equation(numbering: "(1)", supplement: [Eq.])
    ///
    /// We define:
    /// $ phi.alt := (1 + sqrt(5)) / 2 $ <ratio>
    ///
    /// With @ratio, we get:
    /// $ F_n = floor(1 / sqrt(5) phi.alt^n) $
    /// ```
    pub supplement: Smart<Option<Supplement>>,

    /// The contents of the equation.
    #[required]
    pub body: TypstContent,

    /// The size of the glyphs.
    #[internal]
    #[default(TypstMathSize::Text)]
    #[ghost]
    pub size: TypstMathSize,

    /// The style variant to select.
    #[internal]
    #[ghost]
    pub variant: MathVariant,

    /// Affects the height of exponents.
    #[internal]
    #[default(false)]
    #[ghost]
    pub cramped: bool,

    /// Whether to use bold glyphs.
    #[internal]
    #[default(false)]
    #[ghost]
    pub bold: bool,

    /// Whether to use italic glyphs.
    #[internal]
    #[ghost]
    pub italic: Smart<bool>,

    /// A forced class to use for all fragment.
    #[internal]
    #[ghost]
    pub class: Option<MathClass>,
}

impl TypstSynthesize for TypstContentRefined<TypstEquationElem> {
    fn synthesize(
        &mut self,
        engine: &mut TypstEngine,
        styles: TypstStyleChain,
    ) -> TypstSourceResult<()> {
        let supplement = match self.as_ref().supplement(styles) {
            Smart::Auto => TextElem::packed(Self::local_name_in(styles)),
            Smart::Custom(None) => TypstContent::empty(),
            Smart::Custom(Some(supplement)) => supplement.resolve(engine, [self.clone().pack()])?,
        };

        self.push_supplement(Smart::Custom(Some(Supplement::Content(supplement))));
        Ok(())
    }
}

impl TypstShowSet for TypstContentRefined<TypstEquationElem> {
    fn show_set(&self, styles: TypstStyleChain) -> TypstStyles {
        let mut out = TypstStyles::new();
        if self.block(styles) {
            out.set(AlignElem::set_alignment(TypstAlignment::CENTER));
            out.set(TypstEquationElem::set_size(TypstMathSize::Display));
        }
        out.set(TextElem::set_weight(FontWeight::from_number(450)));
        out.set(TextElem::set_font(FontList(vec![FontFamily::new(
            "New Computer Modern Math",
        )])));
        out
    }
}

/// Layouted items suitable for placing in a paragraph.
#[derive(Debug, Clone)]
pub enum MathParItem {
    Space(TypstAbsLength),
    Frame(TypstFrame),
}

impl MathParItem {
    /// The text representation of this item.
    pub fn text(&self) -> char {
        match self {
            MathParItem::Space(_) => ' ',        // Space
            MathParItem::Frame(_) => '\u{FFFC}', // Object Replacement Character
        }
    }
}

impl TypstContentRefined<TypstEquationElem> {
    pub fn layout_inline(
        &self,
        engine: &mut TypstEngine<'_>,
        styles: TypstStyleChain,
        regions: TypstRegions,
    ) -> TypstSourceResult<Vec<MathParItem>> {
        assert!(!self.block(styles));

        // Find a math font.
        let font = find_math_font(engine, styles, self.span())?;

        let mut ctx = TypstMathContext::new(engine, styles, regions, &font);
        let rows = ctx.layout_root(self, styles)?;

        let mut items = if rows.row_count() == 1 {
            rows.into_par_items()
        } else {
            vec![MathParItem::Frame(
                rows.into_fragment(&ctx, styles).into_frame(),
            )]
        };

        for item in &mut items {
            let MathParItem::Frame(frame) = item else {
                continue;
            };

            let font_size = scaled_font_size(&ctx, styles);
            let slack = ParagraphTypstElem::leading_in(styles) * 0.7;
            let top_edge = TextElem::top_edge_in(styles).resolve(font_size, &font, None);
            let bottom_edge = -TextElem::bottom_edge_in(styles).resolve(font_size, &font, None);

            let ascent = top_edge.max(frame.ascent() - slack);
            let descent = bottom_edge.max(frame.descent() - slack);
            frame.translate(TypstPoint::with_y(ascent - frame.baseline()));
            frame.size_mut().y = ascent + descent;
        }

        Ok(items)
    }
}

impl TypstLayoutSingle for TypstContentRefined<TypstEquationElem> {
    #[husky_typst_macros::time(name = "math.equation", span = self.span())]
    fn layout(
        &self,
        engine: &mut TypstEngine,
        styles: TypstStyleChain,
        regions: TypstRegions,
    ) -> TypstSourceResult<TypstFrame> {
        const NUMBER_GUTTER: TypstEmLength = TypstEmLength::new(0.5);

        assert!(self.block(styles));

        // Find a math font.
        let font = find_math_font(engine, styles, self.span())?;

        let mut ctx = TypstMathContext::new(engine, styles, regions, &font);
        let mut frame = ctx.layout_frame(self, styles)?;

        if let Some(numbering) = (**self).numbering(styles) {
            let pod = TypstRegions::one(regions.base(), Axes::splat(false));
            let counter = Counter::of(TypstEquationElem::elem())
                .at(engine, self.location().unwrap())?
                .display(engine, numbering)?
                .spanned(self.span())
                .layout(engine, styles, pod)?
                .into_frame();

            let full_counter_width = counter.width() + NUMBER_GUTTER.resolve(styles);
            let width = if regions.size.x.is_finite() {
                regions.size.x
            } else {
                frame.width() + 2.0 * full_counter_width
            };

            let height = frame.height().max(counter.height());
            let align = AlignElem::alignment_in(styles).resolve(styles).x;
            frame.resize(Size::new(width, height), Axes::splat(align));

            let dir = TextElem::dir_in(styles);
            let offset = match (align, dir) {
                (FixedAlignment::Start, TypstLayoutDirection::RightLeft) => full_counter_width,
                (FixedAlignment::End, TypstLayoutDirection::LeftRight) => -full_counter_width,
                _ => TypstAbsLength::zero(),
            };
            frame.translate(TypstPoint::with_x(offset));

            let x = if dir.is_positive() {
                frame.width() - counter.width()
            } else {
                TypstAbsLength::zero()
            };
            let y = (frame.height() - counter.height()) / 2.0;

            frame.push_frame(TypstPoint::new(x, y), counter)
        }

        Ok(frame)
    }
}

impl TypstCount for TypstContentRefined<TypstEquationElem> {
    fn update(&self) -> Option<CounterUpdate> {
        (self.block(TypstStyleChain::default()) && self.numbering().is_some())
            .then(|| CounterUpdate::Step(NonZeroUsize::ONE))
    }
}

impl LocalName for TypstContentRefined<TypstEquationElem> {
    fn local_name(lang: Lang, region: Option<Region>) -> &'static str {
        match lang {
            Lang::ALBANIAN => "Ekuacion",
            Lang::ARABIC => "معادلة",
            Lang::BOKMÅL => "Ligning",
            Lang::CATALAN => "Equació",
            Lang::CHINESE if option_eq(region, "TW") => "方程式",
            Lang::CHINESE => "公式",
            Lang::CZECH => "Rovnice",
            Lang::DANISH => "Ligning",
            Lang::DUTCH => "Vergelijking",
            Lang::ESTONIAN => "Valem",
            Lang::FILIPINO => "Ekwasyon",
            Lang::FINNISH => "Yhtälö",
            Lang::FRENCH => "Équation",
            Lang::GERMAN => "Gleichung",
            Lang::GREEK => "Εξίσωση",
            Lang::HUNGARIAN => "Egyenlet",
            Lang::ITALIAN => "Equazione",
            Lang::NYNORSK => "Likning",
            Lang::POLISH => "Równanie",
            Lang::PORTUGUESE => "Equação",
            Lang::ROMANIAN => "Ecuația",
            Lang::RUSSIAN => "Уравнение",
            Lang::SERBIAN => "Једначина",
            Lang::SLOVENIAN => "Enačba",
            Lang::SPANISH => "Ecuación",
            Lang::SWEDISH => "Ekvation",
            Lang::TURKISH => "Denklem",
            Lang::UKRAINIAN => "Рівняння",
            Lang::VIETNAMESE => "Phương trình",
            Lang::JAPANESE => "式",
            Lang::ENGLISH | _ => "Equation",
        }
    }
}

impl Refable for TypstContentRefined<TypstEquationElem> {
    fn supplement(&self) -> TypstContent {
        // After synthesis, this should always be custom content.
        match (**self).supplement(TypstStyleChain::default()) {
            Smart::Custom(Some(Supplement::Content(content))) => content,
            _ => TypstContent::empty(),
        }
    }

    fn counter(&self) -> Counter {
        Counter::of(TypstEquationElem::elem())
    }

    fn numbering(&self) -> Option<&Numbering> {
        (**self).numbering(TypstStyleChain::default()).as_ref()
    }
}

impl TypstOutlinable for TypstContentRefined<TypstEquationElem> {
    fn outline(&self, engine: &mut TypstEngine) -> TypstSourceResult<Option<TypstContent>> {
        if !self.block(TypstStyleChain::default()) {
            return Ok(None);
        }
        let Some(numbering) = self.numbering() else {
            return Ok(None);
        };

        // After synthesis, this should always be custom content.
        let mut supplement = match (**self).supplement(TypstStyleChain::default()) {
            Smart::Custom(Some(Supplement::Content(content))) => content,
            _ => TypstContent::empty(),
        };

        if !supplement.is_empty() {
            supplement += TextElem::packed("\u{a0}");
        }

        let numbers = self
            .counter()
            .at(engine, self.location().unwrap())?
            .display(engine, numbering)?;

        Ok(Some(supplement + numbers))
    }
}

impl TypstLayoutMath for TypstContentRefined<TypstEquationElem> {
    #[husky_typst_macros::time(name = "math.equation", span = self.span())]
    fn layout_math(
        &self,
        ctx: &mut TypstMathContext,
        styles: TypstStyleChain,
    ) -> TypstSourceResult<()> {
        self.body().layout_math(ctx, styles)
    }
}

fn find_math_font(
    engine: &mut TypstEngine<'_>,
    styles: TypstStyleChain,
    span: TypstSynSpan,
) -> TypstSourceResult<TypstFont> {
    let variant = variant(styles);
    let world = engine.world;
    let Some(font) = families(styles).find_map(|family| {
        let id = world.book().select(family, variant)?;
        let font = world.font(id)?;
        let _ = font.ttf().tables().math?.constants?;
        Some(font)
    }) else {
        bail!(span, "current font does not support math");
    };
    Ok(font)
}