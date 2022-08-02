mod cyclic_slice_;
mod firstx;
mod lastx;

use cyclic_slice::CyclicSlice;
pub use cyclic_slice_::*;
pub use firstx::*;
use husky_entity_route::EntityRoutePtr;
use husky_print_utils::msg_once;
use husky_visual_syntax::StaticVisualTy;
pub use lastx::*;

use super::*;
use husky_check_utils::should_eq;

pub trait __VecX<T> {
    fn ilen(&self) -> i32;

    fn __call__(__variadics: Vec<T>) -> Self;

    fn cyclic_slice<'a>(&'a self, start: i32, end: i32) -> CyclicSlice<'a, T>;

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

    fn __call__(__variadics: Vec<T>) -> Self {
        __variadics
    }

    fn cyclic_slice<'a>(&'a self, start: i32, end: i32) -> CyclicSlice<'a, T> {
        CyclicSlice::<T> {
            start,
            end,
            total: self.as_slice(),
        }
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
                        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
                            copy_fp: linkage_fp!(virtual_vec_index_copy),
                            eval_ref_fp: linkage_fp!(virtual_vec_index_eval_ref),
                            temp_ref_fp: linkage_fp!(virtual_vec_index_temp_ref),
                            temp_mut_fp: linkage_fp!(virtual_vec_index_temp_mut),
                            move_fp: linkage_fp!(virtual_vec_index_move),
                        })),
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
        visual_ty: StaticVisualTy::Group,
        opt_type_call: Some(&VEC_TYPE_CALL_DEFN),
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};

static VEC_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicTemplate::SingleTyped { ty: "E" },
        output_ty: "Vec<E>",
        output_liason: OutputLiason::Transfer,
        linkage: transfer_linkage!(virtual_vec_type_call, none),
    },
    dev_src: static_dev_src!(),
};

unsafe fn virtual_vec_type_call<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let mut data = vec![];
    for value in values {
        data.push(value.bind_move())
    }
    (__Register::new_box(VirtualVec::new(data), &__VIRTUAL_VEC_VTABLE))
}

unsafe fn virtual_vec_push<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let element = values[1].bind_move();
    let virtual_vec: &mut VirtualVec = values[0].downcast_temp_mut();
    virtual_vec.push(element);
    ().to_register()
}

unsafe fn virtual_vec_pop<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    let virtual_vec: &mut VirtualVec = values[0].downcast_temp_mut();
    virtual_vec.pop().unwrap()
}

fn virtual_vec_index_move<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    todo!()
}

unsafe fn virtual_vec_index_copy<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    assert_eq!(
        values[0].vtable as *const _,
        &__VIRTUAL_VEC_VTABLE as *const _
    );
    let this_value: &VirtualVec = values[0].downcast_temp_ref();
    let i: usize = values[1].downcast_i32() as usize;
    if i >= this_value.len() {
        todo!()
    }
    this_value[i].bind_copy()
}

unsafe fn virtual_vec_index_eval_ref<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    assert_eq!(
        values[0].vtable as *const _,
        &__VIRTUAL_VEC_VTABLE as *const _
    );
    let this_value: &'eval VirtualVec = values[0].downcast_eval_ref();
    let i: usize = values[1].downcast_i32() as usize;
    this_value[i].eval_bind_eval_ref()
}

unsafe fn virtual_vec_index_temp_ref<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    let this_value: &VirtualVec = values[0].downcast_temp_ref();
    let i: usize = values[1].downcast_i32() as usize;
    this_value[i].bind_temp_ref()
}

unsafe fn virtual_vec_index_temp_mut<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let i: usize = values[1].downcast_i32() as usize;
    let this_value: &mut VirtualVec = values[0].downcast_temp_mut();
    if i >= this_value.len() {
        todo!()
    }
    this_value[i].bind_temp_mut()
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
        opt_linkage: Some(transfer_linkage!(virtual_vec_len, none)),
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};

unsafe fn virtual_vec_len<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let virtual_vec: &VirtualVec = values[0].downcast_temp_ref();
    let len: i32 = virtual_vec.len().try_into().unwrap();
    len.to_register()
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
        opt_linkage: Some(transfer_linkage!(virtual_vec_push, none)),
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
        opt_linkage: Some(transfer_linkage!(virtual_vec_pop, none)),
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};
