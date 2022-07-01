mod cyclic_slice_;
mod firstx;
mod lastx;

use cyclic_slice::CyclicSlice;
pub use cyclic_slice_::*;
use entity_route::EntityRoutePtr;
pub use firstx::*;
pub use lastx::*;
use visual_syntax::{StaticVisualTy, StaticVisualizerVariant};

use super::*;
use check_utils::should_eq;

pub trait __VecX<T> {
    fn ilen(&self) -> i32;

    fn __call__() -> Self;

    fn cyclic_slice<'eval>(&self, start: i32, end: i32) -> CyclicSlice<'eval, T>;

    fn popx(&mut self) -> T;

    fn firstx(&self) -> &T;

    fn firstx_mut(&mut self) -> &mut T;

    fn lastx(&self) -> &T;

    fn lastx_mut(&mut self) -> &mut T;
}

impl<T> __VecX<T> for Vec<T> {
    fn ilen(&self) -> i32 {
        self.len() as i32
    }

    fn __call__() -> Self {
        Default::default()
    }

    fn cyclic_slice<'eval>(&self, start: i32, end: i32) -> CyclicSlice<'eval, T> {
        todo!()
    }

    fn popx(&mut self) -> T {
        self.pop().unwrap()
    }

    fn firstx(&self) -> &T {
        self.first().unwrap()
    }

    fn firstx_mut(&mut self) -> &mut T {
        self.first_mut().unwrap()
    }

    fn lastx(&self) -> &T {
        self.last().unwrap()
    }

    fn lastx_mut(&mut self) -> &mut T {
        self.last_mut().unwrap()
    }
}

pub static VEC_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "Vec",
        spatial_parameters: &[StaticSpatialParameter {
            name: "E",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
        }],
        static_trait_impls: &[StaticTraitImplDefn {
            trai: "std::ops::Index<i32>",
            member_impls: &[
                associated_type_impl!("Output", "E"),
                EntityStaticDefn {
                    dev_src: static_dev_src!(),
                    name: "index",
                    items: &[],
                    variant: EntityStaticDefnVariant::Method {
                        this_liason: ParameterLiason::MemberAccess,
                        parameters: &[],
                        output_ty: "E",
                        output_liason: OutputLiason::MemberAccess {
                            member_liason: MemberLiason::Mutable,
                        },
                        spatial_parameters: &[],
                        method_static_defn_kind: MethodStaticDefnKind::TraitMethodImpl,
                        opt_linkage: Some(Linkage::MemberAccess {
                            copy_access: routine_linkage!(generic_vec_element_copy_access, 2),
                            eval_ref_access: routine_linkage!(
                                generic_vec_element_eval_ref_access,
                                2
                            ),
                            temp_ref_access: routine_linkage!(
                                generic_vec_element_temp_ref_access,
                                2
                            ),
                            move_access: routine_linkage!(generic_vec_element_move_access, 2),
                            temp_mut_access: routine_linkage!(
                                generic_vec_element_borrow_mut_access,
                                2
                            ),
                        }),
                    },
                },
            ],
            dev_src: static_dev_src!(),
        }],
        ty_members: &[
            &VEC_LEN,
            &VEC_PUSH,
            &VEC_POPX,
            &VEC_FIRST,
            &VEC_LAST,
            &VEC_CYCLIC_SLICE,
        ],
        variants: &[],
        kind: TyKind::Vec,
        visualizer: &StaticVisualizer {
            ty: StaticVisualTy::Group,
            variant: StaticVisualizerVariant::Vec,
        },
        opt_type_call: Some(&VEC_TYPE_CALL_DEFN),
    },
    dev_src: dev_utils::static_dev_src!(),
};

static VEC_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        output_ty: "Vec<E>",
        output_liason: OutputLiason::Transfer,
        linkage: generic_routine_linkage!(generic_vec_type_call, 0).into(),
    },
    dev_src: static_dev_src!(),
};

pub(crate) fn generic_vec_type_call<'temp, 'eval>(
    ty: EntityRoutePtr,
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    Ok(TempValue::OwnedEval(OwnedValue::new(VirtualVec::new(ty))))
}

fn generic_vec_push<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let element = values[1].into_member();
    let generic_vec: &mut VirtualVec<'eval> = values[0].downcast_mut();
    generic_vec.push(element);
    Ok(TempValue::Copyable(().into()))
}

fn generic_vec_pop<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let generic_vec: &mut VirtualVec<'eval> = values[0].downcast_mut();
    Ok(generic_vec.pop().unwrap().into_stack())
}

pub(crate) fn generic_vec_element_move_access<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    todo!()
}

pub(crate) fn generic_vec_element_copy_access<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let this_value: &VirtualVec<'eval> = values[0].downcast_ref();
    let i: usize = match values[1] {
        TempValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    if i >= this_value.len() {
        todo!()
    }
    Ok(this_value[i].copy_into_stack())
}

pub(crate) fn generic_vec_element_eval_ref_access<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let this_value: &VirtualVec<'eval> = values[0].downcast_ref();
    let i: usize = match values[1] {
        TempValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    if i >= this_value.len() {
        return Err(vm_runtime_error!(format!(
            "index out of bounds: the len is {} but the index is {}",
            this_value.len(),
            i
        )));
    }
    let any_ptr: *const (dyn AnyValueDyn<'eval> + 'eval) = this_value[i].any_ref();
    Ok(match values[0] {
        TempValue::EvalRef(_) => TempValue::EvalRef(EvalRef(unsafe { &*any_ptr })),
        TempValue::TempRefEval(_) => TempValue::TempRefEval(unsafe { &*any_ptr }),
        _ => panic!(),
    })
}

pub(crate) fn generic_vec_element_temp_ref_access<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let this_value: &VirtualVec<'eval> = values[0].downcast_ref();
    let i: usize = match values[1] {
        TempValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    if i >= this_value.len() {
        return Err(vm_runtime_error!(format!(
            "index out of bounds: the len is {} but the index is {}",
            this_value.len(),
            i
        )));
    }
    Ok(this_value[i].bind_temp_ref())
}

pub(crate) fn generic_vec_element_borrow_mut_access<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let i: usize = match values[1] {
        TempValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    let (this_value, stack_idx, gen): (&mut VirtualVec<'eval>, _, _) =
        values[0].downcast_mut_full();
    if i >= this_value.len() {
        todo!()
    }
    Ok(this_value[i].bind_mut(stack_idx))
}

pub static VEC_LEN: EntityStaticDefn = EntityStaticDefn {
    name: "ilen",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(Linkage::SpecificTransfer(routine_linkage!(
            generic_vec_len,
            1
        ))),
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};

fn generic_vec_len<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let generic_vec: &VirtualVec<'eval> = values[0].downcast_ref();
    let len: i32 = generic_vec.len().try_into().unwrap();
    Ok(TempValue::Copyable(len.into()))
}

pub static VEC_PUSH: EntityStaticDefn = EntityStaticDefn {
    name: "push",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::TempRefMut,
        parameters: &[StaticParameter {
            liason: ParameterLiason::Move,
            ty: "E",
            name: "element",
        }],
        output_ty: "void",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(Linkage::SpecificTransfer(routine_linkage!(
            generic_vec_push,
            2
        ))),
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};

pub static VEC_POPX: EntityStaticDefn = EntityStaticDefn {
    name: "popx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::TempRefMut,
        parameters: &[],
        output_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(Linkage::SpecificTransfer(routine_linkage!(
            generic_vec_pop,
            1
        ))),
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};
