//! Composable layouts.

mod abs;
mod align;
mod angle;
mod axes;
mod columns;
mod container;
mod corners;
mod direction;
mod em;
mod flow;
mod fraction;
mod fragment;
mod frame;
mod grid;
mod hide;
mod inline;
#[path = "layout/layout.rs"]
mod layout_;
mod length;
#[path = "layout/measure.rs"]
mod measure_;
mod pad;
mod page;
mod place;
mod point;
mod ratio;
mod regions;
mod rel;
mod repeat;
mod sides;
mod size;
mod spacing;
mod stack;
mod transform;

pub use self::abs::*;
pub use self::align::*;
pub use self::angle::*;
pub use self::axes::*;
pub use self::columns::*;
pub use self::container::*;
pub use self::corners::*;
pub use self::direction::*;
pub use self::em::*;
pub use self::flow::*;
pub use self::fraction::*;
pub use self::fragment::*;
pub use self::frame::*;
pub use self::grid::*;
pub use self::hide::*;
pub use self::layout_::*;
pub use self::length::*;
pub use self::measure_::*;
pub use self::pad::*;
pub use self::page::*;
pub use self::place::*;
pub use self::point::*;
pub use self::ratio::*;
pub use self::regions::Regions;
pub use self::rel::*;
pub use self::repeat::*;
pub use self::sides::*;
pub use self::size::*;
pub use self::spacing::*;
pub use self::stack::*;
pub use self::transform::*;

pub(crate) use self::inline::*;

use comemo::{Tracked, TrackedMut};

use crate::diag::{bail, SourceResult};
use crate::engine::{Route, TexEngine};
use crate::eval::Tracer;
use crate::foundations::{category, StyleChain, TexContent, TexDefnKind, TexValueAssignmentGroup};
use crate::introspection::{Introspector, Locator};
use crate::model::TexDocument;
use crate::realize::{realize_block, realize_root, Scratch};
use crate::IsTexWorld;

/// Arranging elements on the page in different ways.
///
/// By combining layout functions, you can create complex and automatic layouts.
#[category]
pub static LAYOUT: TexDefnKind;

/// Hook up all `layout` definitions.
pub fn define(global: &mut TexValueAssignmentGroup) {
    global.category(LAYOUT);
    global.define_type::<Length>();
    global.define_type::<Angle>();
    global.define_type::<Ratio>();
    global.define_type::<Rel<Length>>();
    global.define_type::<TexFraction>();
    global.define_type::<TexLayoutDirection>();
    global.define_type::<TexAlignment>();
    global.define_elem::<PageElem>();
    global.define_elem::<PagebreakElem>();
    global.define_elem::<VElem>();
    global.define_elem::<HElem>();
    global.define_elem::<BoxTexElem>();
    global.define_elem::<BlockElem>();
    global.define_elem::<StackElem>();
    global.define_elem::<GridElem>();
    global.define_elem::<ColumnsElem>();
    global.define_elem::<ColbreakElem>();
    global.define_elem::<PlaceElem>();
    global.define_elem::<AlignElem>();
    global.define_elem::<PadElem>();
    global.define_elem::<RepeatElem>();
    global.define_elem::<MoveElem>();
    global.define_elem::<ScaleElem>();
    global.define_elem::<RotateElem>();
    global.define_elem::<HideElem>();
    global.define_func::<measure>();
    global.define_func::<layout>();
}

/// Root-level layout.
pub trait LayoutRoot {
    /// Layout into a document with one frame per page.
    fn layout_root(&self, engine: &mut TexEngine, styles: StyleChain) -> SourceResult<TexDocument>;
}

/// Layout into multiple regions.
pub trait LayoutMultiple {
    /// Layout into one frame per region.
    fn layout(
        &self,
        engine: &mut TexEngine,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<TexLayoutFragment>;

    /// Layout without side effects.
    ///
    /// This element must be layouted again in the same order for the results to
    /// be valid.
    fn measure(
        &self,
        engine: &mut TexEngine,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<TexLayoutFragment> {
        let mut locator = Locator::chained(engine.locator.track());
        let mut engine = TexEngine {
            world: engine.world,
            route: engine.route.clone(),
            introspector: engine.introspector,
            locator: &mut locator,
            tracer: TrackedMut::reborrow_mut(&mut engine.tracer),
        };
        self.layout(&mut engine, styles, regions)
    }
}

/// Layout into a single region.
pub trait LayoutSingle {
    /// Layout into one frame per region.
    fn layout(
        &self,
        engine: &mut TexEngine,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<TexFrame>;
}

impl LayoutRoot for TexContent {
    fn layout_root(&self, engine: &mut TexEngine, styles: StyleChain) -> SourceResult<TexDocument> {
        #[comemo::memoize]
        fn cached(
            content: &TexContent,
            world: Tracked<dyn IsTexWorld + '_>,
            introspector: Tracked<Introspector>,
            route: Tracked<Route>,
            locator: Tracked<Locator>,
            tracer: TrackedMut<Tracer>,
            styles: StyleChain,
        ) -> SourceResult<TexDocument> {
            let mut locator = Locator::chained(locator);
            let mut engine = TexEngine {
                world,
                introspector,
                route: Route::extend(route).unnested(),
                locator: &mut locator,
                tracer,
            };
            let scratch = Scratch::default();
            let (document, styles) = realize_root(&mut engine, &scratch, content, styles)?;
            document.layout_root(&mut engine, styles)
        }

        cached(
            self,
            engine.world,
            engine.introspector,
            engine.route.track(),
            engine.locator.track(),
            TrackedMut::reborrow_mut(&mut engine.tracer),
            styles,
        )
    }
}

impl LayoutMultiple for TexContent {
    fn layout(
        &self,
        engine: &mut TexEngine,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<TexLayoutFragment> {
        #[allow(clippy::too_many_arguments)]
        #[comemo::memoize]
        fn cached(
            content: &TexContent,
            world: Tracked<dyn IsTexWorld + '_>,
            introspector: Tracked<Introspector>,
            route: Tracked<Route>,
            locator: Tracked<Locator>,
            tracer: TrackedMut<Tracer>,
            styles: StyleChain,
            regions: Regions,
        ) -> SourceResult<TexLayoutFragment> {
            let mut locator = Locator::chained(locator);
            let mut engine = TexEngine {
                world,
                introspector,
                route: Route::extend(route),
                locator: &mut locator,
                tracer,
            };

            if !engine.route.within(Route::MAX_LAYOUT_DEPTH) {
                bail!(
                    content.span(), "maximum layout depth exceeded";
                    hint: "try to reduce the amount of nesting in your layout",
                );
            }

            let scratch = Scratch::default();
            let (realized, styles) = realize_block(&mut engine, &scratch, content, styles)?;
            realized
                .with::<dyn LayoutMultiple>()
                .unwrap()
                .layout(&mut engine, styles, regions)
        }

        let fragment = cached(
            self,
            engine.world,
            engine.introspector,
            engine.route.track(),
            engine.locator.track(),
            TrackedMut::reborrow_mut(&mut engine.tracer),
            styles,
            regions,
        )?;

        engine.locator.visit_frames(&fragment);
        Ok(fragment)
    }
}