[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn(core::mem::Ref mnist_classifier::line_segment_sketch::LineSegmentSketch) -> [] mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
        ),
    ),
]