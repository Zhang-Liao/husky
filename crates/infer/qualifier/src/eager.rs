use crate::*;
use infer_error::*;
use std::fmt::Write;
use test_utils::{TestDisplay, TestDisplayConfig};
use word::RootIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EagerQualifiedTy {
    pub qual: EagerQualifier,
    pub ty: EntityRoutePtr,
}

impl TestDisplay for EagerQualifiedTy {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}{: <12?}{} {}{:?}{}",
                print_utils::PINK,
                self.qual,
                print_utils::RESET,
                print_utils::GREEN,
                self.ty,
                print_utils::RESET,
            )
            .unwrap()
        } else {
            write!(result, "{: <12?} {:?}", self.qual, self.ty,).unwrap()
        }
    }
}

impl EagerQualifiedTy {
    pub(crate) fn ty_qualified_ty() -> Self {
        Self {
            qual: EagerQualifier::GlobalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TypeType),
        }
    }
    pub(crate) fn from_parameter_use(
        db: &dyn InferQualifiedTyQueryGroup,
        input_liason: InputLiason,
        ty: EntityRoutePtr,
        contract: EagerContract,
    ) -> InferResult<Self> {
        Ok(EagerQualifiedTy::new(
            EagerQualifier::from_parameter_use(input_liason, db.is_copyable(ty)?, contract)?,
            ty,
        ))
    }

    pub(crate) fn new(qual: EagerQualifier, ty: EntityRoutePtr) -> Self {
        emsg_once!("handle ref");
        Self { qual, ty }
    }

    pub(crate) fn init_variable_qualified_ty(self, init_kind: InitKind) -> InferResult<Self> {
        let qual = match init_kind {
            InitKind::Let => match self.qual {
                EagerQualifier::Copyable | EagerQualifier::CopyableMut => EagerQualifier::Copyable,
                EagerQualifier::PureRef => EagerQualifier::PureRef,
                EagerQualifier::LocalRef => EagerQualifier::LocalRef,
                EagerQualifier::Transient | EagerQualifier::OwnedMut => EagerQualifier::Owned,
                EagerQualifier::Owned => todo!(),
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRefMut => todo!(),
            },
            InitKind::Var => match self.qual {
                EagerQualifier::Copyable | EagerQualifier::CopyableMut => {
                    EagerQualifier::CopyableMut
                }
                EagerQualifier::PureRef => todo!(),
                EagerQualifier::LocalRef => todo!(),
                EagerQualifier::Transient => EagerQualifier::OwnedMut,
                EagerQualifier::Owned => todo!(),
                EagerQualifier::OwnedMut => todo!(),
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRefMut => todo!(),
            },
            InitKind::Decl => match self.qual {
                EagerQualifier::Copyable => EagerQualifier::Copyable,
                EagerQualifier::CopyableMut => panic!(),
                EagerQualifier::PureRef => EagerQualifier::PureRef,
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRef => todo!(),
                EagerQualifier::Transient => EagerQualifier::Owned,
                EagerQualifier::Owned => todo!(),
                EagerQualifier::OwnedMut => panic!(),
                EagerQualifier::LocalRefMut => todo!(),
            },
        };
        Ok(Self { qual, ty: self.ty })
    }

    pub fn is_implicitly_castable_to_output(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        output_liason: OutputLiason,
        output_ty: EntityRoutePtr,
    ) -> bool {
        if !db.is_implicitly_castable(self.ty, output_ty) {
            return false;
        }
        match output_liason {
            OutputLiason::Transfer => match self.qual {
                EagerQualifier::PureRef | EagerQualifier::LocalRef => false,
                EagerQualifier::Transient
                | EagerQualifier::Copyable
                | EagerQualifier::CopyableMut
                | EagerQualifier::Owned
                | EagerQualifier::OwnedMut => true,
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRefMut => todo!(),
            },
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }

    pub fn as_ty(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        if !db.is_explicitly_castable(self.ty, ty)? {
            todo!()
        }
        Ok(Self {
            qual: self.qual,
            ty,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EagerQualifier {
    Copyable,
    CopyableMut,
    Owned,
    OwnedMut,
    PureRef,
    GlobalRef,
    LocalRef,
    LocalRefMut,
    Transient,
}

impl std::fmt::Debug for EagerQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(match self {
            EagerQualifier::Copyable => "Copyable",
            EagerQualifier::CopyableMut => "CopyableMut",
            EagerQualifier::Owned => "Owned",
            EagerQualifier::OwnedMut => "OwnedMut",
            EagerQualifier::PureRef => "PureRef",
            EagerQualifier::GlobalRef => "GlobalRef",
            EagerQualifier::LocalRef => "LocalRef",
            EagerQualifier::LocalRefMut => "RefMut",
            EagerQualifier::Transient => "Transient",
        })
    }
}

impl EagerQualifier {
    pub fn mutable(&self) -> bool {
        match self {
            EagerQualifier::Copyable
            | EagerQualifier::PureRef
            | EagerQualifier::GlobalRef
            | EagerQualifier::LocalRef
            | EagerQualifier::Owned
            | EagerQualifier::Transient => false,
            EagerQualifier::CopyableMut
            | EagerQualifier::OwnedMut
            | EagerQualifier::LocalRefMut => true,
        }
    }

    pub fn binding(self, contract: EagerContract) -> Binding {
        match self {
            EagerQualifier::PureRef => match contract {
                EagerContract::Pure => Binding::Ref,
                EagerContract::Move => panic!(),
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseMemberForLetInit => Binding::Ref,
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::RefMut => todo!(),
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::GlobalRef => todo!(),
            },
            EagerQualifier::LocalRef => match contract {
                EagerContract::Pure => Binding::Ref,
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::RefMut => todo!(),
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::GlobalRef => todo!(),
            },
            EagerQualifier::Transient => todo!(),
            EagerQualifier::Copyable => Binding::Copy,
            EagerQualifier::CopyableMut => match contract {
                EagerContract::Pure => Binding::Copy,
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => Binding::Copy,
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseMemberForLetInit => panic!(),
                EagerContract::UseMemberForVarInit => panic!(),
                EagerContract::Return => Binding::Copy,
                EagerContract::RefMut => Binding::RefMut,
                EagerContract::MoveMut => Binding::Copy,
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::GlobalRef => todo!(),
            },
            EagerQualifier::Owned => match contract {
                EagerContract::Pure => Binding::Ref,
                EagerContract::Move => Binding::Move,
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => Binding::Move,
                EagerContract::RefMut => Binding::RefMut,
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssignRvalue => Binding::Move,
                EagerContract::GlobalRef => todo!(),
            },
            EagerQualifier::OwnedMut => match contract {
                EagerContract::Pure => Binding::Ref,
                EagerContract::Move => Binding::Move,
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseMemberForLetInit => Binding::Ref,
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => Binding::Move,
                EagerContract::RefMut => Binding::RefMut,
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::GlobalRef => todo!(),
            },
            EagerQualifier::GlobalRef => todo!(),
            EagerQualifier::LocalRefMut => todo!(),
        }
    }

    pub fn method_opt_output_binding(
        self,
        output_liason: OutputLiason,
        output_contract: EagerContract,
        is_output_ty_copyable: bool,
    ) -> Option<Binding> {
        match output_liason {
            OutputLiason::Transfer => None,
            OutputLiason::MemberAccess { member_liason } => {
                Some(self.member_binding(member_liason, output_contract, is_output_ty_copyable))
            }
        }
    }

    pub fn member_binding(
        self,
        member_liason: MemberLiason,
        member_contract: EagerContract,
        is_member_ty_copyable: bool,
    ) -> Binding {
        if is_member_ty_copyable {
            match member_contract {
                EagerContract::Pure
                | EagerContract::UseForLetInit
                | EagerContract::UseForAssignRvalue
                | EagerContract::UseMemberForLetInit
                | EagerContract::UseMemberForVarInit
                | EagerContract::Return => Binding::Copy,
                EagerContract::Move => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::RefMut => match member_liason {
                    MemberLiason::Immutable => todo!(),
                    MemberLiason::Mutable => Binding::RefMut,
                    MemberLiason::Derived => todo!(),
                },
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::GlobalRef => todo!(),
            }
        } else {
            // non-copyable
            match self {
                EagerQualifier::Copyable => todo!(),
                EagerQualifier::CopyableMut => todo!(),
                EagerQualifier::Owned => todo!(),
                EagerQualifier::OwnedMut => todo!(),
                EagerQualifier::PureRef => match member_contract {
                    EagerContract::Pure => Binding::Ref,
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => Binding::Ref,
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::UseMemberForLetInit => Binding::Ref,
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => todo!(),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::GlobalRef => todo!(),
                },
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRef => match member_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit | EagerContract::UseMemberForLetInit => {
                        Binding::Ref
                    }
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => todo!(),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::GlobalRef => todo!(),
                },
                EagerQualifier::LocalRefMut => match member_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => todo!(),
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => Binding::RefMut,
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::GlobalRef => todo!(),
                },
                EagerQualifier::Transient => todo!(),
            }
        }
    }

    pub fn from_parameter_use(
        input_liason: InputLiason,
        is_copyable: bool,
        contract: EagerContract,
    ) -> InferQueryResult<Self> {
        Self::from_parameter(input_liason, is_copyable).variable_use(contract)
    }

    pub fn from_parameter(input_liason: InputLiason, is_copyable: bool) -> Self {
        match input_liason {
            InputLiason::Pure => {
                if is_copyable {
                    EagerQualifier::Copyable
                } else {
                    EagerQualifier::PureRef
                }
            }
            InputLiason::GlobalRef => EagerQualifier::GlobalRef,
            InputLiason::Move => todo!(),
            InputLiason::LocalRefMut => todo!(),
            InputLiason::MoveMut => todo!(),
            InputLiason::MemberAccess => todo!(),
        }
    }

    pub fn from_field(
        this_qual: EagerQualifier,
        field_liason: MemberLiason,
        is_field_copyable: bool,
    ) -> InferResult<Self> {
        Ok(if is_field_copyable {
            if this_qual.mutable() && field_liason.mutable() {
                EagerQualifier::CopyableMut
            } else {
                EagerQualifier::Copyable
            }
        } else {
            // non-copyable
            match this_qual {
                EagerQualifier::Copyable | EagerQualifier::CopyableMut => panic!(),
                EagerQualifier::PureRef => EagerQualifier::PureRef,
                EagerQualifier::GlobalRef => EagerQualifier::GlobalRef,
                EagerQualifier::LocalRef => EagerQualifier::LocalRef,
                EagerQualifier::Transient => match field_liason {
                    MemberLiason::Immutable => todo!(),
                    MemberLiason::Mutable => todo!(),
                    MemberLiason::Derived => todo!(),
                },
                EagerQualifier::Owned | EagerQualifier::OwnedMut => panic!(),
                // match field_liason {
                //     FieldLiason::Mutable => EagerQualifier::LocalRefMut,
                //     FieldLiason::Immutable => EagerQualifier::LocalRef,
                //     FieldLiason::Derived => todo!(),
                // },
                EagerQualifier::LocalRefMut => match field_liason {
                    MemberLiason::Mutable => EagerQualifier::LocalRefMut,
                    MemberLiason::Immutable => panic!("shouldn't be here"),
                    MemberLiason::Derived => todo!(),
                },
            }
        })
    }

    pub fn from_output(output_liason: OutputLiason, is_copyable: bool) -> Self {
        match output_liason {
            OutputLiason::Transfer => Self::transitive(is_copyable),
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }

    pub fn variable_use(self, contract: EagerContract) -> InferQueryResult<Self> {
        Ok(match self {
            EagerQualifier::Copyable => match contract {
                EagerContract::Pure => EagerQualifier::Copyable,
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => EagerQualifier::Copyable,
                EagerContract::UseForVarInit => EagerQualifier::Copyable,
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => EagerQualifier::Copyable,
                EagerContract::RefMut => EagerQualifier::LocalRefMut,
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::GlobalRef => todo!(),
            },
            EagerQualifier::CopyableMut => match contract {
                EagerContract::Pure => EagerQualifier::Copyable,
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => EagerQualifier::Copyable,
                EagerContract::UseForVarInit => EagerQualifier::Copyable,
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => EagerQualifier::Copyable,
                EagerContract::RefMut => EagerQualifier::LocalRefMut,
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::GlobalRef => todo!(),
            },
            EagerQualifier::Owned => match contract {
                EagerContract::Pure => EagerQualifier::PureRef,
                EagerContract::Move => EagerQualifier::Transient,
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseForAssignRvalue => EagerQualifier::Transient,
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::RefMut => todo!(),
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::GlobalRef => todo!(),
            },
            EagerQualifier::OwnedMut => match contract {
                EagerContract::Pure => EagerQualifier::PureRef,
                EagerContract::Move => EagerQualifier::Transient,
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::UseMemberForLetInit => EagerQualifier::LocalRef,
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => EagerQualifier::Transient,
                EagerContract::RefMut => EagerQualifier::LocalRefMut,
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::GlobalRef => todo!(),
            },
            EagerQualifier::PureRef => match contract {
                EagerContract::Pure => EagerQualifier::PureRef,
                EagerContract::Move => {
                    return Err(query_error!(format!("can't move from a pure ref",)))
                }
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::UseMemberForLetInit => EagerQualifier::PureRef,
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::RefMut => todo!(),
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::GlobalRef => {
                    return Err(query_error!(format!(
                        "can't turn a pure ref to a global ref",
                    )))
                }
            },
            EagerQualifier::GlobalRef => match contract {
                EagerContract::Pure => EagerQualifier::PureRef,
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::RefMut => todo!(),
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::GlobalRef => todo!(),
            },
            EagerQualifier::LocalRef => match contract {
                EagerContract::Pure => EagerQualifier::PureRef,
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::RefMut => todo!(),
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::GlobalRef => todo!(),
            },
            EagerQualifier::LocalRefMut => todo!(),
            EagerQualifier::Transient => todo!(),
        })
    }

    pub fn transitive(is_copyable: bool) -> Self {
        if is_copyable {
            EagerQualifier::Copyable
        } else {
            EagerQualifier::Transient
        }
    }

    pub fn element_access_qual(
        this_qual: Self,
        this_contract: EagerContract,
        is_element_copyable: bool,
    ) -> Self {
        if is_element_copyable {
            match this_contract {
                EagerContract::Pure => EagerQualifier::Copyable,
                EagerContract::Move => panic!(),
                EagerContract::UseForLetInit => EagerQualifier::Copyable,
                EagerContract::UseForVarInit => EagerQualifier::CopyableMut,
                EagerContract::UseMemberForLetInit => EagerQualifier::Copyable,
                EagerContract::UseMemberForVarInit => EagerQualifier::CopyableMut,
                EagerContract::Return => EagerQualifier::Copyable,
                EagerContract::RefMut => EagerQualifier::CopyableMut,
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::MoveMut => panic!(),
                EagerContract::Exec => panic!(),
                EagerContract::GlobalRef => todo!(),
            }
        } else {
            match this_qual {
                EagerQualifier::Copyable | EagerQualifier::CopyableMut => panic!(),
                EagerQualifier::PureRef => match this_contract {
                    EagerContract::Pure => EagerQualifier::PureRef,
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => EagerQualifier::PureRef,
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseMemberForLetInit => EagerQualifier::PureRef,
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => todo!(),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::GlobalRef => todo!(),
                },
                EagerQualifier::LocalRef => match this_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => todo!(),
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseMemberForLetInit => EagerQualifier::LocalRef,
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => todo!(),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::GlobalRef => todo!(),
                },
                EagerQualifier::Transient => match this_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => todo!(),
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => todo!(),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::GlobalRef => todo!(),
                },
                EagerQualifier::Owned => match this_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => todo!(),
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => todo!(),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::GlobalRef => todo!(),
                },
                EagerQualifier::OwnedMut => panic!(),
                EagerQualifier::GlobalRef => todo!(),
                EagerQualifier::LocalRefMut => match this_contract {
                    EagerContract::Pure => panic!(),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => todo!(),
                    EagerContract::UseForVarInit => todo!(),
                    // let stmt doesn't move, but create a ref instead
                    EagerContract::UseMemberForLetInit => EagerQualifier::LocalRef,
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => EagerQualifier::LocalRefMut,
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::GlobalRef => todo!(),
                },
            }
        }
    }
}
