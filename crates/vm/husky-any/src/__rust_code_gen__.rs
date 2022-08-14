// this is generated by husky_vm_interface_code_gen::rust_code::write_rust_code
// do not modify by hand

use crate::*;

type void = ();
type b32 = u32;
type b64 = u64;

use husky_trace_protocol::VisualData;

// VirtualStruct
#[no_mangle]
pub unsafe extern "C" fn __virtual_struct_clone(data: *mut ()) -> *mut () {
    Box::<VirtualStruct>::into_raw(Box::new((*(data as *mut VirtualStruct)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_struct_drop(data: *mut ()) {
    Box::from_raw(data as *mut VirtualStruct);
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_struct_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const VirtualStruct) == *(other as *const () as *const VirtualStruct)
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_struct_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<VirtualStruct>(&__VIRTUAL_STRUCT_VTABLE) =
        registers[1].downcast_move(&__VIRTUAL_STRUCT_VTABLE)
}
#[no_mangle]
pub static __VIRTUAL_STRUCT_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __virtual_struct_clone,
    drop: __virtual_struct_drop,
    eq: __virtual_struct_eq,
    assign: __virtual_struct_assign,
    typename_str_hash_u64: 14990497878918864822,
    typename_str: "VirtualStruct",
};

// VirtualVec
#[no_mangle]
pub unsafe extern "C" fn __virtual_vec_clone(data: *mut ()) -> *mut () {
    Box::<VirtualVec>::into_raw(Box::new((*(data as *mut VirtualVec)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_vec_drop(data: *mut ()) {
    Box::from_raw(data as *mut VirtualVec);
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_vec_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const VirtualVec) == *(other as *const () as *const VirtualVec)
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_vec_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<VirtualVec>(&__VIRTUAL_VEC_VTABLE) =
        registers[1].downcast_move(&__VIRTUAL_VEC_VTABLE)
}
#[no_mangle]
pub static __VIRTUAL_VEC_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __virtual_vec_clone,
    drop: __virtual_vec_drop,
    eq: __virtual_vec_eq,
    assign: __virtual_vec_assign,
    typename_str_hash_u64: 11613109528630800846,
    typename_str: "VirtualVec",
};

// VirtualCyclicSlice
#[no_mangle]
pub unsafe extern "C" fn __virtual_cyclic_slice_clone(data: *mut ()) -> *mut () {
    Box::<VirtualCyclicSlice>::into_raw(Box::new((*(data as *mut VirtualCyclicSlice)).clone()))
        as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_cyclic_slice_drop(data: *mut ()) {
    Box::from_raw(data as *mut VirtualCyclicSlice);
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_cyclic_slice_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const VirtualCyclicSlice)
        == *(other as *const () as *const VirtualCyclicSlice)
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_cyclic_slice_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<VirtualCyclicSlice>(&__VIRTUAL_CYCLIC_SLICE_VTABLE) =
        registers[1].downcast_move(&__VIRTUAL_CYCLIC_SLICE_VTABLE)
}
#[no_mangle]
pub static __VIRTUAL_CYCLIC_SLICE_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __virtual_cyclic_slice_clone,
    drop: __virtual_cyclic_slice_drop,
    eq: __virtual_cyclic_slice_eq,
    assign: __virtual_cyclic_slice_assign,
    typename_str_hash_u64: 18190648919425218071,
    typename_str: "VirtualCyclicSlice",
};

// VisualData
#[no_mangle]
pub unsafe extern "C" fn __visual_data_clone(data: *mut ()) -> *mut () {
    Box::<VisualData>::into_raw(Box::new((*(data as *mut VisualData)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __visual_data_drop(data: *mut ()) {
    Box::from_raw(data as *mut VisualData);
}
#[no_mangle]
pub unsafe extern "C" fn __visual_data_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const VisualData) == *(other as *const () as *const VisualData)
}
#[no_mangle]
pub unsafe extern "C" fn __visual_data_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<VisualData>(&__VISUAL_DATA_VTABLE) =
        registers[1].downcast_move(&__VISUAL_DATA_VTABLE)
}
#[no_mangle]
pub static __VISUAL_DATA_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __visual_data_clone,
    drop: __visual_data_drop,
    eq: __visual_data_eq,
    assign: __visual_data_assign,
    typename_str_hash_u64: 271246442404434293,
    typename_str: "VisualData",
};
