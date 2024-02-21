use std::any::TypeId;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{self, Sum};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Deref, DerefMut};
use std::sync::Arc;

use comemo::Prehashed;
use ecow::{eco_format, EcoString};
use serde::{Serialize, Serializer};
use smallvec::smallvec;

use crate::diag::{StrResult, TypstSourceResult};
use crate::engine::TypstEngine;
use crate::foundations::{
    elem, func, scope, ty, ElementSchemaRef, Fields, IntoTypstValue, IsTypstElem, Label, Recipe,
    RecipeIndex, Repr, Selector, Str, Style, TypstDict, TypstStyleChain, TypstStyles, TypstValue,
};
use crate::introspection::{Location, MetaTypstElem, TypstMeta};
use crate::layout::{
    AlignElem, Axes, MoveElem, Rel, Sides, TypstAlignment, TypstLength, TypstPadElem,
};
use crate::model::{EmphElem, StrongElem, TypstDestination};
use crate::syntax::TypstSynSpan;
use crate::text::UnderlineElem;
use crate::util::{fat, BitSet};

/// A piece of document content.
///
/// This type is at the heart of Typst. All markup you write and most
/// [functions]($function) you call produce content values. You can create a
/// content value by enclosing markup in square brackets. This is also how you
/// pass content to functions.
///
/// # Example
/// ```example
/// Type of *Hello!* is
/// #type([*Hello!*])
/// ```
///
/// Content can be added with the `+` operator,
/// [joined together]($scripting/#blocks) and multiplied with integers. Wherever
/// content is expected, you can also pass a [string]($str) or `{none}`.
///
/// # Representation
/// Content consists of elements with fields. When constructing an element with
/// its _element function,_ you provide these fields as arguments and when you
/// have a content value, you can access its fields with [field access
/// syntax]($scripting/#field-access).
///
/// Some fields are required: These must be provided when constructing an
/// element and as a consequence, they are always available through field access
/// on content of that type. Required fields are marked as such in the
/// documentation.
///
/// Most fields are optional: Like required fields, they can be passed to the
/// element function to configure them for a single element. However, these can
/// also be configured with [set rules]($styling/#set-rules) to apply them to
/// all elements within a scope. Optional fields are only available with field
/// access syntax when they were explicitly passed to the element function, not
/// when they result from a set rule.
///
/// Each element has a default appearance. However, you can also completely
/// customize its appearance with a [show rule]($styling/#show-rules). The show
/// rule is passed the element. It can access the element's field and produce
/// arbitrary content from it.
///
/// In the web app, you can hover over a content variable to see exactly which
/// elements the content is composed of and what fields they have.
/// Alternatively, you can inspect the output of the [`repr`]($repr) function.
#[ty(scope, cast)]
#[derive(Clone, Hash)]
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct TypstContent {
    /// The partially element-dependant inner data.
    inner: Arc<TypstContentInner<dyn IsTypstElemDyn>>,
    /// The element's source code location.
    span: TypstSynSpan,
}

/// The inner representation behind the `Arc`.
#[derive(Hash)]
struct TypstContentInner<T: ?Sized> {
    /// An optional label attached to the element.
    label: Option<Label>,
    /// The element's location which identifies it in the layouted output.
    location: Option<Location>,
    /// Manages the element during realization.
    /// - If bit 0 is set, the element is prepared.
    /// - If bit n is set, the element is guarded against the n-th show rule
    ///   recipe from the top of the style chain (counting from 1).
    lifecycle: BitSet,
    /// The element's raw data.
    elem: T,
}

impl TypstContent {
    /// Creates a new content from an element.
    pub fn new<T: IsTypstElem>(elem: T) -> Self {
        Self {
            inner: Arc::new(TypstContentInner {
                label: None,
                location: None,
                lifecycle: BitSet::new(),
                elem,
            }),
            span: TypstSynSpan::detached(),
        }
    }

    /// Creates a new empty sequence content.
    pub fn empty() -> Self {
        Self::new(SequenceElem::default())
    }

    /// Get the element of this content.
    pub fn elem(&self) -> ElementSchemaRef {
        self.inner.elem.element_schema_ref_dyn()
    }

    /// Get the span of the content.
    pub fn span(&self) -> TypstSynSpan {
        self.span
    }

    /// Set the span of the content.
    pub fn spanned(mut self, span: TypstSynSpan) -> Self {
        if self.span.is_detached() {
            self.span = span;
        }
        self
    }

    /// Get the label of the content.
    pub fn label(&self) -> Option<Label> {
        self.inner.label
    }

    /// Set the label of the content.
    pub fn labelled(mut self, label: Label) -> Self {
        self.make_mut().label = Some(label);
        self
    }

    /// Check whether a show rule recipe is disabled.
    pub fn is_guarded(&self, index: RecipeIndex) -> bool {
        self.inner.lifecycle.contains(index.0)
    }

    /// Whether this content has already been prepared.
    pub fn is_prepared(&self) -> bool {
        self.inner.lifecycle.contains(0)
    }

    /// Set the location of the content.
    pub fn set_location(&mut self, location: Location) {
        self.make_mut().location = Some(location);
    }

    /// Disable a show rule recipe.
    pub fn guarded(mut self, index: RecipeIndex) -> Self {
        self.make_mut().lifecycle.insert(index.0);
        self
    }

    /// Mark this content as prepared.
    pub fn mark_prepared(&mut self) {
        self.make_mut().lifecycle.insert(0);
    }

    /// Get a field by ID.
    ///
    /// This is the preferred way to access fields. However, you can only use it
    /// if you have set the field IDs yourself or are using the field IDs
    /// generated by the `#[elem]` macro.
    pub fn get(&self, id: u8, styles: Option<TypstStyleChain>) -> Option<TypstValue> {
        if id == 255 {
            if let Some(label) = self.label() {
                return Some(label.into_value());
            }
        }
        match styles {
            Some(styles) => self.inner.elem.field_with_styles(id, styles),
            None => self.inner.elem.field(id),
        }
    }

    /// Get a field by name.
    ///
    /// If you have access to the field IDs of the element, use [`Self::get`]
    /// instead.
    pub fn get_by_name(&self, name: &str) -> Option<TypstValue> {
        if name == "label" {
            if let Some(label) = self.label() {
                return Some(label.into_value());
            }
        }
        let id = self.elem().field_id(name)?;
        self.get(id, None)
    }

    /// Get a field by ID, returning a missing field error if it does not exist.
    ///
    /// This is the preferred way to access fields. However, you can only use it
    /// if you have set the field IDs yourself or are using the field IDs
    /// generated by the `#[elem]` macro.
    pub fn field(&self, id: u8) -> StrResult<TypstValue> {
        self.get(id, None)
            .ok_or_else(|| missing_field(self.elem().field_name(id).unwrap()))
    }

    /// Get a field by name, returning a missing field error if it does not
    /// exist.
    ///
    /// If you have access to the field IDs of the element, use [`Self::field`]
    /// instead.
    pub fn field_by_name(&self, name: &str) -> StrResult<TypstValue> {
        self.get_by_name(name).ok_or_else(|| missing_field(name))
    }

    /// Resolve all fields with the styles and save them in-place.
    pub fn materialize(&mut self, styles: TypstStyleChain) {
        self.make_mut().elem.materialize(styles);
    }

    /// Create a new sequence element from multiples elements.
    pub fn sequence(iter: impl IntoIterator<Item = Self>) -> Self {
        let mut iter = iter.into_iter();
        let Some(first) = iter.next() else {
            return Self::empty();
        };
        let Some(second) = iter.next() else {
            return first;
        };
        SequenceElem::new(
            std::iter::once(Prehashed::new(first))
                .chain(std::iter::once(Prehashed::new(second)))
                .chain(iter.map(Prehashed::new))
                .collect(),
        )
        .into()
    }

    /// Whether the contained element is of type `T`.
    pub fn is<T: IsTypstElem>(&self) -> bool {
        self.inner.elem.type_id_dyn() == TypeId::of::<T>()
    }

    /// Downcasts the element to a packed value.
    pub fn to_packed<T: IsTypstElem>(&self) -> Option<&TypstContentRefined<T>> {
        TypstContentRefined::from_ref(self)
    }

    /// Downcasts the element to a mutable packed value.
    pub fn to_packed_mut<T: IsTypstElem>(&mut self) -> Option<&mut TypstContentRefined<T>> {
        TypstContentRefined::from_mut(self)
    }

    /// Downcasts the element into an owned packed value.
    pub fn into_packed<T: IsTypstElem>(self) -> Result<TypstContentRefined<T>, Self> {
        TypstContentRefined::from_owned(self)
    }

    /// Extract the raw underlying element.
    pub fn unpack<T: IsTypstElem>(self) -> Result<T, Self> {
        self.into_packed::<T>().map(TypstContentRefined::unpack)
    }

    /// Makes sure the content is not shared and returns a mutable reference to
    /// the inner data.
    fn make_mut(&mut self) -> &mut TypstContentInner<dyn IsTypstElemDyn> {
        let arc = &mut self.inner;
        if Arc::strong_count(arc) > 1 || Arc::weak_count(arc) > 0 {
            *self = arc.elem.clone_dyn(arc, self.span);
        }
        Arc::get_mut(&mut self.inner).unwrap()
    }

    /// Whether the contained element has the given capability.
    pub fn can<C>(&self) -> bool
    where
        C: ?Sized + 'static,
    {
        self.elem().can::<C>()
    }

    /// Cast to a trait object if the contained element has the given
    /// capability.
    pub fn with<C>(&self) -> Option<&C>
    where
        C: ?Sized + 'static,
    {
        // Safety: The vtable comes from the `Capable` implementation which
        // guarantees to return a matching vtable for `TypstContentRefined<T>` and `C`.
        // Since any `TypstContentRefined<T>` is a repr(transparent) `Content`, we can also
        // use a `*const Content` pointer.
        let vtable = self.elem().vtable()(TypeId::of::<C>())?;
        let data = self as *const TypstContent as *const ();
        Some(unsafe { &*fat::from_raw_parts(data, vtable) })
    }

    /// Cast to a mutable trait object if the contained element has the given
    /// capability.
    pub fn with_mut<C>(&mut self) -> Option<&mut C>
    where
        C: ?Sized + 'static,
    {
        // Safety: The vtable comes from the `Capable` implementation which
        // guarantees to return a matching vtable for `TypstContentRefined<T>` and `C`.
        // Since any `TypstContentRefined<T>` is a repr(transparent) `Content`, we can also
        // use a `*const Content` pointer.
        //
        // The resulting trait object contains an `&mut TypstContentRefined<T>`. We do _not_
        // need to ensure that we hold the only reference to the `Arc` here
        // because `TypstContentRefined<T>`'s DerefMut impl will take care of that if
        // mutable access is required.
        let vtable = self.elem().vtable()(TypeId::of::<C>())?;
        let data = self as *mut TypstContent as *mut ();
        Some(unsafe { &mut *fat::from_raw_parts_mut(data, vtable) })
    }

    /// Whether the content is an empty sequence.
    pub fn is_empty(&self) -> bool {
        let Some(sequence) = self.to_packed::<SequenceElem>() else {
            return false;
        };

        sequence.children.is_empty()
    }

    /// Whether the content is a sequence.
    pub fn is_sequence(&self) -> bool {
        self.is::<SequenceElem>()
    }

    /// Access the children if this is a sequence.
    pub fn to_sequence(&self) -> Option<impl Iterator<Item = &Prehashed<TypstContent>>> {
        let sequence = self.to_packed::<SequenceElem>()?;
        Some(sequence.children.iter())
    }

    /// Also auto expands sequence of sequences into flat sequence
    pub fn sequence_recursive_for_each(&self, f: &mut impl FnMut(&Self)) {
        if let Some(children) = self.to_sequence() {
            children.for_each(|c| c.sequence_recursive_for_each(f));
        } else {
            f(self);
        }
    }

    /// Access the child and styles.
    pub fn to_styled(&self) -> Option<(&TypstContent, &TypstStyles)> {
        let styled = self.to_packed::<StyledTypstElement>()?;
        let child = styled.child();
        let styles = styled.styles();
        Some((child, styles))
    }

    /// Style this content with a recipe, eagerly applying it if possible.
    pub fn styled_with_recipe(
        self,
        engine: &mut TypstEngine,
        recipe: Recipe,
    ) -> TypstSourceResult<Self> {
        if recipe.selector.is_none() {
            recipe.apply(engine, self)
        } else {
            Ok(self.styled(recipe))
        }
    }

    /// Repeat this content `count` times.
    pub fn repeat(&self, count: usize) -> Self {
        Self::sequence(std::iter::repeat_with(|| self.clone()).take(count))
    }

    /// Style this content with a style entry.
    pub fn styled(mut self, style: impl Into<Style>) -> Self {
        if let Some(style_elem) = self.to_packed_mut::<StyledTypstElement>() {
            style_elem.styles.apply_one(style.into());
            self
        } else {
            self.styled_with_map(style.into().into())
        }
    }

    /// Style this content with a full style map.
    pub fn styled_with_map(mut self, styles: TypstStyles) -> Self {
        if styles.is_empty() {
            return self;
        }

        if let Some(style_elem) = self.to_packed_mut::<StyledTypstElement>() {
            style_elem.styles.apply(styles);
            self
        } else {
            StyledTypstElement::new(Prehashed::new(self), styles).into()
        }
    }

    /// Queries the content tree for all elements that match the given selector.
    ///
    /// Elements produced in `show` rules will not be included in the results.
    pub fn query(&self, selector: Selector) -> Vec<TypstContent> {
        let mut results = Vec::new();
        self.traverse(&mut |element| {
            if selector.matches(&element, None) {
                results.push(element);
            }
        });
        results
    }

    /// Queries the content tree for the first element that match the given
    /// selector.
    ///
    /// Elements produced in `show` rules will not be included in the results.
    pub fn query_first(&self, selector: Selector) -> Option<TypstContent> {
        let mut result = None;
        self.traverse(&mut |element| {
            if result.is_none() && selector.matches(&element, None) {
                result = Some(element);
            }
        });
        result
    }

    /// Extracts the plain text of this content.
    pub fn plain_text(&self) -> EcoString {
        let mut text = EcoString::new();
        self.traverse(&mut |element| {
            if let Some(textable) = element.with::<dyn PlainText>() {
                textable.plain_text(&mut text);
            }
        });
        text
    }

    /// Traverse this content.
    fn traverse<F>(&self, f: &mut F)
    where
        F: FnMut(TypstContent),
    {
        f(self.clone());

        self.inner
            .elem
            .fields()
            .into_iter()
            .for_each(|(_, value)| walk_value(value, f));

        /// Walks a given value to find any content that matches the selector.
        fn walk_value<F>(value: TypstValue, f: &mut F)
        where
            F: FnMut(TypstContent),
        {
            match value {
                TypstValue::Content(content) => content.traverse(f),
                TypstValue::Array(array) => {
                    for value in array {
                        walk_value(value, f);
                    }
                }
                _ => {}
            }
        }
    }
}

impl TypstContent {
    /// Strongly emphasize this content.
    pub fn strong(self) -> Self {
        StrongElem::new(self).pack()
    }

    /// Emphasize this content.
    pub fn emph(self) -> Self {
        EmphElem::new(self).pack()
    }

    /// Underline this content.
    pub fn underlined(self) -> Self {
        UnderlineElem::new(self).pack()
    }

    /// Link the content somewhere.
    pub fn linked(self, dest: TypstDestination) -> Self {
        self.styled(MetaTypstElem::set_data(smallvec![TypstMeta::Link(dest)]))
    }

    /// Make the content linkable by `.linked(Destination::Location(loc))`.
    ///
    /// Should be used in combination with [`Location::variant`].
    pub fn backlinked(self, loc: Location) -> Self {
        let mut backlink = TypstContent::empty();
        backlink.set_location(loc);
        self.styled(MetaTypstElem::set_data(smallvec![TypstMeta::Elem(
            backlink
        )]))
    }

    /// Set alignments for this content.
    pub fn aligned(self, align: TypstAlignment) -> Self {
        self.styled(AlignElem::set_alignment(align))
    }

    /// Pad this content at the sides.
    pub fn padded(self, padding: Sides<Rel<TypstLength>>) -> Self {
        TypstPadElem::new(self)
            .with_left(padding.left)
            .with_top(padding.top)
            .with_right(padding.right)
            .with_bottom(padding.bottom)
            .pack()
    }

    /// Transform this content's contents without affecting layout.
    pub fn moved(self, delta: Axes<Rel<TypstLength>>) -> Self {
        MoveElem::new(self).with_dx(delta.x).with_dy(delta.y).pack()
    }
}

#[scope]
impl TypstContent {
    /// The content's element function. This function can be used to create the element
    /// contained in this content. It can be used in set and show rules for the
    /// element. Can be compared with global functions to check whether you have
    /// a specific
    /// kind of element.
    #[func]
    pub fn func(&self) -> ElementSchemaRef {
        self.elem()
    }

    /// Whether the content has the specified field.
    #[func]
    pub fn has(
        &self,
        /// The field to look for.
        field: Str,
    ) -> bool {
        if field.as_str() == "label" {
            return self.label().is_some();
        }

        let Some(id) = self.elem().field_id(&field) else {
            return false;
        };

        self.inner.elem.has(id)
    }

    /// Access the specified field on the content. Returns the default value if
    /// the field does not exist or fails with an error if no default value was
    /// specified.
    #[func]
    pub fn at(
        &self,
        /// The field to access.
        field: Str,
        /// A default value to return if the field does not exist.
        #[named]
        default: Option<TypstValue>,
    ) -> StrResult<TypstValue> {
        self.get_by_name(&field)
            .or(default)
            .ok_or_else(|| missing_field_no_default(&field))
    }

    /// Returns the fields of this content.
    ///
    /// ```example
    /// #rect(
    ///   width: 10cm,
    ///   height: 10cm,
    /// ).fields()
    /// ```
    #[func]
    pub fn fields(&self) -> TypstDict {
        let mut dict = self.inner.elem.fields();
        if let Some(label) = self.label() {
            dict.insert("label".into(), label.into_value());
        }
        dict
    }

    /// The location of the content. This is only available on content returned
    /// by [query]($query) or provided by a
    /// [show rule]($reference/styling/#show-rules), for other content it will
    /// be `{none}`. The resulting location can be used with
    /// [counters]($counter), [state]($state) and [queries]($query).
    #[func]
    pub fn location(&self) -> Option<Location> {
        self.inner.location
    }
}

impl Default for TypstContent {
    fn default() -> Self {
        Self::empty()
    }
}

impl Debug for TypstContent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.inner.elem.fmt(f)
    }
}

impl<T: IsTypstElem> From<T> for TypstContent {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl PartialEq for TypstContent {
    fn eq(&self, other: &Self) -> bool {
        // Additional short circuit for different elements.
        self.elem() == other.elem() && self.inner.elem.eq_dyn(other)
    }
}

impl Repr for TypstContent {
    fn repr(&self) -> EcoString {
        self.inner.elem.repr()
    }
}

impl Add for TypstContent {
    type Output = Self;

    fn add(self, mut rhs: Self) -> Self::Output {
        let mut lhs = self;
        match (
            lhs.to_packed_mut::<SequenceElem>(),
            rhs.to_packed_mut::<SequenceElem>(),
        ) {
            (Some(seq_lhs), Some(rhs)) => {
                seq_lhs.children.extend(rhs.children.iter().cloned());
                lhs
            }
            (Some(seq_lhs), None) => {
                seq_lhs.children.push(Prehashed::new(rhs));
                lhs
            }
            (None, Some(rhs_seq)) => {
                rhs_seq.children.insert(0, Prehashed::new(lhs));
                rhs
            }
            (None, None) => Self::sequence([lhs, rhs]),
        }
    }
}

impl<'a> Add<&'a Self> for TypstContent {
    type Output = Self;

    fn add(self, rhs: &'a Self) -> Self::Output {
        let mut lhs = self;
        match (
            lhs.to_packed_mut::<SequenceElem>(),
            rhs.to_packed::<SequenceElem>(),
        ) {
            (Some(seq_lhs), Some(rhs)) => {
                seq_lhs.children.extend(rhs.children.iter().cloned());
                lhs
            }
            (Some(seq_lhs), None) => {
                seq_lhs.children.push(Prehashed::new(rhs.clone()));
                lhs
            }
            (None, Some(_)) => {
                let mut rhs = rhs.clone();
                rhs.to_packed_mut::<SequenceElem>()
                    .unwrap()
                    .children
                    .insert(0, Prehashed::new(lhs));
                rhs
            }
            (None, None) => Self::sequence([lhs, rhs.clone()]),
        }
    }
}

impl AddAssign for TypstContent {
    fn add_assign(&mut self, rhs: Self) {
        *self = std::mem::take(self) + rhs;
    }
}

impl AddAssign<&Self> for TypstContent {
    fn add_assign(&mut self, rhs: &Self) {
        *self = std::mem::take(self) + rhs;
    }
}

impl Sum for TypstContent {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::sequence(iter)
    }
}

impl Serialize for TypstContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(
            iter::once(("func".into(), self.func().name().into_value())).chain(self.fields()),
        )
    }
}

/// The trait that combines all the other traits into a trait object.
trait IsTypstElemDyn: Debug + Repr + Fields + Send + Sync + 'static {
    fn type_id_dyn(&self) -> TypeId;
    fn element_schema_ref_dyn(&self) -> ElementSchemaRef;
    fn clone_dyn(
        &self,
        inner: &TypstContentInner<dyn IsTypstElemDyn>,
        span: TypstSynSpan,
    ) -> TypstContent;
    fn hash_dyn(&self, hasher: &mut dyn Hasher);
    fn eq_dyn(&self, other: &TypstContent) -> bool;
}

impl<T: IsTypstElem> IsTypstElemDyn for T {
    fn type_id_dyn(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn element_schema_ref_dyn(&self) -> ElementSchemaRef {
        Self::elem()
    }

    fn clone_dyn(
        &self,
        inner: &TypstContentInner<dyn IsTypstElemDyn>,
        span: TypstSynSpan,
    ) -> TypstContent {
        TypstContent {
            inner: Arc::new(TypstContentInner {
                label: inner.label,
                location: inner.location,
                lifecycle: inner.lifecycle.clone(),
                elem: self.clone(),
            }),
            span,
        }
    }

    fn hash_dyn(&self, mut state: &mut dyn Hasher) {
        TypeId::of::<Self>().hash(&mut state);
        self.hash(&mut state);
    }

    fn eq_dyn(&self, other: &TypstContent) -> bool {
        let Some(other) = other.to_packed::<Self>() else {
            return false;
        };
        *self == **other
    }
}

impl Hash for dyn IsTypstElemDyn {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_dyn(state);
    }
}

/// A packed element of a static type.
#[derive(Clone, PartialEq, Hash)]
#[repr(transparent)]
pub struct TypstContentRefined<T: IsTypstElem>(
    /// Invariant: Must be of type `T`.
    TypstContent,
    PhantomData<T>,
);

impl<T: IsTypstElem> TypstContentRefined<T> {
    /// Pack element while retaining its static type.
    pub fn new(element: T) -> Self {
        // Safety: The element is known to be of type `T`.
        TypstContentRefined(element.pack(), PhantomData)
    }

    /// Try to cast type-erased content into a statically known packed element.
    pub fn from_ref(content: &TypstContent) -> Option<&Self> {
        if content.is::<T>() {
            // Safety:
            // - We have checked the type.
            // - TypstContentRefined<T> is repr(transparent).
            return Some(unsafe { std::mem::transmute(content) });
        }
        None
    }

    /// Try to cast type-erased content into a statically known packed element.
    pub fn from_mut(content: &mut TypstContent) -> Option<&mut Self> {
        if content.is::<T>() {
            // Safety:
            // - We have checked the type.
            // - TypstContentRefined<T> is repr(transparent).
            return Some(unsafe { std::mem::transmute(content) });
        }
        None
    }

    /// Try to cast type-erased content into a statically known packed element.
    pub fn from_owned(content: TypstContent) -> Result<Self, TypstContent> {
        if content.is::<T>() {
            // Safety:
            // - We have checked the type.
            // - TypstContentRefined<T> is repr(transparent).
            return Ok(unsafe { std::mem::transmute(content) });
        }
        Err(content)
    }

    /// Pack back into content.
    pub fn pack(self) -> TypstContent {
        self.0
    }

    /// Extract the raw underlying element.
    pub fn unpack(self) -> T {
        // This function doesn't yet need owned self, but might in the future.
        (*self).clone()
    }

    /// The element's span.
    pub fn span(&self) -> TypstSynSpan {
        self.0.span()
    }

    /// Set the span of the element.
    pub fn spanned(self, span: TypstSynSpan) -> Self {
        Self(self.0.spanned(span), PhantomData)
    }

    /// Accesses the label of the element.
    pub fn label(&self) -> Option<Label> {
        self.0.label()
    }

    /// Accesses the location of the element.
    pub fn location(&self) -> Option<Location> {
        self.0.location()
    }

    /// Sets the location of the element.
    pub fn set_location(&mut self, location: Location) {
        self.0.set_location(location);
    }
}

impl<T: IsTypstElem> AsRef<T> for TypstContentRefined<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: IsTypstElem> AsMut<T> for TypstContentRefined<T> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: IsTypstElem> Deref for TypstContentRefined<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety:
        // - TypstContentRefined<T> guarantees that the content trait object wraps
        //   an element of type `T`.
        // - This downcast works the same way as dyn Any's does. We can't reuse
        //   that one because we don't want to pay the cost for every deref.
        let elem = &self.0.inner.elem;
        unsafe { &*(elem as *const dyn IsTypstElemDyn as *const T) }
    }
}

impl<T: IsTypstElem> DerefMut for TypstContentRefined<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety:
        // - TypstContentRefined<T> guarantees that the content trait object wraps
        //   an element of type `T`.
        // - We have guaranteed unique access thanks to `make_mut`.
        // - This downcast works the same way as dyn Any's does. We can't reuse
        //   that one because we don't want to pay the cost for every deref.
        let elem = &mut self.0.make_mut().elem;
        unsafe { &mut *(elem as *mut dyn IsTypstElemDyn as *mut T) }
    }
}

impl<T: IsTypstElem + Debug> Debug for TypstContentRefined<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Defines the element for sequences.
#[elem(Debug, Repr, PartialEq)]
struct SequenceElem {
    #[required]
    children: Vec<Prehashed<TypstContent>>,
}

impl Debug for SequenceElem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Sequence ")?;
        f.debug_list().entries(&self.children).finish()
    }
}

// Derive is currently incompatible with `elem` macro.
#[allow(clippy::derivable_impls)]
impl Default for SequenceElem {
    fn default() -> Self {
        Self {
            children: Default::default(),
        }
    }
}

impl PartialEq for SequenceElem {
    fn eq(&self, other: &Self) -> bool {
        self.children
            .iter()
            .map(|c| &**c)
            .eq(other.children.iter().map(|c| &**c))
    }
}

impl Repr for SequenceElem {
    fn repr(&self) -> EcoString {
        if self.children.is_empty() {
            "[]".into()
        } else {
            eco_format!(
                "[{}]",
                crate::foundations::repr::pretty_array_like(
                    &self
                        .children
                        .iter()
                        .map(|c| c.inner.elem.repr())
                        .collect::<Vec<_>>(),
                    false
                )
            )
        }
    }
}

/// Defines the `ElemFunc` for styled elements.
#[elem(Debug, Repr, PartialEq)]
struct StyledTypstElement {
    #[required]
    child: Prehashed<TypstContent>,
    #[required]
    styles: TypstStyles,
}

impl Debug for StyledTypstElement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for style in self.styles.iter() {
            writeln!(f, "#{style:?}")?;
        }
        self.child.fmt(f)
    }
}

impl PartialEq for StyledTypstElement {
    fn eq(&self, other: &Self) -> bool {
        *self.child == *other.child
    }
}

impl Repr for StyledTypstElement {
    fn repr(&self) -> EcoString {
        eco_format!("styled(child: {}, ..)", self.child.repr())
    }
}

/// Tries to extract the plain-text representation of the element.
pub trait PlainText {
    /// Write this element's plain text into the given buffer.
    fn plain_text(&self, text: &mut EcoString);
}

/// The missing field access error message.
#[cold]
fn missing_field(field: &str) -> EcoString {
    eco_format!("content does not contain field {}", field.repr())
}

/// The missing field access error message when no default value was given.
#[cold]
fn missing_field_no_default(field: &str) -> EcoString {
    eco_format!(
        "content does not contain field {} and \
         no default value was specified",
        field.repr()
    )
}