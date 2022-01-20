mod alias;
mod intern;
mod kind;

pub use alias::ScopeAliasTable;
use common::*;
use file::FileId;
pub use intern::{new_scope_interner, InternScope, ScopeId, ScopeInterner};
pub use kind::ScopeKind;

use interpret::Compiled;
use word::{CustomIdentifier, Identifier, ReservedIdentifier};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    pub route: ScopeRoute,
    pub generics: Vec<GenericArgument>,
}

impl std::fmt::Debug for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.route {
            ScopeRoute::Reserved { ident } => ident.fmt(f)?,
            ScopeRoute::Package { main, ident } => {
                f.write_str("(package ")?;
                main.fmt(f)?;
                f.write_str(") ")?;
                ident.fmt(f)?
            }
            ScopeRoute::ChildScope { parent, ident } => {
                parent.fmt(f)?;
                f.write_str("::")?;
                ident.fmt(f)?
            }
        };
        if self.generics.len() > 0 {
            todo!()
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericArgument {
    Const(usize),
    Scope(ScopeId),
}

impl From<usize> for GenericArgument {
    fn from(size: usize) -> Self {
        Self::Const(size)
    }
}

impl From<ScopeId> for GenericArgument {
    fn from(scope: ScopeId) -> Self {
        GenericArgument::Scope(scope)
    }
}

impl From<ReservedIdentifier> for ScopeRoute {
    fn from(ident: ReservedIdentifier) -> Self {
        Self::Reserved { ident }
    }
}

impl From<&ReservedIdentifier> for ScopeRoute {
    fn from(ident: &ReservedIdentifier) -> Self {
        Self::Reserved { ident: *ident }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeRoute {
    Reserved {
        ident: ReservedIdentifier,
    },
    Package {
        main: FileId,
        ident: CustomIdentifier,
    },
    ChildScope {
        parent: ScopeId,
        ident: CustomIdentifier,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinScopeData {
    pub scope_kind: ScopeKind,
    pub subscopes: &'static [(&'static str, &'static BuiltinScopeData)],
    pub signature: ScopeSignature,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScopeSignature {
    Func(FuncSignature),
    Module,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FuncSignature {
    pub inputs: Vec<ScopeId>,
    pub output: ScopeId,
    pub compiled: Option<Compiled>,
}

impl Scope {
    pub fn package(main: FileId, ident: CustomIdentifier) -> Self {
        Scope {
            route: ScopeRoute::Package { main, ident },
            generics: Vec::new(),
        }
    }
    pub fn child_scope(
        parent: ScopeId,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> Scope {
        Scope {
            route: ScopeRoute::ChildScope { parent, ident },
            generics,
        }
    }

    pub fn builtin(ident: ReservedIdentifier, generic_arguments: Vec<GenericArgument>) -> Scope {
        Scope {
            route: ScopeRoute::Reserved { ident },
            generics: generic_arguments,
        }
    }

    pub fn vec(element: GenericArgument) -> Self {
        Self::builtin(ReservedIdentifier::Vector, vec![element])
    }

    pub fn array(element: GenericArgument, size: usize) -> Self {
        Self::builtin(ReservedIdentifier::Array, vec![element, size.into()])
    }

    pub fn tuple_or_void(args: Vec<GenericArgument>) -> Self {
        Scope::builtin(
            if args.len() > 0 {
                ReservedIdentifier::Tuple
            } else {
                ReservedIdentifier::Void
            },
            args,
        )
    }

    pub fn default_func_type(args: Vec<GenericArgument>) -> Self {
        Scope::builtin(word::default_func_type(), args)
    }
}

impl From<ReservedIdentifier> for Scope {
    fn from(ident: ReservedIdentifier) -> Self {
        Self::builtin(ident, Vec::new())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeSource {
    Builtin(&'static BuiltinScopeData),
    WithinBuiltinModule,
    WithinModule {
        file_id: FileId,
        token_group_index: usize, // None means the whole file
    },
    Module {
        file_id: FileId,
    },
}

impl ScopeSource {
    pub fn from_file(file_id: FileId, token_group_index: usize) -> ScopeSource {
        ScopeSource::WithinModule {
            file_id,
            token_group_index: token_group_index,
        }
    }
}

impl From<&'static BuiltinScopeData> for ScopeSource {
    fn from(data: &'static BuiltinScopeData) -> Self {
        Self::Builtin(data)
    }
}
