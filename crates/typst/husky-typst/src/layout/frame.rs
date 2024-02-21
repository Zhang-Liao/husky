//! Finished documents.

use std::fmt::{self, Debug, Formatter};
use std::num::NonZeroUsize;
use std::sync::Arc;

use crate::foundations::{cast, dict, TypstDict, TypstStyleChain, TypstValue};
use crate::introspection::{MetaTypstElem, TypstMeta};
use crate::layout::{
    Axes, Corners, FixedAlignment, Rel, Sides, Size, Transform, TypstAbsLength, TypstLength,
    TypstPoint,
};
use crate::syntax::TypstSynSpan;
use crate::text::TypstTextItem;
use crate::util::TypstNumeric;
use crate::visualize::{
    ellipse, styled_rect, Path, TypstColor, TypstFixedStroke, TypstGeometry, TypstImage,
    TypstPaint, TypstShape,
};

/// A finished layout with items at fixed positions.
#[derive(Default, Clone, Hash)]
pub struct TypstFrame {
    /// The size of the frame.
    size: Size,
    /// The baseline of the frame measured from the top. If this is `None`, the
    /// frame's implicit baseline is at the bottom.
    baseline: Option<TypstAbsLength>,
    /// The items composing this layout.
    items: Arc<Vec<(TypstPoint, TypstFrameItem)>>,
    /// The hardness of this frame.
    kind: FrameKind,
}

/// Constructor, accessors and setters.
impl TypstFrame {
    /// Create a new, empty frame.
    ///
    /// Panics the size is not finite.
    #[track_caller]
    pub fn new(size: Size, kind: FrameKind) -> Self {
        assert!(size.is_finite());
        Self {
            size,
            baseline: None,
            items: Arc::new(vec![]),
            kind,
        }
    }

    /// Create a new, empty soft frame.
    ///
    /// Panics if the size is not finite.
    #[track_caller]
    pub fn soft(size: Size) -> Self {
        Self::new(size, FrameKind::Soft)
    }

    /// Create a new, empty hard frame.
    ///
    /// Panics if the size is not finite.
    #[track_caller]
    pub fn hard(size: Size) -> Self {
        Self::new(size, FrameKind::Hard)
    }

    /// Sets the frame's hardness.
    pub fn set_kind(&mut self, kind: FrameKind) {
        self.kind = kind;
    }

    /// Whether the frame is hard or soft.
    pub fn kind(&self) -> FrameKind {
        self.kind
    }

    /// Whether the frame contains no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// The size of the frame.
    pub fn size(&self) -> Size {
        self.size
    }

    /// The size of the frame, mutably.
    pub fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }

    /// Set the size of the frame.
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    /// The width of the frame.
    pub fn width(&self) -> TypstAbsLength {
        self.size.x
    }

    /// The height of the frame.
    pub fn height(&self) -> TypstAbsLength {
        self.size.y
    }

    /// The vertical position of the frame's baseline.
    pub fn baseline(&self) -> TypstAbsLength {
        self.baseline.unwrap_or(self.size.y)
    }

    /// Whether the frame has a non-default baseline.
    pub fn has_baseline(&self) -> bool {
        self.baseline.is_some()
    }

    /// Set the frame's baseline from the top.
    pub fn set_baseline(&mut self, baseline: TypstAbsLength) {
        self.baseline = Some(baseline);
    }

    /// The distance from the baseline to the top of the frame.
    ///
    /// This is the same as `baseline()`, but more in line with the terminology
    /// used in math layout.
    pub fn ascent(&self) -> TypstAbsLength {
        self.baseline()
    }

    /// The distance from the baseline to the bottom of the frame.
    pub fn descent(&self) -> TypstAbsLength {
        self.size.y - self.baseline()
    }

    /// An iterator over the items inside this frame alongside their positions
    /// relative to the top-left of the frame.
    pub fn items(&self) -> std::slice::Iter<'_, (TypstPoint, TypstFrameItem)> {
        self.items.iter()
    }
}

/// Insert items and subframes.
impl TypstFrame {
    /// The layer the next item will be added on. This corresponds to the number
    /// of items in the frame.
    pub fn layer(&self) -> usize {
        self.items.len()
    }

    /// Add an item at a position in the foreground.
    pub fn push(&mut self, pos: TypstPoint, item: TypstFrameItem) {
        Arc::make_mut(&mut self.items).push((pos, item));
    }

    /// Add a frame at a position in the foreground.
    ///
    /// Automatically decides whether to inline the frame or to include it as a
    /// group based on the number of items in it.
    pub fn push_frame(&mut self, pos: TypstPoint, frame: TypstFrame) {
        if self.should_inline(&frame) {
            self.inline(self.layer(), pos, frame);
        } else {
            self.push(pos, TypstFrameItem::Group(TypstGroupItem::new(frame)));
        }
    }

    /// Add zero-sized metadata at the origin.
    pub fn push_positionless_meta(&mut self, meta: TypstMeta) {
        self.push(TypstPoint::zero(), TypstFrameItem::Meta(meta, Size::zero()));
    }

    /// Insert an item at the given layer in the frame.
    ///
    /// This panics if the layer is greater than the number of layers present.
    #[track_caller]
    pub fn insert(&mut self, layer: usize, pos: TypstPoint, item: TypstFrameItem) {
        Arc::make_mut(&mut self.items).insert(layer, (pos, item));
    }

    /// Add an item at a position in the background.
    pub fn prepend(&mut self, pos: TypstPoint, item: TypstFrameItem) {
        self.insert(0, pos, item);
    }

    /// Add multiple items at a position in the background.
    ///
    /// The first item in the iterator will be the one that is most in the
    /// background.
    pub fn prepend_multiple<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = (TypstPoint, TypstFrameItem)>,
    {
        Arc::make_mut(&mut self.items).splice(0..0, items);
    }

    /// Add a frame at a position in the background.
    pub fn prepend_frame(&mut self, pos: TypstPoint, frame: TypstFrame) {
        if self.should_inline(&frame) {
            self.inline(0, pos, frame);
        } else {
            self.prepend(pos, TypstFrameItem::Group(TypstGroupItem::new(frame)));
        }
    }

    /// Whether the given frame should be inlined.
    fn should_inline(&self, frame: &TypstFrame) -> bool {
        // We do not inline big frames and hard frames.
        frame.kind().is_soft() && (self.items.is_empty() || frame.items.len() <= 5)
    }

    /// Inline a frame at the given layer.
    fn inline(&mut self, layer: usize, pos: TypstPoint, frame: TypstFrame) {
        // Try to just reuse the items.
        if pos.is_zero() && self.items.is_empty() {
            self.items = frame.items;
            return;
        }

        // Try to transfer the items without adjusting the position.
        // Also try to reuse the items if the Arc isn't shared.
        let range = layer..layer;
        if pos.is_zero() {
            let sink = Arc::make_mut(&mut self.items);
            match Arc::try_unwrap(frame.items) {
                Ok(items) => {
                    sink.splice(range, items);
                }
                Err(arc) => {
                    sink.splice(range, arc.iter().cloned());
                }
            }
            return;
        }

        // We have to adjust the item positions.
        // But still try to reuse the items if the Arc isn't shared.
        let sink = Arc::make_mut(&mut self.items);
        match Arc::try_unwrap(frame.items) {
            Ok(items) => {
                sink.splice(range, items.into_iter().map(|(p, e)| (p + pos, e)));
            }
            Err(arc) => {
                sink.splice(range, arc.iter().cloned().map(|(p, e)| (p + pos, e)));
            }
        }
    }
}

/// Modify the frame.
impl TypstFrame {
    /// Remove all items from the frame.
    pub fn clear(&mut self) {
        if Arc::strong_count(&self.items) == 1 {
            Arc::make_mut(&mut self.items).clear();
        } else {
            self.items = Arc::new(vec![]);
        }
    }

    /// Resize the frame to a new size, distributing new space according to the
    /// given alignments.
    pub fn resize(&mut self, target: Size, align: Axes<FixedAlignment>) {
        if self.size != target {
            let offset = align.zip_map(target - self.size, FixedAlignment::position);
            self.size = target;
            self.translate(offset.to_point());
        }
    }

    /// Move the baseline and contents of the frame by an offset.
    pub fn translate(&mut self, offset: TypstPoint) {
        if !offset.is_zero() {
            if let Some(baseline) = &mut self.baseline {
                *baseline += offset.y;
            }
            for (point, _) in Arc::make_mut(&mut self.items) {
                *point += offset;
            }
        }
    }

    /// Attach the metadata from this style chain to the frame.
    pub fn meta(&mut self, styles: TypstStyleChain, force: bool) {
        if force || !self.is_empty() {
            self.meta_iter(MetaTypstElem::data_in(styles));
        }
    }

    /// Attach metadata from an iterator.
    pub fn meta_iter(&mut self, iter: impl IntoIterator<Item = TypstMeta>) {
        let mut hide = false;
        let size = self.size;
        self.prepend_multiple(iter.into_iter().filter_map(|meta| {
            if matches!(meta, TypstMeta::Hide) {
                hide = true;
                None
            } else {
                Some((TypstPoint::zero(), TypstFrameItem::Meta(meta, size)))
            }
        }));
        if hide {
            self.hide();
        }
    }

    /// Hide all content in the frame, but keep metadata.
    pub fn hide(&mut self) {
        Arc::make_mut(&mut self.items).retain_mut(|(_, item)| match item {
            TypstFrameItem::Group(group) => {
                group.frame.hide();
                !group.frame.is_empty()
            }
            TypstFrameItem::Meta(TypstMeta::Elem(_), _) => true,
            _ => false,
        });
    }

    /// Add a background fill.
    pub fn fill(&mut self, fill: TypstPaint) {
        self.prepend(
            TypstPoint::zero(),
            TypstFrameItem::Shape(
                TypstGeometry::Rect(self.size()).filled(fill),
                TypstSynSpan::detached(),
            ),
        );
    }

    /// Add a fill and stroke with optional radius and outset to the frame.
    pub fn fill_and_stroke(
        &mut self,
        fill: Option<TypstPaint>,
        stroke: Sides<Option<TypstFixedStroke>>,
        outset: Sides<Rel<TypstAbsLength>>,
        radius: Corners<Rel<TypstAbsLength>>,
        span: TypstSynSpan,
    ) {
        let outset = outset.relative_to(self.size());
        let size = self.size() + outset.sum_by_axis();
        let pos = TypstPoint::new(-outset.left, -outset.top);
        self.prepend_multiple(
            styled_rect(size, radius, fill, stroke)
                .into_iter()
                .map(|x| (pos, TypstFrameItem::Shape(x, span))),
        )
    }

    /// Arbitrarily transform the contents of the frame.
    pub fn transform(&mut self, transform: Transform) {
        if !self.is_empty() {
            self.group(|g| g.transform = transform);
        }
    }

    /// Clip the contents of a frame to a clip path.
    ///
    /// The clip path can be the size of the frame in the case of a
    /// rectangular frame. In the case of a frame with rounded corner,
    /// this should be a path that matches the frame's outline.
    pub fn clip(&mut self, clip_path: Path) {
        if !self.is_empty() {
            self.group(|g| g.clip_path = Some(clip_path));
        }
    }

    /// Wrap the frame's contents in a group and modify that group with `f`.
    fn group<F>(&mut self, f: F)
    where
        F: FnOnce(&mut TypstGroupItem),
    {
        let mut wrapper = TypstFrame::soft(self.size);
        wrapper.baseline = self.baseline;
        let mut group = TypstGroupItem::new(std::mem::take(self));
        f(&mut group);
        wrapper.push(TypstPoint::zero(), TypstFrameItem::Group(group));
        *self = wrapper;
    }
}

/// Tools for debugging.
impl TypstFrame {
    /// Add a full size aqua background and a red baseline for debugging.
    pub fn mark_box(mut self) -> Self {
        self.mark_box_in_place();
        self
    }

    /// Debug in place. Add a full size aqua background and a red baseline for debugging.
    pub fn mark_box_in_place(&mut self) {
        self.insert(
            0,
            TypstPoint::zero(),
            TypstFrameItem::Shape(
                TypstGeometry::Rect(self.size).filled(TypstColor::TEAL.with_alpha(0.5).into()),
                TypstSynSpan::detached(),
            ),
        );
        self.insert(
            1,
            TypstPoint::with_y(self.baseline()),
            TypstFrameItem::Shape(
                TypstGeometry::Line(TypstPoint::with_x(self.size.x)).stroked(
                    TypstFixedStroke::from_pair(TypstColor::RED, TypstAbsLength::pt(1.0)),
                ),
                TypstSynSpan::detached(),
            ),
        );
    }

    /// Add a green marker at a position for debugging.
    pub fn mark_point(&mut self, pos: TypstPoint) {
        let radius = TypstAbsLength::pt(2.0);
        self.push(
            pos - TypstPoint::splat(radius),
            TypstFrameItem::Shape(
                ellipse(
                    Size::splat(2.0 * radius),
                    Some(TypstColor::GREEN.into()),
                    None,
                ),
                TypstSynSpan::detached(),
            ),
        );
    }

    /// Add a green marker line at a position for debugging.
    pub fn mark_line(&mut self, y: TypstAbsLength) {
        self.push(
            TypstPoint::with_y(y),
            TypstFrameItem::Shape(
                TypstGeometry::Line(TypstPoint::with_x(self.size.x)).stroked(
                    TypstFixedStroke::from_pair(TypstColor::GREEN, TypstAbsLength::pt(1.0)),
                ),
                TypstSynSpan::detached(),
            ),
        );
    }
}

impl Debug for TypstFrame {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("Frame ")?;
        f.debug_list()
            .entries(self.items.iter().map(|(_, item)| item))
            .finish()
    }
}

/// The hardness of a frame.
///
/// This corresponds to whether or not the frame is considered to be the
/// innermost parent of its contents. This is used to determine the coordinate
/// reference system for gradients.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FrameKind {
    /// A container which follows its parent's size.
    ///
    /// Soft frames are the default since they do not impact the layout of
    /// a gradient set on one of its children.
    #[default]
    Soft,
    /// A container which uses its own size.
    Hard,
}

impl FrameKind {
    /// Returns `true` if the frame is soft.
    pub fn is_soft(self) -> bool {
        matches!(self, Self::Soft)
    }

    /// Returns `true` if the frame is hard.
    pub fn is_hard(self) -> bool {
        matches!(self, Self::Hard)
    }
}

/// The building block frames are composed of.
#[derive(Clone, Hash)]
pub enum TypstFrameItem {
    /// A subframe with optional transformation and clipping.
    Group(TypstGroupItem),
    /// A run of shaped text.
    Text(TypstTextItem),
    /// A geometric shape with optional fill and stroke.
    Shape(TypstShape, TypstSynSpan),
    /// An image and its size.
    Image(TypstImage, Size, TypstSynSpan),
    /// Meta information and the region it applies to.
    Meta(TypstMeta, Size),
}

impl Debug for TypstFrameItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Group(group) => group.fmt(f),
            Self::Text(text) => write!(f, "{text:?}"),
            Self::Shape(shape, _) => write!(f, "{shape:?}"),
            Self::Image(image, _, _) => write!(f, "{image:?}"),
            Self::Meta(meta, _) => write!(f, "{meta:?}"),
        }
    }
}

/// A subframe with optional transformation and clipping.
#[derive(Clone, Hash)]
pub struct TypstGroupItem {
    /// The group's frame.
    pub frame: TypstFrame,
    /// A transformation to apply to the group.
    pub transform: Transform,
    /// Whether the frame should be a clipping boundary.
    pub clip_path: Option<Path>,
}

impl TypstGroupItem {
    /// Create a new group with default settings.
    pub fn new(frame: TypstFrame) -> Self {
        Self {
            frame,
            transform: Transform::identity(),
            clip_path: None,
        }
    }
}

impl Debug for TypstGroupItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("Group ")?;
        self.frame.fmt(f)
    }
}

/// A physical position in a document.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    /// The page, starting at 1.
    pub page: NonZeroUsize,
    /// The exact coordinates on the page (from the top left, as usual).
    pub point: TypstPoint,
}

cast! {
    Position,
    self => TypstValue::Dict(self.into()),
    mut dict: TypstDict => {
        let page = dict.take("page")?.cast()?;
        let x: TypstLength = dict.take("x")?.cast()?;
        let y: TypstLength = dict.take("y")?.cast()?;
        dict.finish(&["page", "x", "y"])?;
        Self { page, point: TypstPoint::new(x.abs, y.abs) }
    },
}

impl From<Position> for TypstDict {
    fn from(pos: Position) -> Self {
        dict! {
            "page" => pos.page,
            "x" => pos.point.x,
            "y" => pos.point.y,
        }
    }
}
