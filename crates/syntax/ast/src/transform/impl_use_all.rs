use crate::*;
use atom::symbol::{Symbol, SymbolKind};
use entity_route::{EntityRoute, EntityRouteKind};
use text::TextRange;

impl<'a> AstTransformer<'a> {
    pub(super) fn use_all(&mut self, parent: EntityRoutePtr, range: TextRange) -> AstResult<()> {
        self.symbols.extend(
            self.db
                .subscope_table(parent)
                .map_err(|_| error!("scope not found", range))?
                .entries
                .iter()
                .filter_map(|entry| {
                    entry.ident.map(|ident| Symbol {
                        ident: ident.into(),
                        kind: SymbolKind::EntityRoute(self.db.intern_entity_route(EntityRoute {
                            kind: EntityRouteKind::Child { parent, ident },
                            generic_arguments: vec![],
                        })),
                    })
                }),
        );
        Ok(())
    }

    pub(super) fn use_route(&mut self, route: EntityRoutePtr) -> AstResult<()> {
        if route.generic_arguments.len() != 0 {
            todo!()
        }
        self.symbols.push(Symbol {
            ident: route.ident().custom(),
            kind: SymbolKind::EntityRoute(route),
        });
        Ok(())
    }
}
