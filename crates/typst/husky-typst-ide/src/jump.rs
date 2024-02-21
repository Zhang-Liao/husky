use std::num::NonZeroUsize;

use ecow::EcoString;
use husky_typst::introspection::TypstMeta;
use husky_typst::layout::{Position, Size, TypstFrame, TypstFrameItem, TypstPoint};
use husky_typst::model::{TypstDestination, TypstDocument};
use husky_typst::syntax::{FileId, LinkedNode, Source, TypstSynSpan, TypstSyntaxKind};
use husky_typst::visualize::TypstGeometry;
use husky_typst::IsTypstWorld;

/// Where to [jump](jump_from_click) to.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Jump {
    /// Jump to a position in a source file.
    Source(FileId, usize),
    /// Jump to an external URL.
    Url(EcoString),
    /// Jump to a point on a page.
    Position(Position),
}

impl Jump {
    fn from_span(world: &dyn IsTypstWorld, span: TypstSynSpan) -> Option<Self> {
        let id = span.id()?;
        let source = world.source(id).ok()?;
        let node = source.find(span)?;
        Some(Self::Source(id, node.offset()))
    }
}

/// Determine where to jump to based on a click in a frame.
pub fn jump_from_click(
    world: &dyn IsTypstWorld,
    document: &TypstDocument,
    frame: &TypstFrame,
    click: TypstPoint,
) -> Option<Jump> {
    // Try to find a link first.
    for (pos, item) in frame.items() {
        if let TypstFrameItem::Meta(TypstMeta::Link(dest), size) = item {
            if is_in_rect(*pos, *size, click) {
                return Some(match dest {
                    TypstDestination::Url(url) => Jump::Url(url.clone()),
                    TypstDestination::Position(pos) => Jump::Position(*pos),
                    TypstDestination::Location(loc) => {
                        Jump::Position(document.introspector.position(*loc))
                    }
                });
            }
        }
    }

    // If there's no link, search for a jump target.
    for (mut pos, item) in frame.items().rev() {
        match item {
            TypstFrameItem::Group(group) => {
                // TODO: Handle transformation.
                if let Some(span) = jump_from_click(world, document, &group.frame, click - pos) {
                    return Some(span);
                }
            }

            TypstFrameItem::Text(text) => {
                for glyph in &text.glyphs {
                    let width = glyph.x_advance.at(text.size);
                    if is_in_rect(
                        TypstPoint::new(pos.x, pos.y - text.size),
                        Size::new(width, text.size),
                        click,
                    ) {
                        let (span, span_offset) = glyph.span;
                        let Some(id) = span.id() else { continue };
                        let source = world.source(id).ok()?;
                        let node = source.find(span)?;
                        let pos = if node.kind() == TypstSyntaxKind::Text {
                            let range = node.range();
                            let mut offset = range.start + usize::from(span_offset);
                            if (click.x - pos.x) > width / 2.0 {
                                offset += glyph.range().len();
                            }
                            offset.min(range.end)
                        } else {
                            node.offset()
                        };
                        return Some(Jump::Source(source.id(), pos));
                    }

                    pos.x += width;
                }
            }

            TypstFrameItem::Shape(shape, span) => {
                let TypstGeometry::Rect(size) = shape.geometry else {
                    continue;
                };
                if is_in_rect(pos, size, click) {
                    return Jump::from_span(world, *span);
                }
            }

            TypstFrameItem::Image(_, size, span) if is_in_rect(pos, *size, click) => {
                return Jump::from_span(world, *span);
            }

            _ => {}
        }
    }

    None
}

/// Find the output location in the document for a cursor position.
pub fn jump_from_cursor(
    document: &TypstDocument,
    source: &Source,
    cursor: usize,
) -> Option<Position> {
    let node = LinkedNode::new(source.root()).leaf_at(cursor)?;
    if node.kind() != TypstSyntaxKind::Text {
        return None;
    }

    let span = node.span();
    for (i, page) in document.pages.iter().enumerate() {
        if let Some(pos) = find_in_frame(&page.frame, span) {
            return Some(Position {
                page: NonZeroUsize::new(i + 1).unwrap(),
                point: pos,
            });
        }
    }

    None
}

/// Find the position of a span in a frame.
fn find_in_frame(frame: &TypstFrame, span: TypstSynSpan) -> Option<TypstPoint> {
    for (mut pos, item) in frame.items() {
        if let TypstFrameItem::Group(group) = item {
            // TODO: Handle transformation.
            if let Some(point) = find_in_frame(&group.frame, span) {
                return Some(point + pos);
            }
        }

        if let TypstFrameItem::Text(text) = item {
            for glyph in &text.glyphs {
                if glyph.span.0 == span {
                    return Some(pos);
                }
                pos.x += glyph.x_advance.at(text.size);
            }
        }
    }

    None
}

/// Whether a rectangle with the given size at the given position contains the
/// click position.
fn is_in_rect(pos: TypstPoint, size: Size, click: TypstPoint) -> bool {
    pos.x <= click.x && pos.x + size.x >= click.x && pos.y <= click.y && pos.y + size.y >= click.y
}
