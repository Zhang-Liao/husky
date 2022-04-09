use vec_map::HasKey;

use super::*;

pub(crate) fn enum_decl(
    db: &dyn DeclQueryGroup,
    entity_route_kind: EntityRouteKind,
    generic_placeholders: IdentDict<GenericPlaceholder>,
    children: AstIter,
) -> InferResultArc<TyDecl> {
    let mut variants = VecDict::default();
    let mut traits = Vec::new();
    for subitem in children {
        match subitem.value.as_ref()?.kind {
            AstKind::EnumVariantDefnHead {
                ident,
                variant_class: ref raw_variant_kind,
            } => variants.insert_new(EnumVariantDecl {
                ident,
                variant: match raw_variant_kind {
                    EnumVariantKind::Constant => EnumVariantDeclVariant::Constant,
                },
            }),
            _ => panic!(),
        }
    }
    Ok(Arc::new(TyDecl {
        this_type: todo!(),
        generic_placeholders,
        traits,
        fields: todo!(),
        methods: todo!(),
        variants,
        kind: TyKind::Enum,
    }))
    // Ok(Arc::new(TyDecl::new(
    //     db,
    //     entity_route_kind,
    //     generic_placeholders,
    //     traits,
    //     TyKind::Enum { variants },
    // )))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumVariantDecl {
    ident: CustomIdentifier,
    variant: EnumVariantDeclVariant,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantDeclVariant {
    Constant,
}

impl HasKey<CustomIdentifier> for EnumVariantDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}
